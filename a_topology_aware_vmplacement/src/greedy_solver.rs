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

pub struct GreedySolver {
    rnd: Random,
    params: TestParams,
    placement_groups: Vec<PlacementGroup>,
    placement_groups_vms: Vec<PlacementGroupVms>,
    created_vms: Vec<CreatedVm>,
    machines: Vec<MachineUsedStats>,
    alive_vm: Vec<bool>,
    time: usize,
    machines_perm: Vec<usize>,
}

const DEBUG: bool = false;

struct PlaceOnMachineResult {
    vms: Vec<CreatedVm>,
    full_vm: bool,
    m_id: usize,
}

impl GreedySolver {
    pub fn new(params: TestParams) -> Self {
        // Self::create_pngs_dir();
        let mut machines_perm = vec![];
        for rack in 0..params.num_racks {
            for dc in 0..params.num_dc {
                for inside_rack in 0..params.num_machines_per_rack {
                    let m_id = params.get_machine_id2(dc, rack, inside_rack);
                    machines_perm.push(m_id);
                }
            }
        }
        Self {
            machines: params.gen_usage_stats(),
            rnd: Random::new(78778889),
            params,
            placement_groups: vec![],
            placement_groups_vms: vec![],
            created_vms: vec![],
            alive_vm: vec![],
            time: 0,
            machines_perm,
        }
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
        let mut fake_state = State::new(self.params.clone());
        for i in 0..self.created_vms.len() {
            if self.alive_vm[i] {
                fake_state.register_new_vms(&[self.created_vms[i].clone()]);
            }
        }

        fake_state.save_png(&format!("test_pics/{:03}.png", self.time));
    }

    pub fn new_placement_group(&mut self, idx: usize, placement_group: PlacementGroup) {
        self.placement_groups.push(placement_group);
        self.placement_groups_vms.push(PlacementGroupVms::new());
    }

    fn register_created_vms(
        &mut self,
        mut created: Vec<CreatedVm>,
        part_ids: Vec<i32>,
        placement_group_id: usize,
    ) -> Vec<CreatedVm> {
        assert_eq!(created.len(), part_ids.len());

        for i in 0..created.len() {
            created[i].placement_group_id = placement_group_id;
            self.placement_groups_vms[created[i].placement_group_id].register_vm(
                self.created_vms.len() + i,
                part_ids[i],
                created[i].machine,
            );
            let m_id = self.params.get_machine_id(&created[i].machine);
            self.machines[m_id].register_vm(&created[i]);
        }
        self.created_vms.extend(created.clone());
        self.alive_vm.extend(vec![true; created.len()]);
        created
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

    fn max_vms_here_soft_restrictions(
        &self,
        placement_group_id: usize,
        try_soft_constraints: bool,
        m_id: MachineId,
    ) -> usize {
        let already = *self.placement_groups_vms[placement_group_id]
            .cnt_by_machine
            .get(&m_id)
            .unwrap_or(&0);
        let limit = self.placement_groups[placement_group_id].soft_max_vms_per_machine;
        if limit == 0 || !try_soft_constraints {
            std::usize::MAX
        } else {
            limit - already
        }
    }

    fn try_place_in_vms_on_machine(
        &mut self,
        m_id: usize,
        vm_spec: &VmSpec,
        placement_group_id: usize,
        try_soft_constraints: bool,
    ) -> PlaceOnMachineResult {
        let mut vms = vec![];
        let machine = self.params.get_machine_by_id(m_id);

        let limit = self.max_vms_here_soft_restrictions(
            placement_group_id,
            try_soft_constraints,
            self.params.get_machine_by_id(m_id),
        );

        while let Some(placement) =
            self.machines[m_id].can_place_vm(vm_spec, machine, placement_group_id)
        {
            if vms.len() >= limit {
                break;
            }
            self.machines[m_id].register_vm(&placement);
            vms.push(placement);
        }
        let full_vm = self.machines[m_id].is_full();
        for p in vms.iter() {
            self.machines[m_id].unregister_vm(p);
        }
        PlaceOnMachineResult { vms, full_vm, m_id }
    }

    pub fn find_almost_no_restrictions(
        &mut self,
        placement_group_id: usize,
        vm_spec: VmSpec,
        need_cnt: usize,
        try_soft_constraints: bool,
    ) -> Option<Vec<CreatedVm>> {
        // TODO: this is absolutely terrible
        // TODO: try different dcs.
        let fixed_dc = None; //self.get_fixed_dc(placement_group_id, try_soft_constraints);

        let mut use_vms = vec![];

        let mut by_machine = vec![];

        let mut last_time_changed = vec![std::usize::MAX; self.machines.len()];
        for i in 0..self.created_vms.len() {
            if self.alive_vm[i] {
                let m_id = self.params.get_machine_id(&self.created_vms[i].machine);
                last_time_changed[m_id] = i;
            }
        }
        for m_id in self.machines_perm.clone() {
            if let Some(fixed_dc) = fixed_dc {
                if self.params.get_machine_by_id(m_id).dc != fixed_dc {
                    continue;
                }
            }
            let here = self.try_place_in_vms_on_machine(
                m_id,
                &vm_spec,
                placement_group_id,
                try_soft_constraints,
            );
            if !here.vms.is_empty() {
                by_machine.push(here);
            }
        }

        by_machine.sort_by_key(|bm| {
            (
                !bm.full_vm,
                if bm.full_vm {
                    bm.vms.len()
                } else {
                    last_time_changed[bm.m_id]
                },
            )
        });
        for bm in by_machine.iter() {
            use_vms.extend(bm.vms.clone());
            if use_vms.len() >= need_cnt {
                break;
            }
        }

        use_vms.truncate(need_cnt);
        if use_vms.len() == need_cnt {
            return Some(use_vms);
        }
        None
    }

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

        dbg!(vm_spec, pg, need_cnt);

        // TODO: we do not try soft constraints currently.
        for try_soft_constraints in [false].into_iter() {
            if try_soft_constraints && !should_try_soft_constraints {
                continue;
            }
            let created = if pg.rack_affinity_type == 2
                || (pg.rack_affinity_type == 1 && try_soft_constraints)
            {
                // dbg!(1);
                // return None;
                self.find_almost_no_restrictions(
                    placement_group_id,
                    vm_spec,
                    need_cnt,
                    try_soft_constraints,
                )
            } else if pg.hard_rack_anti_affinity_partitions != 0 {
                // dbg!(2);
                // return None;
                self.find_almost_no_restrictions(
                    placement_group_id,
                    vm_spec,
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
                return Some(res);
            }
        }
        None
    }

    pub fn delete_vms(&mut self, ids: &[usize]) {
        for &id in ids.iter() {
            let vm = &self.created_vms[id];
            self.machines[self.params.get_machine_id(&vm.machine)].unregister_vm(&vm);
            self.placement_groups_vms[vm.placement_group_id].unregister_vm(id, vm.machine);
            self.alive_vm[id] = false;
        }
    }
}
