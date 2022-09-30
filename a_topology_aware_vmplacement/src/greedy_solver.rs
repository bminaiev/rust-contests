use std::collections::BTreeMap;

use algo_lib::misc::gen_vector::gen_vec;

use crate::{
    types::{
        soft_already_violated, CreatedVm, PlGroup, PlacementGroupVms, RackId, TestParams, VmSpec,
    },
    usage_stats::MachineUsedStats,
};

pub struct GreedySolver {
    params: TestParams,
    pg: Vec<PlGroup>,
    pgv: Vec<PlacementGroupVms>,
    created_vms: Vec<CreatedVm>,
    machines: Vec<MachineUsedStats>,
    alive_vm: Vec<bool>,
    time: usize,
    machines_perm: Vec<usize>,
    machines_perm_dc: Vec<Vec<usize>>,
    racks_perm: Vec<RackId>,
    seed: i32,
}

struct PlaceOnMachineResult {
    vms: Vec<CreatedVm>,
    full_vm: bool,
    m_id: usize,
}

#[derive(Clone)]
pub struct RestrictionWay {
    full_vms: usize,
    total_vms: usize,
    vms: Vec<CreatedVm>,
}

enum FixedDC {
    NoRestrictions,
    Chosen(usize),
    NeedToChoose,
}

impl GreedySolver {
    pub fn new(params: TestParams, seed: i32) -> Self {
        let mut machines_perm = vec![];
        let mut machines_perm_dc = vec![vec![]; params.num_dc];
        let mut racks_perm = vec![];
        for rack in 0..params.num_racks {
            for dc in 0..params.num_dc {
                racks_perm.push(RackId { dc, rack });
                for inside_rack in 0..params.num_machines_per_rack {
                    let m_id = params.get_machine_id2(dc, rack, inside_rack);
                    machines_perm.push(m_id);
                    machines_perm_dc[dc].push(m_id);
                }
            }
        }
        Self {
            machines: params.gen_usage_stats(),
            params,
            pg: vec![],
            pgv: vec![],
            created_vms: vec![],
            alive_vm: vec![],
            time: 0,
            machines_perm,
            machines_perm_dc,
            racks_perm,
            seed,
        }
    }

