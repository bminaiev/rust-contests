use std::{cmp::max, collections::HashMap};

use algo_lib::misc::min_max::FindMinMaxPos;

use crate::{
    types::{CreatedVm, MachineId, PlGroup, RackId, TestParams, VmSpec},
    usage_stats::MachineUsedStats,
};

#[derive(Clone)]
struct PlacementGroupMapping {
    possible_machines: Vec<Vec<MachineId>>,
    racks_used: Vec<bool>,
    fixed_dc: Option<usize>,
    fixed_rack: Option<RackId>,
}

pub struct Solver {
    params: TestParams,
    pub placement_groups: Vec<PlGroup>,
    placement_group_mappings: Vec<PlacementGroupMapping>,
    machines: Vec<MachineId>,
    machines_stats: Vec<MachineUsedStats>,
    created_vms: Vec<CreatedVm>,
    created_vm_specs: Vec<VmSpec>,
    created_vm_pg: Vec<usize>,
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
    pub fn new(params: TestParams) -> Self {
        let machines_stats = params.gen_usage_stats();
        Self {
            machines: params.machine_ids.clone(),
            params,
            placement_groups: vec![],
            placement_group_mappings: vec![],
            created_vms: vec![],
            created_vm_specs: vec![],
            machines_stats,
            soft_machine_affinity: SoftMachineAffinity::new(),
            created_vm_pg: vec![],
        }
    }
    pub fn new_pg(&mut self, placement_group: PlGroup) {
        self.placement_groups.push(placement_group);
        let num_groups = max(1, placement_group.hraap);
        if placement_group.hraap != 0 {
            assert!(placement_group.rack_affinity_type == 0);
        }

        self.placement_group_mappings.push(PlacementGroupMapping {
            possible_machines: vec![vec![]; num_groups],
            racks_used: vec![false; self.params.num_dc * self.params.num_racks],
            fixed_dc: None,
            fixed_rack: None,
        });
    }

    fn get_rack_id(&self, dc: usize, rack: usize) -> usize {
        self.params.num_racks * dc + rack
    }

