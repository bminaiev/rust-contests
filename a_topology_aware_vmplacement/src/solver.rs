use std::{
    cmp::{max, min},
    collections::HashMap,
};

use algo_lib::misc::{min_max::FindMinMaxPos, rand::Random};

use crate::types::{CreatedVm, MachineId, Numa, PlacementGroup, RackId, VmSpec};

#[derive(Clone)]
struct PlacementGroupMapping {
    possible_machines: Vec<Vec<MachineId>>,
    racks_used: Vec<bool>,
    fixed_dc: Option<usize>,
    fixed_rack: Option<RackId>,
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
    created_vm_pg: Vec<usize>,
    rnd: Random,
    soft_machine_affinity: SoftMachineAffinity,
}

#[derive(Clone, Debug)]
struct AvailableRack {
    dc: usize,
    rack: usize,
    max_possible_vms: u32,
    fixed_ok: bool,
}

#[derive(Clone)]
struct SoftMachineAffinity {
    cnt: HashMap<(MachineId, usize), usize>,
}

impl SoftMachineAffinity {
    pub fn new() -> Self {
        Self {
            cnt: Default::default(),
        }
    }

    pub fn register_vm(&mut self, machine: MachineId, placement_group_id: usize) {
        let key = (machine, placement_group_id);
        *self.cnt.entry(key).or_default() += 1;
    }

    pub fn unregister_vm(&mut self, machine: MachineId, placement_group_id: usize) {
        let key = (machine, placement_group_id);
        *self.cnt.entry(key).or_default() -= 1;
    }
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
            soft_machine_affinity: SoftMachineAffinity::new(),
            created_vm_pg: vec![],
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

