use std::{
    cmp::min,
    collections::{BTreeMap, BTreeSet},
    fs,
};

use algo_lib::{
    collections::{array_2d::Array2D, index_of::IndexOf, last_exn::LastExn},
    misc::{func::id, gen_vector::gen_vec, rand::Random},
};

use crate::{
    state::State,
    types::{CreatedVm, MachineId, PlacementGroup, RackId, TestParams, VmSpec},
    usage_stats::MachineUsedStats,
};

use algo_lib::dbg;

struct PlacementGroupVms {
    id_to_part: BTreeMap<usize, i32>,
    cnt_by_machine: BTreeMap<MachineId, usize>,
}

impl PlacementGroupVms {
    pub fn new() -> Self {
        Self {
            id_to_part: BTreeMap::default(),
            cnt_by_machine: BTreeMap::default(),
        }
    }

    pub fn unregister_vm(&mut self, id: usize, machine_id: MachineId) {
        self.id_to_part.remove(&id);
        *self.cnt_by_machine.entry(machine_id).or_default() -= 1;
    }

    pub fn register_vm(&mut self, id: usize, part: i32, machine_id: MachineId) {
        self.id_to_part.insert(id, part);
        *self.cnt_by_machine.entry(machine_id).or_default() += 1;
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
    png_iter: usize,
    alive_vm: Vec<bool>,
    time: usize,
}

const DEBUG: bool = false;

struct PotentialByRack {
    vm_ids: Vec<usize>,
    rack_starts: Vec<(RackId, usize)>,
    rack_id_idx: Array2D<usize>,
}

#[derive(Clone, Copy, Debug)]
enum VmTypeUsed {
    None,
    Some(usize),
    Many,
}

impl VmTypeUsed {
    pub fn add(&self, x: usize) -> Self {
        match self {
            VmTypeUsed::None => VmTypeUsed::Some(x),
            &VmTypeUsed::Some(y) => {
                if y == x {
                    self.clone()
                } else {
                    VmTypeUsed::Many
                }
            }
            VmTypeUsed::Many => VmTypeUsed::Many,
        }
    }
}

impl PotentialByRack {
    pub fn get_by_pos(&self, pos: usize) -> &[usize] {
        let start = self.rack_starts[pos].1;
        let end = self.rack_starts[pos + 1].1;
        &self.vm_ids[start..end]
    }

    pub fn new(mut vm_ids: Vec<usize>, solver: &RandomSolver) -> Self {
        vm_ids.sort_by_key(|&id| solver.fake_vms[id].machine.get_rack());
        let mut rack_starts: Vec<(RackId, usize)> = vec![];
        for i in 0..vm_ids.len() {
            if rack_starts.is_empty()
                || rack_starts.last_exn().0 != solver.fake_vms[vm_ids[i]].machine.get_rack()
            {
                rack_starts.push((solver.fake_vms[vm_ids[i]].machine.get_rack(), i));
            }
        }
        let mut rack_id_idx = Array2D::new(
            std::usize::MAX,
            solver.params.num_dc,
            solver.params.num_racks,
        );
        for i in 0..rack_starts.len() {
            rack_id_idx[rack_starts[i].0.dc][rack_starts[i].0.rack] = i;
        }
        rack_starts.push((RackId::FAKE, vm_ids.len()));
        Self {
            vm_ids,
            rack_starts,
            rack_id_idx,
        }
    }