    fn get_available_rack(
        &self,
        dc: usize,
        rack: usize,
        fixed_ok: bool,
        spec: &VmSpec,
    ) -> AvailableRack {
        let mut max_possible_vms = 0;
        for inside_rack in 0..self.params.num_machines_per_rack {
            max_possible_vms += self.machines_stats
                [self.params.get_machine_id2(dc, rack, inside_rack)]
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

    fn can_place_vm(
        &self,
        machine_id: usize,
        vm: &VmSpec,
        placement_group_id: usize,
    ) -> Option<CreatedVm> {
        #[derive(Clone)]
        struct NumaWay {
            id: usize,
            cnt: u32,
        }

        let mut ways = vec![];
        for numa_id in 0..self.params.numa.len() {
            let cnt = self.machines_stats[machine_id].numa[numa_id].max_vms_to_place(&vm);
            if cnt > 0 {
                ways.push(NumaWay { id: numa_id, cnt });
            }
        }
        ways.sort_by_key(|w| w.cnt);
        ways.reverse();
        if ways.len() < vm.numa_cnt {
            return None;
        }

        if vm.numa_cnt == 1 {
            return Some(CreatedVm {
                machine: self.machines[machine_id],
                numa_ids_mask: 1 << ways[0].id,
                spec: vm.clone(),
                placement_group_id,
            });
        } else {
            assert_eq!(vm.numa_cnt, 2);
            return Some(CreatedVm {
                machine: self.machines[machine_id],
                numa_ids_mask: (1 << ways[0].id) | (1 << ways[1].id),
                spec: vm.clone(),
                placement_group_id,
            });
        }
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
        let m_id = self.params.get_machine_id(&vm.machine);
        self.machines_stats[m_id].register_vm(vm);
        self.soft_machine_affinity
            .register_vm(vm.machine, placement_group_id);
        self.created_vm_pg.push(placement_group_id);
    }

    fn unregister_vm(&mut self, vm: &CreatedVm, spec: &VmSpec, placement_group_id: usize) {
        let m_id = self.params.get_machine_id(&vm.machine);
        for numa_id in 0..4 {
            if ((1 << numa_id) & vm.numa_ids_mask) != 0 {
                self.machines_stats[m_id].numa[numa_id].unregister_vm(spec);
            }
        }
        self.soft_machine_affinity
            .unregister_vm(vm.machine, placement_group_id);
    }

    fn increase_group(
        &mut self,
        placement_group_id: usize,
        group_id: usize,
        spec: &VmSpec,
        need_vms: usize,
    ) -> bool {
        let pg = self.placement_groups[placement_group_id];
        let fixed_dc = self.placement_group_mappings[placement_group_id].fixed_dc;
        let fixed_rack = self.placement_group_mappings[placement_group_id].fixed_rack;
        if fixed_rack.is_some() && pg.rack_affinity_type == 2 {
            return false;
        }

        let mut available_racks = vec![];

        for dc in 0..self.params.num_dc {
            for rack in 0..self.params.num_racks {
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

        let full: Vec<_> = available_racks
            .iter()
            .filter(|ar| ar.fixed_ok && ar.max_possible_vms as usize >= need_vms)
            .cloned()
            .collect();
        if !full.is_empty() {
            let mut available_per_dc = vec![0; self.params.num_dc];
            for ar in full.iter() {
                available_per_dc[ar.dc] += ar.max_possible_vms;
            }
            if pg.network_affinity_type == 2 || pg.network_affinity_type == 1 {
                let use_dc = available_per_dc.index_of_max();
                let full_this_dc: Vec<_> =
                    full.iter().filter(|ar| ar.dc == use_dc).cloned().collect();
                assert!(!full_this_dc.is_empty());
                available_racks = full_this_dc;
                // }
            } else {
                available_racks = full;
            }
            available_racks
                .sort_by_key(|ar| (ar.max_possible_vms, u32::MAX - available_per_dc[ar.dc]));
        } else {
            available_racks.sort_by_key(|ar| (ar.fixed_ok, ar.max_possible_vms));
            available_racks.reverse();
        }

        if available_racks.is_empty() {
            return false;
        }
        let ar = available_racks[0].clone();

        {
            let rack_id = self.get_rack_id(ar.dc, ar.rack);
            self.placement_group_mappings[placement_group_id].racks_used[rack_id] = true;
        }
        for inside_rack in 0..self.params.num_machines_per_rack {
            let m_id = self.params.get_machine_id2(ar.dc, ar.rack, inside_rack);
            self.placement_group_mappings[placement_group_id].possible_machines[group_id]
                .push(self.machines[m_id].clone());
        }
        self.placement_group_mappings[placement_group_id].fixed_dc = Some(ar.dc);
        self.placement_group_mappings[placement_group_id].fixed_rack = Some(RackId {
            dc: ar.dc,
            rack: ar.rack,
        });
        true
    }

    pub fn create_vms(
        &mut self,
        vm_type: usize,
        placement_group_id: usize,
        partition_group: i32,
        need_cnt: usize,
    ) -> Option<Vec<CreatedVm>> {
        let mut res = vec![];

        let spec = self.params.vm_specs[vm_type];

        if partition_group == -1 {
            for i in 0..need_cnt {
                // TODO: remove clone
                loop {
                    // TODO: soft constraint on vms / machine
                    let machines: Vec<MachineId> =
                        self.placement_group_mappings[placement_group_id].possible_machines[i]
                            .clone();
                    for &need_soft in [true, false].iter() {
                        for m in machines.iter() {
                            if let Some(placement) = self.can_place_vm(
                                self.params.get_machine_id(m),
                                &spec,
                                placement_group_id,
                            ) {
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
                        if !self.increase_group(placement_group_id, i, &spec, 1) {
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
            for i in 0..need_cnt {
                loop {
                    for &need_soft in [true, false].iter() {
                        let machines = &self.placement_group_mappings[placement_group_id]
                            .possible_machines[group_id];
                        for m in machines.iter() {
                            if let Some(placement) = self.can_place_vm(
                                self.params.get_machine_id(m),
                                &spec,
                                placement_group_id,
                            ) {
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
                        if !self.increase_group(placement_group_id, group_id, &spec, need_cnt - i) {
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

        if res.len() != need_cnt {
            return None;
        }

        self.created_vm_specs.extend(vec![spec; res.len()]);
        self.created_vms.extend(res.clone());

        Some(res)
    }
}
