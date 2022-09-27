use std::collections::{BTreeMap, BTreeSet};

use algo_lib::misc::{gen_vector::gen_vec, rand::Random};

use crate::{
    types::{CreatedVm, PlacementGroup, RackId, TestParams, VmSpec},
    usage_stats::MachineUsedStats,
};

struct PlacementGroupVms {
    id_to_part: BTreeMap<usize, i32>,
}

impl PlacementGroupVms {
    pub fn new() -> Self {
        Self {
            id_to_part: BTreeMap::default(),
        }
    }

    pub fn unregister_vm(&mut self, id: usize) {
        self.id_to_part.remove(&id);
    }

    pub fn register_vm(&mut self, id: usize, part: i32) {
        self.id_to_part.insert(id, part);
    }

    pub fn any_vm_id(&self) -> Option<usize> {
        self.id_to_part.keys().next().map(|&x| x)
    }
}

pub struct RandomSolver {
    rnd: Random,
    params: TestParams,
    placement_groups: Vec<PlacementGroup>,
    placement_groups_vms: Vec<PlacementGroupVms>,
    created_vms: Vec<CreatedVm>,
    machines: Vec<MachineUsedStats>,
    fake_vms: Vec<CreatedVm>,
    last_fake_vms_random: bool,
}

impl RandomSolver {
    pub fn new(params: TestParams) -> Self {
        Self {
            machines: params.gen_usage_stats(),
            rnd: Random::new(787788),
            params,
            placement_groups: vec![],
            placement_groups_vms: vec![],
            created_vms: vec![],
            fake_vms: vec![],
            last_fake_vms_random: false,
        }
    }

    const AT_MOST_PER_MACHINE: usize = 5;

    fn recreate_fake_vms(&mut self, only_specicfic_type: Option<usize>) {
        while let Some(vm) = self.fake_vms.pop() {
            self.machines[self.params.get_machine_id(&vm.machine)].unregister_vm(&vm);
        }
        if let Some(vm_type) = only_specicfic_type {
            let vm_spec = self.params.vm_specs[vm_type];
            for m_id in 0..self.machines.len() {
                let mut at_most = Self::AT_MOST_PER_MACHINE;
                while let Some(placement) = self.machines[m_id].can_place_vm(
                    &vm_spec,
                    self.params.get_machine_by_id(m_id),
                    0,
                ) {
                    self.machines[m_id].register_vm(&placement);
                    self.fake_vms.push(placement);
                    at_most -= 1;
                    if at_most == 0 {
                        break;
                    }
                }
            }
            self.last_fake_vms_random = false;
            return;
        }
        if self.last_fake_vms_random {
            return;
        }
        let mut iter = vec![0; self.params.vm_specs.len()];
        let mut finished = 0;
        while finished != iter.len() {
            let vm_id = self.rnd.gen(0..iter.len());
            if iter[vm_id] == self.machines.len() {
                continue;
            }
            let vm_spec = self.params.vm_specs[vm_id];
            while iter[vm_id] < self.machines.len() {
                let m_id = iter[vm_id];
                if let Some(placement) = self.machines[m_id].can_place_vm(
                    &vm_spec,
                    self.params.get_machine_by_id(m_id),
                    0,
                ) {
                    self.machines[m_id].register_vm(&placement);
                    self.fake_vms.push(placement);
                    break;
                }
                iter[vm_id] += 1;
            }
            if iter[vm_id] == self.machines.len() {
                finished += 1;
            }
        }
        self.last_fake_vms_random = true;
    }

    pub fn new_placement_group(&mut self, idx: usize, placement_group: PlacementGroup) {
        self.placement_groups.push(placement_group);
        self.placement_groups_vms.push(PlacementGroupVms::new());
    }

    // const TRIES: usize = 50;

