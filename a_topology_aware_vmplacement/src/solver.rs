use std::cmp::{max, min};

use algo_lib::misc::{min_max::FindMinMaxPos, rand::Random};

use crate::types::{CreatedVm, MachineId, Numa, PlacementGroup, VmSpec};

#[derive(Clone)]
struct PlacementGroupMapping {
    possible_machines: Vec<Vec<MachineId>>,
}

#[derive(Clone, Copy, Debug)]
struct NumaUsedStats {
    free_cpu: u32,
    free_memory: u32,
}

impl NumaUsedStats {
    pub fn can_place(&self, vm: &VmSpec) -> bool {
        self.free_cpu >= vm.cpu && self.free_memory >= vm.memory
    }

    pub fn max_vms_to_place(&self, vm: &VmSpec) -> u32 {
        min(self.free_cpu / vm.cpu, self.free_memory / vm.memory)
    }

    pub fn register_vm(&mut self, vm: &VmSpec) {
        assert!(self.can_place(vm));
        self.free_cpu -= vm.cpu;
        self.free_memory -= vm.memory;
    }

    pub fn unregister_vm(&mut self, vm: &VmSpec) {
        self.free_cpu += vm.cpu;
        self.free_memory += vm.memory;
    }
}

#[derive(Clone, Debug)]
struct MachineUsedStats {
    numa: Vec<NumaUsedStats>,
}

impl MachineUsedStats {
    pub fn max_vms_to_place(&self, vm: &VmSpec) -> u32 {
        let mut res = 0;
        // TODO: if vm_spec requries two numa nodes?
        for numa in self.numa.iter() {
            res += numa.max_vms_to_place(vm);
        }
        res
    }
}

pub struct Solver {
    num_dc: usize,
    num_racks: usize,
    num_machines_per_rack: usize,
    numa: Vec<Numa>,
    vm_types: Vec<VmSpec>,
    pub placement_groups: Vec<PlacementGroup>,
    placement_group_mappings: Vec<PlacementGroupMapping>,
    machines: Vec<MachineId>,
    machines_stats: Vec<MachineUsedStats>,
    created_vms: Vec<CreatedVm>,
    created_vm_specs: Vec<VmSpec>,
    rnd: Random,
}

impl Solver {
    pub fn new(
        num_dc: usize,
        num_racks: usize,
        num_machines_per_rack: usize,
        numa: Vec<Numa>,
        vm_types: Vec<VmSpec>,
    ) -> Self {
        let mut machines = vec![];
        for dc in 0..num_dc {
            for rack in 0..num_racks {
                for inside_rack in 0..num_machines_per_rack {
                    machines.push(MachineId {
                        dc,
                        rack,
                        inside_rack,
                    });
                }
            }
        }
        let machines_stats = vec![
            MachineUsedStats {
                numa: numa
                    .iter()
                    .map(|numa| NumaUsedStats {
                        free_cpu: numa.cpu,
                        free_memory: numa.memory
                    })
                    .collect()
            };
            machines.len()
        ];
        Self {
            num_dc,
            num_racks,
            num_machines_per_rack,
            numa,
            vm_types,
            placement_groups: vec![],
            placement_group_mappings: vec![],
            machines,
            created_vms: vec![],
            created_vm_specs: vec![],
            machines_stats,
            rnd: Random::new(7877883),
        }
    }
    pub fn new_placement_group(
        &mut self,
        idx: usize,
        hard_rack_anti_affinity_partitions: usize,
        soft_max_vms_per_machine: usize,
        network_affinity_type: usize,
        rack_affinity_type: usize,
    ) {
        assert!(self.placement_groups.len() == idx);
        self.placement_groups.push(PlacementGroup {
            hard_rack_anti_affinity_partitions,
            soft_max_vms_per_machine,
            network_affinity_type,
            rack_affinity_type,
        });
        let num_groups = max(1, hard_rack_anti_affinity_partitions);
        if hard_rack_anti_affinity_partitions != 0 {
            assert!(rack_affinity_type == 0);
        }
        let mut possible_machines = vec![vec![]; num_groups];

        #[derive(Clone)]
        struct AvailableRack {
            dc: usize,
            rack: usize,
            max_possible_vms: u32,
        }

        let get_available_rack = |dc: usize, rack: usize| -> AvailableRack {
            let mut max_possible_vms = 0;
            // TODO: this is wrong.
            let spec = VmSpec {
                numa_cnt: 1,
                cpu: 1,
                memory: 1,
            };
            for inside_rack in 0..self.num_machines_per_rack {
                max_possible_vms += self.machines_stats[self.get_machine_id(dc, rack, inside_rack)]
                    .max_vms_to_place(&spec);
            }
            AvailableRack {
                dc,
                rack,
                max_possible_vms,
            }
        };

        let mut available_racks = vec![];
        for dc in 0..self.num_dc {
            for rack in 0..self.num_racks {
                available_racks.push(get_available_rack(dc, rack));
            }
        }
        available_racks.sort_by_key(|ar| ar.max_possible_vms);
        available_racks.reverse();

        if rack_affinity_type == 2 {
            // all should go to one rack
            assert!(possible_machines.len() == 1);
            available_racks.truncate(1);
        } else {
            if network_affinity_type == 2 {
                // all should go to one dc
                let mut by_dc = vec![0; self.num_dc];
                for ar in available_racks.iter() {
                    by_dc[ar.dc] += ar.max_possible_vms;
                }
                let best_dc = by_dc.index_of_max();
                available_racks = available_racks
                    .iter()
                    .filter(|ar| ar.dc == best_dc)
                    .cloned()
                    .collect();
            }
        }
        {
            let mut group_id = 0;
            for ar in available_racks.iter() {
                for inside_rack in 0..self.num_machines_per_rack {
                    possible_machines[group_id]
                        .push(self.machines[self.get_machine_id(ar.dc, ar.rack, inside_rack)]);
                }
                group_id = (group_id + 1) % possible_machines.len();
            }
        }
        self.placement_group_mappings
            .push(PlacementGroupMapping { possible_machines });
    }