        self.placement_group_mappings.push(PlacementGroupMapping {
            possible_machines: vec![vec![]; num_groups],
            racks_used: vec![false; self.num_dc * self.num_racks],
            fixed_dc: None,
            fixed_rack: None,
        });
    }

    fn get_machine_id(&self, dc: usize, rack: usize, inside_rack: usize) -> usize {
        inside_rack
            + rack * self.num_machines_per_rack
            + dc * self.num_machines_per_rack * self.num_racks
    }

    fn get_machine_id2(&self, machine: &MachineId) -> usize {
        self.get_machine_id(machine.dc, machine.rack, machine.inside_rack)
    }

    fn get_rack_id(&self, dc: usize, rack: usize) -> usize {
        self.num_racks * dc + rack
    }

    fn get_available_rack(
        &self,
        dc: usize,
        rack: usize,
        fixed_ok: bool,
        spec: &VmSpec,
    ) -> AvailableRack {
        let mut max_possible_vms = 0;
        for inside_rack in 0..self.num_machines_per_rack {
            max_possible_vms += self.machines_stats[self.get_machine_id(dc, rack, inside_rack)]
                .max_vms_to_place(&spec);
        }
        AvailableRack {
            dc,
            rack,
            max_possible_vms,
            fixed_ok,
        }
    }

    pub fn delete_vms(&mut self, idxs: &[usize]) {
        for id in idxs.iter() {
            let vm = self.created_vms[*id].clone();
            self.unregister_vm(
                &vm,
                &self.created_vm_specs[*id].clone(),
                self.created_vm_pg[*id],
            );
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

    fn is_safe(&self, machine_id: MachineId, placement_group_id: usize) -> bool {
        if self.placement_groups[placement_group_id].soft_max_vms_per_machine == 0 {
            return true;
        }
        self.soft_machine_affinity
            .cnt
            .get(&(machine_id, placement_group_id))
            .unwrap_or(&0)
            + 1
            <= self.placement_groups[placement_group_id].soft_max_vms_per_machine
    }

    fn register_vm(&mut self, vm: &CreatedVm, spec: &VmSpec, placement_group_id: usize) {
        let m_id = self.get_machine_id2(&vm.machine);
        for &numa_id in vm.numa_ids.iter() {
            self.machines_stats[m_id].numa[numa_id].register_vm(spec);
        }
        self.soft_machine_affinity
            .register_vm(vm.machine, placement_group_id);
        self.created_vm_pg.push(placement_group_id);
    }

    fn unregister_vm(&mut self, vm: &CreatedVm, spec: &VmSpec, placement_group_id: usize) {
        let m_id = self.get_machine_id2(&vm.machine);
        for &numa_id in vm.numa_ids.iter() {
            self.machines_stats[m_id].numa[numa_id].unregister_vm(spec);
        }
        self.soft_machine_affinity
            .unregister_vm(vm.machine, placement_group_id);
    }

    fn increase_group(
        &mut self,
        placement_group_id: usize,
        group_id: usize,
        spec: &VmSpec,
    ) -> bool {
        let pg = self.placement_groups[placement_group_id];
        let fixed_dc = self.placement_group_mappings[placement_group_id].fixed_dc;
        let fixed_rack = self.placement_group_mappings[placement_group_id].fixed_rack;
        if fixed_rack.is_some() && pg.rack_affinity_type == 2 {
            return false;
        }

        let mut available_racks = vec![];
        for dc in 0..self.num_dc {
            for rack in 0..self.num_racks {
                if self.placement_group_mappings[placement_group_id].racks_used
                    [self.get_rack_id(dc, rack)]
                {
                    continue;
                }
                let mut fixed_ok = true;
                if let Some(fixed_dc) = fixed_dc {
                    if fixed_dc != dc && pg.network_affinity_type == 2 {
                        continue;
                    }
                    if fixed_dc != dc && pg.network_affinity_type == 1 {
                        fixed_ok = false;
                    }
                }
                let ar = self.get_available_rack(dc, rack, fixed_ok, spec);
                if ar.max_possible_vms != 0 {
                    available_racks.push(ar);
                }
            }
        }
        available_racks.sort_by_key(|ar| (ar.fixed_ok, ar.max_possible_vms));
        available_racks.reverse();
        if available_racks.is_empty() {
            return false;
        }
        let ar = available_racks[0].clone();
        {
            let rack_id = self.get_rack_id(ar.dc, ar.rack);
            self.placement_group_mappings[placement_group_id].racks_used[rack_id] = true;
        }
        for inside_rack in 0..self.num_machines_per_rack {
            let m_id = self.get_machine_id(ar.dc, ar.rack, inside_rack);
            self.placement_group_mappings[placement_group_id].possible_machines[group_id]
                .push(self.machines[m_id].clone());
        }
        self.placement_group_mappings[placement_group_id].fixed_dc = Some(ar.dc);
        self.placement_group_mappings[placement_group_id].fixed_rack = Some(RackId {
            dc: ar.dc,
            rack: ar.rack,
        });
        return true;
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
        // let mapping = self.placement_group_mappings[placement_group_id].clone();
        let mut res = vec![];

        let spec = self.vm_types[vm_type];

        if partition_group == -1 {
            assert_eq!(
                indexes.len(),
                self.placement_group_mappings[placement_group_id]
                    .possible_machines
                    .len()
            );
            for i in 0..indexes.len() {
                // TODO: remove clone
                loop {
                    // TODO: soft constraint on vms / machine
                    let machines: Vec<MachineId> =
                        self.placement_group_mappings[placement_group_id].possible_machines[i]
                            .clone();
                    for &need_soft in [true, false].iter() {
                        for m in machines.iter() {
                            if let Some(placement) =
                                self.can_place_vm(self.get_machine_id2(m), &spec)
                            {
                                if need_soft && !self.is_safe(m.clone(), placement_group_id) {
                                    continue;
                                }
                                self.register_vm(&placement, &spec, placement_group_id);
                                res.push(placement);
                                break;
                            }
                        }
                        if res.len() == i + 1 {
                            break;
                        }
                    }
                    if res.len() != i + 1 {
                        if !self.increase_group(placement_group_id, i, &spec) {
                            return None;
                        } else {
                            // try again
                        }
                    } else {
                        break;
                    }
                }
            }
        } else {
            let group_id = if partition_group == 0 {
                assert!(
                    self.placement_group_mappings[placement_group_id]
                        .possible_machines
                        .len()
                        == 1
                );
                0
            } else {
                partition_group as usize - 1
            };
            for i in 0..indexes.len() {
                loop {
                    for &need_soft in [true, false].iter() {
                        let machines = &self.placement_group_mappings[placement_group_id]
                            .possible_machines[group_id];
                        for m in machines.iter() {
                            if let Some(placement) =
                                self.can_place_vm(self.get_machine_id2(m), &spec)
                            {
                                if need_soft && !self.is_safe(m.clone(), placement_group_id) {
                                    continue;
                                }
                                self.register_vm(&placement, &spec, placement_group_id);
                                res.push(placement);
                                break;
                            }
                        }
                        if res.len() == i + 1 {
                            break;
                        }
                    }
                    if res.len() != i + 1 {
                        if !self.increase_group(placement_group_id, group_id, &spec) {
                            // dbg!("Can't increase the group...", i, indexes.len());
                            // let machines = &self.placement_group_mappings[placement_group_id]
                            //     .possible_machines[group_id];
                            // for m in machines.iter() {
                            //     dbg!(&self.machines_stats[self.get_machine_id2(m)]);
                            // }

                            return None;
                        } else {
                            // try again..
                        }
                    } else {
                        break;
                    }
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