    fn potential_by_rack(&self, vm_spec: VmSpec) -> BTreeMap<RackId, Vec<usize>> {
        let mut by_rack: BTreeMap<_, Vec<usize>> = BTreeMap::new();
        for i in 0..self.fake_vms.len() {
            let vm = &self.fake_vms[i];
            if vm.spec == vm_spec {
                let machine = vm.machine;
                by_rack.entry(machine.get_rack()).or_default().push(i);
            }
        }
        by_rack
    }

    fn find_same_rack(&mut self, vm_spec: VmSpec, need_cnt: usize) -> Option<Vec<usize>> {
        let by_rack = self.potential_by_rack(vm_spec);
        let good_racks: Vec<_> = by_rack
            .iter()
            .filter(|(_k, v)| v.len() >= need_cnt)
            .collect();
        if good_racks.is_empty() {
            return None;
        } else {
            // TODO: smarter logic here
            let mut ids = good_racks[self.rnd.gen(0..good_racks.len())].1.clone();
            ids.truncate(need_cnt);
            return Some(ids);
        }
    }

    fn register_created_vms(
        &mut self,
        created_ids: Vec<usize>,
        part_ids: Vec<i32>,
        placement_group_id: usize,
    ) -> Option<Vec<CreatedVm>> {
        assert_eq!(created_ids.len(), part_ids.len());

        let mut created: Vec<_> = created_ids
            .iter()
            .map(|&i| self.fake_vms[i].clone())
            .collect();
        for i in 0..created.len() {
            created[i].placement_group_id = placement_group_id;
            self.placement_groups_vms[created[i].placement_group_id]
                .register_vm(self.created_vms.len() + i, part_ids[i]);
        }
        self.created_vms.extend(created.clone());
        {
            let mut used = vec![false; self.fake_vms.len()];
            for &x in created_ids.iter() {
                used[x] = true;
            }
            let mut new_sz = 0;
            for i in 0..used.len() {
                if !used[i] {
                    self.fake_vms.swap(i, new_sz);
                    new_sz += 1;
                }
            }
            self.fake_vms.truncate(new_sz);
        }
        Some(created)
    }

    fn calculate_used_racks(&self, placement_group_id: usize) -> BTreeMap<RackId, i32> {
        let mut res = BTreeMap::new();
        for (&vm_id, &part_id) in self.placement_groups_vms[placement_group_id]
            .id_to_part
            .iter()
        {
            let rack = self.created_vms[vm_id].machine.get_rack();
            res.insert(rack, part_id);
        }
        res
    }

    fn get_fixed_dc(&mut self, placement_group_id: usize) -> Option<usize> {
        let network_affinity = self.placement_groups[placement_group_id].network_affinity_type;
        if network_affinity == 2 {
            Some(
                match self.placement_groups_vms[placement_group_id].any_vm_id() {
                    // TODO: smarter logic
                    None => self.rnd.gen(0..self.params.num_dc),
                    Some(vm_id) => self.created_vms[vm_id].machine.dc,
                },
            )
        } else {
            None
        }
    }

    fn find_hard_rack_anti_affinity_specific_part(
        &mut self,
        partition_group: i32,
        need_cnt: usize,
        used_racks: &mut BTreeMap<RackId, i32>,
        by_rack: &BTreeMap<RackId, Vec<usize>>,
    ) -> Option<Vec<usize>> {
        assert!(partition_group != 0);

        let mut use_ids: Vec<usize> = vec![];
        for (&rack, &part_id) in used_racks.iter() {
            if part_id == partition_group {
                use_ids.extend(by_rack.get(&rack).unwrap_or(&vec![]));
                if use_ids.len() >= need_cnt {
                    break;
                }
            }
        }

        if use_ids.len() < need_cnt {
            for (k, v) in by_rack.iter() {
                if used_racks.contains_key(k) {
                    continue;
                }
                use_ids.extend(v.clone());
                if use_ids.len() >= need_cnt {
                    break;
                }
            }
        }
        use_ids.truncate(need_cnt);
        if use_ids.len() != need_cnt {
            return None;
        }
        for &id in use_ids.iter() {
            let vm = &self.fake_vms[id];
            used_racks.insert(vm.machine.get_rack(), partition_group);
        }
        return Some(use_ids);
    }