    fn get_machine_id(&self, dc: usize, rack: usize, inside_rack: usize) -> usize {
        inside_rack
            + rack * self.num_machines_per_rack
            + dc * self.num_machines_per_rack * self.num_racks
    }

    fn get_machine_id2(&self, machine: &MachineId) -> usize {
        self.get_machine_id(machine.dc, machine.rack, machine.inside_rack)
    }

    pub fn delete_vms(&mut self, idxs: &[usize]) {
        for id in idxs.iter() {
            let vm = self.created_vms[*id].clone();
            self.unregister_vm(&vm, &self.created_vm_specs[*id].clone());
        }
    }

    fn can_place_vm(&self, machine_id: usize, vm: &VmSpec) -> Option<CreatedVm> {
        if vm.numa_cnt == 1 {
            for numa_id in 0..self.numa.len() {
                if self.machines_stats[machine_id].numa[numa_id].can_place(&vm) {
                    return Some(CreatedVm {
                        machine: self.machines[machine_id],
                        numa_ids: vec![numa_id],
                    });
                }
            }
        } else {
            assert_eq!(vm.numa_cnt, 2);
            for numa_id1 in 0..self.numa.len() {
                if self.machines_stats[machine_id].numa[numa_id1].can_place(&vm) {
                    for numa_id2 in numa_id1 + 1..self.numa.len() {
                        if self.machines_stats[machine_id].numa[numa_id2].can_place(&vm) {
                            return Some(CreatedVm {
                                machine: self.machines[machine_id],
                                numa_ids: vec![numa_id1, numa_id2],
                            });
                        }
                    }
                }
            }
        }
        None
    }

    fn register_vm(&mut self, vm: &CreatedVm, spec: &VmSpec) {
        let m_id = self.get_machine_id2(&vm.machine);
        for &numa_id in vm.numa_ids.iter() {
            self.machines_stats[m_id].numa[numa_id].register_vm(spec);
        }
    }

    fn unregister_vm(&mut self, vm: &CreatedVm, spec: &VmSpec) {
        let m_id = self.get_machine_id2(&vm.machine);
        for &numa_id in vm.numa_ids.iter() {
            self.machines_stats[m_id].numa[numa_id].unregister_vm(spec);
        }
    }

    pub fn create_vms(
        &mut self,
        vm_type: usize,
        placement_group_id: usize,
        partition_group: i32,
        indexes: &[usize],
    ) -> Option<Vec<CreatedVm>> {
        assert!(vm_type < self.vm_types.len());
        assert!(indexes[0] == self.created_vms.len());

        // TODO: make it faster...
        let mapping = self.placement_group_mappings[placement_group_id].clone();
        let mut res = vec![];

        let spec = self.vm_types[vm_type];

        if partition_group == -1 {
            assert_eq!(indexes.len(), mapping.possible_machines.len());
            for i in 0..indexes.len() {
                // TODO: remove clone
                let machines: Vec<MachineId> = mapping.possible_machines[i].clone();
                for m in machines.iter() {
                    if let Some(placement) = self.can_place_vm(self.get_machine_id2(m), &spec) {
                        self.register_vm(&placement, &spec);
                        res.push(placement);
                        break;
                    }
                }
                if res.len() != i + 1 {
                    return None;
                }
            }
        } else {
            let group_id = if partition_group == 0 {
                0
            } else {
                partition_group as usize - 1
            };
            for i in 0..indexes.len() {
                let machines = &mapping.possible_machines[group_id];
                for m in machines.iter() {
                    if let Some(placement) = self.can_place_vm(self.get_machine_id2(m), &spec) {
                        self.register_vm(&placement, &spec);
                        res.push(placement);
                        break;
                    }
                }
                if res.len() != i + 1 {
                    return None;
                }
            }
        }

        if res.len() != indexes.len() {
            return None;
        }
        // for vm in res.iter() {
        //     self.register_vm(vm, &spec);
        // }
        self.created_vm_specs.extend(vec![spec; res.len()]);
        self.created_vms.extend(res.clone());

        Some(res)
    }
}