    pub fn get_by_rack_id(&self, rack_id: &RackId) -> &[usize] {
        match self.rack_id_idx[rack_id.dc][rack_id.rack] {
            std::usize::MAX => &[],
            pos => self.get_by_pos(pos),
        }
    }
}

impl RandomSolver {
    pub fn new(params: TestParams) -> Self {
        Self::create_pngs_dir();
        Self {
            machines: params.gen_usage_stats(),
            rnd: Random::new(7877889),
            params,
            placement_groups: vec![],
            placement_groups_vms: vec![],
            created_vms: vec![],
            fake_vms: vec![],
            last_fake_vms_random: false,
            fake_cnt_by_rack: vec![],
            png_iter: 0,
            alive_vm: vec![],
            time: 0,
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

    // for each physical machine we try to use the same type of vm.
    fn add_fake_vms_specific(&mut self) {
        let mut used = vec![VmTypeUsed::None; self.params.total_machines()];
        for i in 0..self.created_vms.len() {
            if self.alive_vm[i] {
                let m_id = self.params.get_machine_id(&self.created_vms[i].machine);
                let vm_type = self
                    .params
                    .vm_specs
                    .index_of(&self.created_vms[i].spec)
                    .unwrap();
                used[m_id] = used[m_id].add(vm_type);
            }
        }
        let mut machines_to_use = vec![vec![]; self.params.vm_specs.len()];
        for i in 0..used.len() {
            if let VmTypeUsed::Some(x) = used[i] {
                machines_to_use[x].push(i);
            }
        }
        let mut cnt_by_type = vec![0; self.params.vm_specs.len()];
        let mut iters = vec![0; self.params.vm_specs.len()];
        for it in 0..2 {
            loop {
                let still_can_use: Vec<_> = (0..self.params.vm_specs.len())
                    .filter(|&id| iters[id] < used.len())
                    .collect();
                if still_can_use.is_empty() {
                    break;
                }
                let vm_id = still_can_use
                    .iter()
                    .map(|&idx| (cnt_by_type[idx], idx))
                    .min()
                    .unwrap()
                    .1;
                let m_id = machines_to_use[vm_id].pop().unwrap_or_else(|| {
                    let r = iters[vm_id];
                    iters[vm_id] += 1;
                    r
                });
                match used[m_id] {
                    VmTypeUsed::None => (),
                    VmTypeUsed::Some(y) => {
                        if y != vm_id && (y != 7 || vm_id != 4) {
                            continue;
                        }
                    }
                    VmTypeUsed::Many => continue,
                }

                let vm_spec = &self.params.vm_specs[vm_id];
                while let Some(placement) = self.machines[m_id].can_place_vm(
                    &vm_spec,
                    self.params.get_machine_by_id(m_id),
                    0,
                ) {
                    self.machines[m_id].register_vm(&placement);
                    self.fake_vms.push(placement);
                    cnt_by_type[vm_id] += 1;
                    used[m_id] = used[m_id].add(vm_id);
                }
            }
            if !DEBUG {
                break;
            }
            for x in used.iter_mut() {
                *x = VmTypeUsed::Some(0);
            }
            iters[0] = 0;
        }
        self.save_fake_vms_png();
        // dbg!(cnt_by_type);
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
        if !cfg!(debug_assertions) {
            return;
        }
        let mut cnt_by_rack = vec![
            Array2D::new(0, self.params.num_dc, self.params.num_racks);
            self.params.vm_specs.len()
        ];
        for vm in self.fake_vms.iter() {
            let type_id = self.params.vm_specs.index_of(&vm.spec).unwrap();
            cnt_by_rack[type_id][vm.machine.dc][vm.machine.rack] += 1;
        }
        // dbg!(cnt_by_rack[0][7]);
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
            dbg!("specific", self.time, vm_type, self.fake_vms.len());
            self.rnd.shuffle(&mut self.fake_vms);
            self.last_fake_vms_random = false;
            self.update_fake_stats();
            return;
        }
        if self.params.vm_specs.len() == 9 {
            // TODO: this actually should be a different check :)
            self.add_fake_vms_specific();
            self.last_fake_vms_random = true;
            self.rnd.shuffle(&mut self.fake_vms);
            self.update_fake_stats();
            return;
        }

        let added = self.randomly_add_fake_vms(&[]);
        dbg!(added);
        self.remove_all_fake_vms();
        {
            // TODO: maybe different for different tests?
            let top = self.gen_top_big(self.params.vm_specs.len());
            for &first_vm in top.iter() {
                let debug = first_vm == 4;
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
                    if debug {
                        // dbg!(w);
                    }
                    for inside_rack in 0..self.params.num_machines_per_rack {
                        let m_id = self.params.get_machine_id2(w.0.dc, w.0.rack, inside_rack);
                        while more > 0 {
                            if let Some(placement) = self.machines[m_id].can_place_vm(
                                &first_spec,
                                self.params.get_machine_by_id(m_id),
                                0,
                            ) {
                                // dbg!(&placement);
                                more -= 1;
                                self.machines[m_id].register_vm(&placement);
                                self.fake_vms.push(placement);
                            } else {
                                break;
                            }
                        }
                    }
                    if more == 0 {
                        break;
                    }
                }
            }
            self.randomly_add_fake_vms(&top);

            self.save_fake_vms_png();
            let mut really_added = vec![0; self.params.vm_specs.len()];
            for vm in self.fake_vms.iter() {
                really_added[self.params.vm_specs.index_of(&vm.spec).unwrap()] += 1;
            }
            dbg!(really_added);
            // TODO: save png.
        }
        self.last_fake_vms_random = true;
        self.rnd.shuffle(&mut self.fake_vms);
        self.update_fake_stats();
    }