    // TODO this could be optimized a lot!
    fn find_hard_rack_anti_affinity(
        &mut self,
        vm_spec: VmSpec,
        placement_group_id: usize,
        partition_group: i32,
        need_cnt: usize,
    ) -> Option<Vec<usize>> {
        assert!(partition_group != 0);
        let mut used_racks = self.calculate_used_racks(placement_group_id);
        let mut by_rack = self.potential_by_rack(vm_spec);
        if let Some(dc) = self.get_fixed_dc(placement_group_id) {
            by_rack = by_rack.into_iter().filter(|(k, _v)| k.dc == dc).collect();
        }

        if partition_group == -1 {
            let mut created = vec![];
            for i in 0..need_cnt {
                let one = self.find_hard_rack_anti_affinity_specific_part(
                    (i + 1) as i32,
                    1,
                    &mut used_racks,
                    &mut by_rack,
                )?[0];
                created.push(one);
            }

            if cfg!(debug_assertions) {
                // TODO: assert different ids
                let different: BTreeSet<_> = created.iter().collect();
                assert!(different.len() == created.len());
            }
            assert_eq!(created.len(), need_cnt);
            Some(created)
        } else {
            self.find_hard_rack_anti_affinity_specific_part(
                partition_group,
                need_cnt,
                &mut used_racks,
                &mut by_rack,
            )
        }
    }

    pub fn find_almost_no_restrictions(
        &mut self,
        placement_group_id: usize,
        vm_spec: VmSpec,
        need_cnt: usize,
    ) -> Option<Vec<usize>> {
        // TODO: this is absolutely terrible
        let fixed_dc = self.get_fixed_dc(placement_group_id);

        let mut use_ids = vec![];
        for i in 0..self.fake_vms.len() {
            if self.fake_vms[i].spec == vm_spec {
                if let Some(dc) = fixed_dc {
                    if self.fake_vms[i].machine.dc != dc {
                        continue;
                    }
                }
                use_ids.push(i);
                if use_ids.len() == need_cnt {
                    break;
                }
            }
        }
        if use_ids.len() == need_cnt {
            return Some(use_ids);
        }
        None
    }

    pub fn create_vms(
        &mut self,
        vm_type: usize,
        placement_group_id: usize,
        partition_group: i32,
        indexes: &[usize],
    ) -> Option<Vec<CreatedVm>> {
        let part_ids = match partition_group {
            0 => vec![0; indexes.len()],
            -1 => gen_vec(indexes.len(), |x| (x + 1) as i32),
            x => vec![x; indexes.len()],
        };
        let pg = self.placement_groups[placement_group_id].clone();
        let vm_spec = self.params.vm_specs[vm_type];

        for only_this_type in [false, true].into_iter() {
            self.recreate_fake_vms(only_this_type.then_some(vm_type));
            let created = if pg.rack_affinity_type == 2 {
                self.find_same_rack(vm_spec, indexes.len())
            } else if pg.hard_rack_anti_affinity_partitions != 0 {
                self.find_hard_rack_anti_affinity(
                    vm_spec,
                    placement_group_id,
                    partition_group,
                    indexes.len(),
                )
            } else {
                self.find_almost_no_restrictions(placement_group_id, vm_spec, indexes.len())
            };

            if let Some(created) = created {
                return self.register_created_vms(created, part_ids, placement_group_id);
            }
        }
        None
    }

    pub fn delete_vms(&mut self, ids: &[usize]) {
        for &id in ids.iter() {
            let vm = &self.created_vms[id];
            self.machines[self.params.get_machine_id(&vm.machine)].unregister_vm(&vm);
            self.placement_groups_vms[vm.placement_group_id].unregister_vm(id);
        }
    }
}
