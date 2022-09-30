use std::cmp::min;

use crate::types::{CreatedVm, MachineId, TestParams, VmSpec};

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct NumaUsedStats {
    pub free_cpu: u32,
    pub free_memory: u32,
}

impl NumaUsedStats {
    pub fn can_place(&self, vm: &VmSpec) -> bool {
        self.free_cpu >= vm.cpu && self.free_memory >= vm.memory
    }

    pub fn max_vms_to_place(&self, vm: &VmSpec) -> u32 {
        min(self.free_cpu / vm.cpu, self.free_memory / vm.memory)
    }

    pub fn register_vm(&mut self, vm: &VmSpec) {
        assert!(self.can_place(vm), "{:?}", vm);
        self.free_cpu -= vm.cpu;
        self.free_memory -= vm.memory;
    }

    pub fn unregister_vm(&mut self, vm: &VmSpec) {
        self.free_cpu += vm.cpu;
        self.free_memory += vm.memory;
    }
}

#[derive(Clone, Debug, Default)]
pub struct MachineUsedStats {
    pub numa: Vec<NumaUsedStats>,
}

impl MachineUsedStats {
    pub fn new(params: &TestParams) -> Self {
        Self {
            numa: params
                .numa
                .iter()
                .map(|numa| NumaUsedStats {
                    free_cpu: numa.cpu,
                    free_memory: numa.memory,
                })
                .collect(),
        }
    }

    pub fn max_vms_to_place(&self, vm: &VmSpec) -> u32 {
        if vm.numa_cnt == 2 {
            let per_numa: Vec<_> = self.numa.iter().map(|n| n.max_vms_to_place(vm)).collect();
            let sum: u32 = per_numa.iter().sum();
            let max_elem = per_numa.iter().max().unwrap();
            return min(sum / 2, sum - max_elem);
        }
        assert_eq!(vm.numa_cnt, 1);

        let mut res = 0;
        // TODO: if vm_spec requries two numa nodes?
        for numa in self.numa.iter() {
            res += numa.max_vms_to_place(vm);
        }

        res
    }

    pub fn can_place_vm(
        &self,
        vm: &VmSpec,
        machine: MachineId,
        placement_group_id: usize,
    ) -> Option<CreatedVm> {
        // TODO: this is not optimal...
        if vm.numa_cnt == 1 {
            for numa_id in 0..self.numa.len() {
                if self.numa[numa_id].can_place(vm) {
                    return Some(CreatedVm {
                        machine,
                        numa_ids_mask: 1 << numa_id,
                        spec: vm.clone(),
                        placement_group_id,
                    });
                }
            }
            return None;
        } else {
            for n1 in 0..self.numa.len() {
                if self.numa[n1].can_place(vm) {
                    for n2 in n1 + 1..self.numa.len() {
                        if self.numa[n2].can_place(vm) {
                            return Some(CreatedVm {
                                machine,
                                numa_ids_mask: (1 << n1) | (1 << n2),
                                spec: vm.clone(),
                                placement_group_id,
                            });
                        }
                    }
                }
            }
            return None;
        }
    }

    pub fn register_vm(&mut self, vm: &CreatedVm) {
        for numa_id in 0..4 {
            if ((1 << numa_id) & vm.numa_ids_mask) != 0 {
                self.numa[numa_id].register_vm(&vm.spec);
            }
        }
    }

    pub fn unregister_vm(&mut self, vm: &CreatedVm) {
        for numa_id in 0..4 {
            if ((1 << numa_id) & vm.numa_ids_mask) != 0 {
                self.numa[numa_id].unregister_vm(&vm.spec);
            }
        }
    }

    pub(crate) fn is_full(&self) -> bool {
        for n in self.numa.iter() {
            if n.free_cpu != 0 || n.free_memory != 0 {
                return false;
            }
        }
        true
    }
}
