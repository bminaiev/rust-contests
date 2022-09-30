use std::collections::BTreeMap;
use std::collections::BTreeSet;

use crate::algo_lib::collections::array_2d::Array2D;
use crate::algo_lib::collections::index_of::IndexOf;
use crate::algo_lib::misc::func::id;
use crate::algo_lib::misc::gen_vector::gen_vec;
use crate::algo_lib::misc::rand::Random;

use crate::types::CreatedVm;
use crate::types::PlacementGroup;
use crate::types::RackId;
use crate::types::TestParams;
use crate::types::VmSpec;
use crate::usage_stats::MachineUsedStats;

use crate::dbg;

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
    fake_cnt_by_rack: Vec<Array2D<usize>>,
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
            fake_cnt_by_rack: vec![],
        }
    }

    // TODO: try different const
    const AT_MOST_PER_MACHINE: usize = 5;

    fn gen_top_big(&self, cnt: usize) -> Vec<usize> {
        let mut top_big = gen_vec(self.params.vm_specs.len(), id);
        top_big.sort_by_key(|&id| {
            let s = &self.params.vm_specs[id];
            (s.cpu + s.memory) * (s.numa_cnt as u32)
        });
        top_big.reverse();
        // TODO: maybe different?
        top_big.truncate(cnt);
        top_big
    }

    fn randomly_add_fake_vms(&mut self, without: &[usize]) -> Vec<usize> {
        let mut iter = vec![0; self.params.vm_specs.len()];
        let mut finished = without.len();
        for &w in without.iter() {
            iter[w] = self.machines.len();
        }
        let mut added = vec![0; self.params.vm_specs.len()];

        let top_big = self.gen_top_big(3);

        while finished != iter.len() {
            let mut vm_id = self.rnd.gen(0..iter.len());
            if self.rnd.gen_double() < 0.5 {
                vm_id = top_big[self.rnd.gen(0..top_big.len())];
            }
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
                    added[vm_id] += 1;
                    break;
                }
                iter[vm_id] += 1;
            }
            if iter[vm_id] == self.machines.len() {
                finished += 1;
            }
        }
        added
    }

    fn remove_all_fake_vms(&mut self) {
        while let Some(vm) = self.fake_vms.pop() {
            self.machines[self.params.get_machine_id(&vm.machine)].unregister_vm(&vm);
        }
    }

    fn update_fake_stats(&mut self) {
        let mut cnt_by_rack = vec![
            Array2D::new(0, self.params.num_dc, self.params.num_racks);
            self.params.vm_specs.len()
        ];
        for vm in self.fake_vms.iter() {
            let type_id = self.params.vm_specs.index_of(&vm.spec).unwrap();
            cnt_by_rack[type_id][vm.machine.dc][vm.machine.rack] += 1;
        }
        // dbg!(cnt_by_rack[3][7]);
        self.fake_cnt_by_rack = cnt_by_rack;
    }

    fn recreate_fake_vms(&mut self, only_specicfic_type: Option<usize>) {
        if self.last_fake_vms_random && only_specicfic_type.is_none() {
            return;
        }
        self.remove_all_fake_vms();
        let perm = self.rnd.gen_permutation(self.machines.len());
        if let Some(vm_type) = only_specicfic_type {
            let vm_spec = self.params.vm_specs[vm_type];
            for &m_id in perm.iter() {
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
            self.rnd.shuffle(&mut self.fake_vms);
            self.last_fake_vms_random = false;
            self.update_fake_stats();
            return;
        }
        let added = self.randomly_add_fake_vms(&[]);
        self.remove_all_fake_vms();
        {
            // TODO: maybe different for different tests?
            let top = self.gen_top_big(1);
            for &first_vm in top.iter() {
                // let first_vm = self.params.vm_specs.len() - 1;
                let first_spec = self.params.vm_specs[first_vm];
                let mut by_rack: BTreeMap<RackId, u32> = BTreeMap::new();
                for m_id in 0..self.machines.len() {
                    let cnt = self.machines[m_id].max_vms_to_place(&first_spec);
                    if cnt != 0 {
                        let rack = self.params.get_machine_by_id(m_id).get_rack();
                        *by_rack.entry(rack).or_default() += cnt;
                    }
                }
                let mut ways: Vec<_> = by_rack.iter().collect();
                ways.sort_by_key(|w| (w.1, w.0.rack));
                ways.reverse();
                let mut more = added[first_vm];
                for w in ways.iter() {
                    for inside_rack in 0..self.params.num_machines_per_rack {
                        let m_id = self.params.get_machine_id2(w.0.dc, w.0.rack, inside_rack);
                        if more > 0 {
                            if let Some(placement) = self.machines[m_id].can_place_vm(
                                &first_spec,
                                self.params.get_machine_by_id(m_id),
                                0,
                            ) {
                                // dbg!(&placement);
                                more -= 1;
                                self.machines[m_id].register_vm(&placement);
                                self.fake_vms.push(placement);
                            }
                        }
                    }
                    if more == 0 {
                        break;
                    }
                }
            }
            self.randomly_add_fake_vms(&top);
        }
        self.last_fake_vms_random = true;
        self.rnd.shuffle(&mut self.fake_vms);
        self.update_fake_stats();
    }

    pub fn new_placement_group(&mut self, idx: usize, placement_group: PlacementGroup) {
        self.placement_groups.push(placement_group);
        self.placement_groups_vms.push(PlacementGroupVms::new());
    }

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

    fn find_same_rack(
        &mut self,
        vm_spec: VmSpec,
        need_cnt: usize,
        try_soft_constraints: bool,
        placement_group_id: usize,
    ) -> Option<Vec<usize>> {
        let by_rack = self.potential_by_rack(vm_spec);
        let mut good_racks: Vec<_> = by_rack
            .iter()
            .filter(|(_k, v)| v.len() >= need_cnt)
            .collect();
        if try_soft_constraints {
            let map = self.calculate_used_racks(placement_group_id);
            if map.len() > 1 {
                return None;
            }
            if map.len() == 1 {
                let use_rack = map.iter().next().unwrap().0.clone();
                good_racks = good_racks
                    .into_iter()
                    .filter(|(&k, v)| k == use_rack)
                    .collect();
            }
        }
        if good_racks.is_empty() {
            // dbg!("can't find good one", by_rack.len());
            // for (k, v) in by_rack.iter() {
            //     dbg!(k, v.len());
            // }
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

    fn get_fixed_dc(
        &mut self,
        placement_group_id: usize,
        try_soft_constraints: bool,
    ) -> Option<usize> {
        let network_affinity = self.placement_groups[placement_group_id].network_affinity_type;
        if network_affinity == 2 || (network_affinity == 1 && try_soft_constraints) {
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

        let mut my_racks = vec![];
        for (&rack, &part_id) in used_racks.iter() {
            if part_id == partition_group {
                my_racks.push(rack);
            }
        }
        self.rnd.shuffle(&mut my_racks);
        for rack in my_racks.iter() {
            use_ids.extend(by_rack.get(&rack).unwrap_or(&vec![]));
            if use_ids.len() >= need_cnt {
                break;
            }
        }

        if use_ids.len() < need_cnt {
            let mut all_racks: Vec<_> = by_rack.keys().collect();
            self.rnd.shuffle(&mut all_racks);

            for &rack in all_racks.iter() {
                if used_racks.contains_key(rack) {
                    continue;
                }
                use_ids.extend(by_rack[rack].clone());
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
        try_soft_constraints: bool,
    ) -> Option<Vec<usize>> {
        assert!(partition_group != 0);
        let mut used_racks = self.calculate_used_racks(placement_group_id);
        let mut by_rack = self.potential_by_rack(vm_spec);
        if let Some(dc) = self.get_fixed_dc(placement_group_id, try_soft_constraints) {
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
        try_soft_constraints: bool,
    ) -> Option<Vec<usize>> {
        // TODO: this is absolutely terrible
        // TODO: try different dcs.
        let fixed_dc = self.get_fixed_dc(placement_group_id, try_soft_constraints);

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

    // TODO: soft vms per machine

    fn are_soft_constraints_already_violated(&self, placement_group_id: usize) -> bool {
        let pg = self.placement_groups[placement_group_id];
        let info = &self.placement_groups_vms[placement_group_id];
        if let Some(any_vm) = info.any_vm_id() {
            for &vm_id in info.id_to_part.keys() {
                if pg.network_affinity_type == 1
                    && self.created_vms[vm_id].machine.dc != self.created_vms[any_vm].machine.dc
                {
                    return true;
                }
                if pg.rack_affinity_type == 1
                    && self.created_vms[vm_id].machine.get_rack()
                        != self.created_vms[any_vm].machine.get_rack()
                {
                    return true;
                }
            }
        }
        false
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

        // dbg!(self.placement_groups[placement_group_id], indexes.len());

        // if self.are_soft_constraints_already_violated(placement_group_id) {
        //     dbg!("already violated...", placement_group_id);
        //     assert!(false);
        // }

        let should_try_soft_constraints = pg.has_soft_constraints()
            && !self.are_soft_constraints_already_violated(placement_group_id);

        for try_soft_constraints in [true, false].into_iter() {
            for only_this_type in [false, true].into_iter() {
                if try_soft_constraints && !should_try_soft_constraints {
                    continue;
                }
                for iter in 0..2 {
                    self.recreate_fake_vms(only_this_type.then_some(vm_type));
                    let created = if pg.rack_affinity_type == 2
                        || (pg.rack_affinity_type == 1 && try_soft_constraints)
                    {
                        self.find_same_rack(
                            vm_spec,
                            indexes.len(),
                            try_soft_constraints,
                            placement_group_id,
                        )
                    } else if pg.hard_rack_anti_affinity_partitions != 0 {
                        self.find_hard_rack_anti_affinity(
                            vm_spec,
                            placement_group_id,
                            partition_group,
                            indexes.len(),
                            try_soft_constraints,
                        )
                    } else {
                        self.find_almost_no_restrictions(
                            placement_group_id,
                            vm_spec,
                            indexes.len(),
                            try_soft_constraints,
                        )
                    };

                    if let Some(created) = created {
                        let res = self.register_created_vms(created, part_ids, placement_group_id);
                        // dbg!(placement_group_id, res);
                        if try_soft_constraints {
                            assert!(!self.are_soft_constraints_already_violated(placement_group_id));
                        }
                        return res;
                    }

                    self.last_fake_vms_random = false;
                }
                // if should_try_soft_constraints {
                //     dbg!(
                //         only_this_type,
                //         vm_spec,
                //         indexes.len(),
                //         self.placement_groups[placement_group_id],
                //         placement_group_id
                //     );
                // }
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
        self.last_fake_vms_random = false;
    }
}