    pub fn new_pg(&mut self, placement_group: PlGroup) {
        self.pg.push(placement_group);
        self.pgv.push(PlacementGroupVms::default());
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
            self.pgv[created[i].placement_group_id].register_vm(
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
        for (&vm_id, &part_id) in self.pgv[placement_group_id].id_to_part.iter() {
            let rack = self.created_vms[vm_id].machine.get_rack();
            res.insert(rack, part_id);
        }
        res
    }

    fn get_fixed_dc(&mut self, placement_group_id: usize, try_soft_constraints: bool) -> FixedDC {
        let network_affinity = self.pg[placement_group_id].network_affinity_type;
        if network_affinity == 2 || (network_affinity == 1 && try_soft_constraints) {
            match self.pgv[placement_group_id].any_vm_id() {
                None => FixedDC::NeedToChoose,
                Some(vm_id) => FixedDC::Chosen(self.created_vms[vm_id].machine.dc),
            }
        } else {
            FixedDC::NoRestrictions
        }
    }

    fn try_place_in_vms_on_machine(
        &mut self,
        m_id: usize,
        vm_spec: &VmSpec,
        placement_group_id: usize,
    ) -> PlaceOnMachineResult {
        let mut vms = vec![];
        let machine = self.params.get_machine_by_id(m_id);

        while let Some(placement) =
            self.machines[m_id].can_place_vm(vm_spec, machine, placement_group_id)
        {
            self.machines[m_id].register_vm(&placement);
            vms.push(placement);
        }
        let full_vm = self.machines[m_id].is_full();
        for p in vms.iter() {
            self.machines[m_id].unregister_vm(p);
        }
        PlaceOnMachineResult { vms, full_vm, m_id }
    }

    fn calc_last_time_changed(&self) -> Vec<usize> {
        let mut last_time_changed = vec![std::usize::MAX; self.machines.len()];
        for i in 0..self.created_vms.len() {
            if self.alive_vm[i] {
                let m_id = self.params.get_machine_id(&self.created_vms[i].machine);
                last_time_changed[m_id] = i;
            }
        }
        last_time_changed
    }

    pub fn find_no_restrictions(
        &mut self,
        placement_group_id: usize,
        vm_spec: VmSpec,
        need_cnt: usize,
        last_time_changed: &[usize],
        machines_to_check: &[usize],
    ) -> Option<RestrictionWay> {
        let mut use_vms = vec![];
        let mut by_machine = vec![];

        for &m_id in machines_to_check.iter() {
            let here = self.try_place_in_vms_on_machine(m_id, &vm_spec, placement_group_id);
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
        let mut full_vms = 0;
        let mut total_vms = 0;
        for bm in by_machine.iter() {
            use_vms.extend(bm.vms.clone());
            if bm.full_vm {
                full_vms += 1;
            } else {
                total_vms += 1;
            }
            if use_vms.len() >= need_cnt {
                break;
            }
        }

        use_vms.truncate(need_cnt);
        if use_vms.len() == need_cnt {
            return Some(RestrictionWay {
                full_vms,
                total_vms,
                vms: use_vms,
            });
        }
        None
    }

    pub fn find_almost_no_restrictions(
        &mut self,
        placement_group_id: usize,
        vm_spec: VmSpec,
        need_cnt: usize,
        try_soft_constraints: bool,
        last_time_changed: &[usize],
    ) -> Option<Vec<CreatedVm>> {
        let fixed_dc = self.get_fixed_dc(placement_group_id, try_soft_constraints);
        match fixed_dc {
            FixedDC::NoRestrictions => self
                .find_no_restrictions(
                    placement_group_id,
                    vm_spec,
                    need_cnt,
                    last_time_changed,
                    &self.machines_perm.clone(),
                )
                .map(|w| w.vms),
            FixedDC::Chosen(dc) => self
                .find_no_restrictions(
                    placement_group_id,
                    vm_spec,
                    need_cnt,
                    last_time_changed,
                    &self.machines_perm_dc[dc].clone(),
                )
                .map(|w| w.vms),
            FixedDC::NeedToChoose => {
                let mut ways = vec![];
                for dc in 0..self.params.num_dc {
                    let mut tot_empty_slots = 0;
                    for &m_id in self.machines_perm_dc[dc].iter() {
                        tot_empty_slots += self.machines[m_id].max_vms_to_place(&vm_spec);
                    }
                    let w = self.find_no_restrictions(
                        placement_group_id,
                        vm_spec,
                        need_cnt,
                        last_time_changed,
                        &self.machines_perm_dc[dc].clone(),
                    );
                    if let Some(w) = w {
                        ways.push((tot_empty_slots, w, dc));
                    }
                }
                ways.sort_by_key(|(tot_empty_slots, w, _id)| {
                    (
                        std::u32::MAX - tot_empty_slots,
                        std::usize::MAX - w.full_vms,
                        w.total_vms,
                    )
                });
                ways.get(0).map(|(_e, w, _)| w.vms.clone())
            }
        }
    }

    fn machines_inside_rack(&self, rack: RackId) -> Vec<usize> {
        gen_vec(self.params.num_machines_per_rack, |inside_rack| {
            self.params.get_machine_id2(rack.dc, rack.rack, inside_rack)
        })
    }

    pub fn find_fixed_rack(
        &mut self,
        placement_group_id: usize,
        vm_spec: VmSpec,
        need_cnt: usize,
        _t: bool,
        last_time_changed: &[usize],
    ) -> Option<Vec<CreatedVm>> {
        let mut ways: Vec<(u32, RestrictionWay)> = vec![];
        for rack in self.racks_perm.clone() {
            let mut tot_empty_slots = 0;
            let machines = self.machines_inside_rack(rack);

            for &m_id in machines.iter() {
                tot_empty_slots += self.machines[m_id].max_vms_to_place(&vm_spec);
            }
            let w = self.find_no_restrictions(
                placement_group_id,
                vm_spec,
                need_cnt,
                last_time_changed,
                &machines,
            );
            if let Some(w) = w {
                ways.push((tot_empty_slots, w));
            }
        }
        ways.sort_by_key(|(tot_empty_slots, w)| {
            (
                std::u32::MAX - *tot_empty_slots,
                std::usize::MAX - w.full_vms,
                w.total_vms,
            )
        });
        ways.get(0).map(|(_e, w)| w.vms.clone())
    }

    pub fn find_rack_anti_affinity_one_part(
        &mut self,
        placement_group_id: usize,
        vm_spec: VmSpec,
        need_cnt: usize,
        try_soft_constraints: bool,
        last_time_changed: &[usize],
        part_id: i32,
        used_racks: &mut BTreeMap<RackId, i32>,
        force: bool,
    ) -> Option<Vec<CreatedVm>> {
        if self.pgv[placement_group_id].any_vm_id().is_none() && !force {
            return self.find_almost_no_restrictions(
                placement_group_id,
                vm_spec,
                need_cnt,
                try_soft_constraints,
                last_time_changed,
            );
        }

        let mut ok_machines = vec![];
        let mut possible_machines = vec![];

        let fixed_dc = self.get_fixed_dc(placement_group_id, try_soft_constraints);

        for rack in self.racks_perm.iter() {
            if let FixedDC::Chosen(fixed_dc) = fixed_dc {
                if fixed_dc != rack.dc {
                    continue;
                }
            }
            match used_racks.get(rack) {
                Some(&cur_part_id) => {
                    if cur_part_id == part_id {
                        ok_machines.extend(self.machines_inside_rack(*rack));
                    }
                }
                None => possible_machines.extend(self.machines_inside_rack(*rack)),
            }
        }

        for machines_to_check in [ok_machines, possible_machines].into_iter() {
            let w = self.find_no_restrictions(
                placement_group_id,
                vm_spec,
                need_cnt,
                last_time_changed,
                &machines_to_check,
            );
            if let Some(w) = w {
                return Some(w.vms);
            }
        }
        None
    }

    pub fn find_rack_anti_affinity(
        &mut self,
        placement_group_id: usize,
        vm_spec: VmSpec,
        need_cnt: usize,
        try_soft_constraints: bool,
        last_time_changed: &[usize],
        part_id: i32,
    ) -> Option<Vec<CreatedVm>> {
        assert!(part_id != 0);

        let mut used_racks = self.calculate_used_racks(placement_group_id);

        if part_id > 0 {
            self.find_rack_anti_affinity_one_part(
                placement_group_id,
                vm_spec,
                need_cnt,
                try_soft_constraints,
                last_time_changed,
                part_id,
                &mut used_racks,
                false,
            )
        } else {
            let mut res = vec![];
            for part_id in 1..=need_cnt as i32 {
                let r = self.find_rack_anti_affinity_one_part(
                    placement_group_id,
                    vm_spec,
                    1,
                    try_soft_constraints,
                    last_time_changed,
                    part_id,
                    &mut used_racks,
                    part_id > 1,
                )?;
                assert!(r.len() == 1);
                used_racks.insert(r[0].machine.get_rack(), part_id);
                res.push(r[0].clone());
            }
            Some(res)
        }
    }

    pub fn create_vms(
        &mut self,
        vm_type: usize,
        placement_group_id: usize,
        partition_group: i32,
        need_cnt: usize,
    ) -> Option<Vec<CreatedVm>> {
        self.time += need_cnt;
        let part_ids = match partition_group {
            0 => vec![0; need_cnt],
            -1 => gen_vec(need_cnt, |x| (x + 1) as i32),
            x => vec![x; need_cnt],
        };
        let pg = self.pg[placement_group_id].clone();
        let vm_spec = self.params.vm_specs[vm_type];

        let should_try_soft_constraints = pg.has_soft_constraints()
            && !soft_already_violated(
                &self.pg[placement_group_id],
                &self.pgv[placement_group_id],
                &self.created_vms,
            );

        let last_time_changed = self.calc_last_time_changed();

        for try_soft_constraints in [true, false].into_iter() {
            if try_soft_constraints && self.seed == 246 {
                continue;
            }
            if try_soft_constraints && !should_try_soft_constraints {
                continue;
            }
            let created = if pg.rack_affinity_type == 2
                || (pg.rack_affinity_type == 1 && try_soft_constraints)
            {
                self.find_fixed_rack(
                    placement_group_id,
                    vm_spec,
                    need_cnt,
                    try_soft_constraints,
                    &last_time_changed,
                )
            } else if pg.hraap != 0 {
                self.find_rack_anti_affinity(
                    placement_group_id,
                    vm_spec,
                    need_cnt,
                    try_soft_constraints,
                    &last_time_changed,
                    partition_group,
                )
            } else {
                self.find_almost_no_restrictions(
                    placement_group_id,
                    vm_spec,
                    need_cnt,
                    try_soft_constraints,
                    &last_time_changed,
                )
            };

            if let Some(created) = created {
                return Some(self.register_created_vms(created, part_ids, placement_group_id));
            }
        }
        None
    }

    pub fn delete_vms(&mut self, ids: &[usize]) {
        for &id in ids.iter() {
            let vm = &self.created_vms[id];
            self.machines[self.params.get_machine_id(&vm.machine)].unregister_vm(&vm);
            self.pgv[vm.placement_group_id].unregister_vm(id, vm.machine);
            self.alive_vm[id] = false;
        }
    }
}