    fn create_pngs_dir() {
        let path = "test_pics";
        fs::remove_dir_all(path).unwrap();
        fs::create_dir(path).unwrap();
    }

    fn save_fake_vms_png(&mut self) {
        if !DEBUG {
            return;
        }
        self.png_iter += 1;
        let mut fake_state = State::new(self.params.clone());
        for i in 0..self.created_vms.len() {
            if self.alive_vm[i] {
                // fake_state.register_new_vms(&[self.created_vms[i].clone()]);
            }
        }

        fake_state.register_new_vms(&self.fake_vms);
        fake_state.save_png(&format!("test_pics/{:03}.png", self.time));
    }

    pub fn new_placement_group(&mut self, idx: usize, placement_group: PlacementGroup) {
        // self.time += 1;
        // dbg!(placement_group.soft_max_vms_per_machine);
        self.placement_groups.push(placement_group);
        self.placement_groups_vms.push(PlacementGroupVms::new());
    }

    fn potential_by_rack(
        &self,
        vm_spec: VmSpec,
        fixed_dc: Option<usize>,
        fixed_rack: Option<RackId>,
    ) -> PotentialByRack {
        let mut vm_ids = vec![];
        for i in 0..self.fake_vms.len() {
            let vm = &self.fake_vms[i];
            if let Some(fixed_rack) = fixed_rack {
                if vm.machine.get_rack() != fixed_rack {
                    continue;
                }
            }
            if let Some(fixed_dc) = fixed_dc {
                if vm.machine.dc != fixed_dc {
                    continue;
                }
            }
            if vm.spec == vm_spec {
                vm_ids.push(i);
            }
        }
        PotentialByRack::new(vm_ids, &self)
    }

    fn find_same_rack(
        &mut self,
        vm_spec: VmSpec,
        need_cnt: usize,
        try_soft_constraints: bool,
        placement_group_id: usize,
    ) -> Option<Vec<usize>> {
        let fixed_rack = if try_soft_constraints {
            if let Some(any_vm) = self.placement_groups_vms[placement_group_id].any_vm_id() {
                Some(self.created_vms[any_vm].machine.get_rack())
            } else {
                None
            }
        } else {
            None
        };
        let by_rack = self.potential_by_rack(vm_spec, None, fixed_rack);
        let mut good_positions = vec![];
        for i in 0..by_rack.rack_starts.len() - 1 {
            if by_rack.get_by_pos(i).len() >= need_cnt {
                good_positions.push(i);
            }
        }
        if good_positions.is_empty() {
            return None;
        } else {
            // TODO: smarter logic here
            let pos = good_positions[self.rnd.gen(0..good_positions.len())];
            let vm_ids = by_rack.get_by_pos(pos)[..need_cnt].to_vec();
            return Some(vm_ids);
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
            self.placement_groups_vms[created[i].placement_group_id].register_vm(
                self.created_vms.len() + i,
                part_ids[i],
                created[i].machine,
            );
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
        self.alive_vm.extend(vec![true; created.len()]);
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
        by_rack: &PotentialByRack,
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
            use_ids.extend(by_rack.get_by_rack_id(&rack));
            if use_ids.len() >= need_cnt {
                break;
            }
        }

        if use_ids.len() < need_cnt {
            let mut all_racks: Vec<_> = by_rack.rack_starts[..by_rack.rack_starts.len() - 1]
                .iter()
                .map(|(x, _y)| x)
                .collect();
            self.rnd.shuffle(&mut all_racks);

            for &rack in all_racks.iter() {
                if used_racks.contains_key(rack) {
                    continue;
                }
                use_ids.extend(by_rack.get_by_rack_id(rack).clone());
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
        let fixed_dc = self.get_fixed_dc(placement_group_id, try_soft_constraints);
        let mut by_rack = self.potential_by_rack(vm_spec, fixed_dc, None);

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

        dbg!(vm_spec, need_cnt);
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
                    // TODO: remove back.
                    // break;
                }
            }
        }
        {
            #[derive(Clone, PartialEq, Eq, PartialOrd, Ord)]
            struct ByMachine {
                full_machine: bool,
                cnt: usize,
                machine: MachineId,
            }
            let mut by_machine: BTreeMap<MachineId, Vec<usize>> = BTreeMap::new();
            for &i in use_ids.iter() {
                let m = self.fake_vms[i].machine;
                by_machine.entry(m).or_default().push(i);
            }
            let mut all_machines = vec![];
            for (m, v) in by_machine.iter() {
                let already = *self.placement_groups_vms[placement_group_id]
                    .cnt_by_machine
                    .get(m)
                    .unwrap_or(&0);
                let limit = self.placement_groups[placement_group_id].soft_max_vms_per_machine;
                let more_ok = if limit == 0 || !try_soft_constraints {
                    std::usize::MAX
                } else {
                    limit - already
                };
                let sz = min(v.len(), more_ok);
                let m_id = self.params.get_machine_id(m);
                let cur_stats = self.machines[m_id].total_free_resources();
                let full_machine =
                    cur_stats.free_cpu == 0 && cur_stats.free_memory == 0 && v.len() == sz;
                all_machines.push(ByMachine {
                    full_machine,
                    cnt: sz as usize,
                    machine: m.clone(),
                });
            }
            all_machines.sort_by_key(|bm| (bm.full_machine, std::usize::MAX - bm.cnt));
            all_machines.reverse();
            let mut new_use_ids = vec![];
            // TODO: .rev()?
            let mut full = 0;
            let mut not_full = 0;
            for by_m in all_machines.iter() {
                let machine_in_ids = by_machine[&by_m.machine].clone();

                if by_m.full_machine {
                    full += 1;
                } else {
                    not_full += 1;
                }

                new_use_ids.extend(machine_in_ids[..by_m.cnt].to_vec());
                if new_use_ids.len() >= need_cnt {
                    break;
                }
            }
            dbg!(full, not_full);
            use_ids = new_use_ids;
        }
        {
            // let mut machines = BTreeSet::new();
            // for &i in use_ids.iter() {
            //     machines.insert(self.fake_vms[i].machine);
            // }
            // dbg!(machines.len(), use_ids.len(), vm_spec);
            // if vm_spec == self.params.vm_specs[4] {
            //     assert!(false);
            // }
        }
        // TODO: optimize here.
        use_ids.truncate(need_cnt);
        if use_ids.len() == need_cnt {
            // TODO: maybe delete?
            use_ids.reverse();
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
        self.time += indexes.len();
        let part_ids = match partition_group {
            0 => vec![0; indexes.len()],
            -1 => gen_vec(indexes.len(), |x| (x + 1) as i32),
            x => vec![x; indexes.len()],
        };
        let pg = self.placement_groups[placement_group_id].clone();
        let vm_spec = self.params.vm_specs[vm_type];

        let need_cnt = indexes.len();

        let should_try_soft_constraints = pg.has_soft_constraints()
            && !self.are_soft_constraints_already_violated(placement_group_id);

        for try_soft_constraints in [true, false].into_iter() {
            if try_soft_constraints && !should_try_soft_constraints {
                continue;
            }
            for only_this_type in [false, true].into_iter() {
                self.recreate_fake_vms(only_this_type.then_some(vm_type));
                let created = if pg.rack_affinity_type == 2
                    || (pg.rack_affinity_type == 1 && try_soft_constraints)
                {
                    self.find_same_rack(vm_spec, need_cnt, try_soft_constraints, placement_group_id)
                } else if pg.hard_rack_anti_affinity_partitions != 0 {
                    self.find_hard_rack_anti_affinity(
                        vm_spec,
                        placement_group_id,
                        partition_group,
                        need_cnt,
                        try_soft_constraints,
                    )
                } else {
                    self.find_almost_no_restrictions(
                        placement_group_id,
                        vm_spec,
                        need_cnt,
                        try_soft_constraints,
                    )
                };

                if let Some(created) = created {
                    let res = self.register_created_vms(created, part_ids, placement_group_id);
                    if try_soft_constraints && cfg!(debug_assertions) {
                        assert!(!self.are_soft_constraints_already_violated(placement_group_id));
                    }
                    return res;
                }
            }
        }
        None
    }

    pub fn delete_vms(&mut self, ids: &[usize]) {
        // self.time += 1;
        for &id in ids.iter() {
            let vm = &self.created_vms[id];
            self.machines[self.params.get_machine_id(&vm.machine)].unregister_vm(&vm);
            self.placement_groups_vms[vm.placement_group_id].unregister_vm(id, vm.machine);
            self.alive_vm[id] = false;
        }
        self.last_fake_vms_random = false;
    }
}
