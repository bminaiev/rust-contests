use std::{
    cmp::{max, min},
    collections::HashMap,
};

use algo_lib::{collections::index_of::IndexOf, dbg};
use algo_lib::{collections::shuffled::ShuffledTrait, io::output::output};
use algo_lib::{
    collections::{array_2d::Array2D, last_exn::LastExn},
    misc::{
        func::id,
        gen_vector::gen_vec,
        min_max::FindMinMaxPos,
        rand::Random,
        rec_function::{Callable, RecursiveFunction},
    },
};
use algo_lib::{out, out_line};

use crate::{
    graph_solver::find_shuffling,
    state::State,
    types::{CreatedVm, MachineId, PlGroup, RackId, TestParams, VmSpec},
    usage_stats::{MachineUsedStats, NumaUsedStats},
};

pub struct FakeSolver {
    params: TestParams,
    pub placement_groups: Vec<PlGroup>,
    created_vms: Vec<CreatedVm>,
    is_vm_alive: Vec<bool>,
    seeds_to_test: Vec<usize>,
    all_good_perms: Vec<Vec<usize>>,
}

impl FakeSolver {
    pub fn new(params: TestParams) -> Self {
        let mut all_good_perms = vec![];
        let should_go_before =
            Array2D::new_f(params.vm_specs.len(), params.vm_specs.len(), |i, j| {
                let s1 = params.vm_specs[i];
                let s2 = params.vm_specs[j];
                s2.cpu % s1.cpu == 0
                    && s2.memory % s1.memory == 0
                    && s2.numa_cnt % s1.numa_cnt == 0
                    && i != j
            });
        RecursiveFunction::new(|f, vm_types_perm: Vec<usize>| {
            if vm_types_perm.len() == params.vm_specs.len() {
                all_good_perms.push(vm_types_perm);
            } else {
                let candidates: Vec<_> = (0..params.vm_specs.len())
                    .filter(|&x| !vm_types_perm.contains(&x))
                    .filter(|&x| {
                        for i in 0..params.vm_specs.len() {
                            if should_go_before[i][x] && !vm_types_perm.contains(&i) {
                                return false;
                            }
                        }
                        true
                    })
                    .collect();
                assert!(candidates.len() > 0);
                for elem in candidates.into_iter() {
                    let mut next = vm_types_perm.clone();
                    next.push(elem);
                    f.call(next);
                }
            }
        })
        .call(vec![]);
        dbg!(all_good_perms.len());
        Self {
            params,
            placement_groups: vec![],
            created_vms: vec![],
            is_vm_alive: vec![],
            seeds_to_test: vec![],
            all_good_perms,
        }
    }
    pub fn new_placement_group(&mut self, idx: usize, placement_group: PlGroup) {
        assert!(self.placement_groups.len() == idx);
        self.placement_groups.push(placement_group);
    }

    pub fn delete_vms(&mut self, idxs: &[usize]) {
        for &id in idxs.iter() {
            self.is_vm_alive[id] = false;
        }
    }

    fn calc_num_vms_by_type(&self) -> Vec<usize> {
        let mut vms_by_type = vec![0; self.params.vm_specs.len()];
        for i in 0..self.is_vm_alive.len() {
            if self.is_vm_alive[i] {
                let type_id = self
                    .params
                    .vm_specs
                    .index_of(&self.created_vms[i].spec)
                    .unwrap();
                vms_by_type[type_id] += 1;
            }
        }
        vms_by_type
    }

    fn can_place_from_start2(&self) -> bool {
        let vms_by_type = self.calc_num_vms_by_type();
        find_shuffling(&self.params, &vms_by_type)
    }

    fn can_place_from_start(&self, seed: usize, save_png: bool) -> bool {
        let mut vm_types_perm = gen_vec(self.params.vm_specs.len(), id);
        vm_types_perm.sort_by_key(|&id| {
            let spec = &self.params.vm_specs[id];
            (spec.cpu + spec.memory, spec.numa_cnt)
        });
        if seed != 0 {
            vm_types_perm = self.all_good_perms[seed - 1].clone();
            // dbg!("Found good perm?");
        }
        let mut vms_by_type = self.calc_num_vms_by_type();

        let mut machines_stats = self.params.gen_usage_stats();
        let mut best_state = State::new(self.params.clone());
        for &id in vm_types_perm.iter().rev() {
            let spec = self.params.vm_specs[id];
            let mut more = vms_by_type[id];
            for m_id in 0..machines_stats.len() {
                let machine = self.params.get_machine_by_id(m_id);
                while more > 0 {
                    if let Some(placement) = machines_stats[m_id].can_place_vm(&spec, machine, 0) {
                        machines_stats[m_id].register_vm(&placement);
                        best_state.register_new_vms(&[placement]);
                        more -= 1;
                    } else {
                        break;
                    }
                }
            }
            if more > 0 {
                return false;
            }
        }

        if save_png {
            best_state.save_png("a_topology_aware_vmplacement/pics/last-state.png");
        }

        true
    }

    fn can_place_from_start3(&self, seed: usize, save_png: bool) -> bool {
        let vms_by_type = self.calc_num_vms_by_type();
        let mut all = vec![];
        for i in 0..vms_by_type.len() {
            all.extend(vec![i; vms_by_type[i]]);
        }
        let mut rnd = Random::new(seed as u64);
        rnd.shuffle(&mut all);

        let mut machines_stats = self.params.gen_usage_stats();
        let mut best_state = State::new(self.params.clone());
        let mut iters = vec![0; self.params.vm_specs.len()];
        for &id in all.iter() {
            let spec = self.params.vm_specs[id];
            while iters[id] < machines_stats.len() {
                let m_id = iters[id];
                let machine = self.params.get_machine_by_id(m_id);
                if let Some(placement) = machines_stats[m_id].can_place_vm(&spec, machine, 0) {
                    machines_stats[m_id].register_vm(&placement);
                    best_state.register_new_vms(&[placement]);
                    break;
                }
                iters[id] += 1;
            }
            if iters[id] == machines_stats.len() {
                return false;
            }
        }

        if save_png {
            best_state.save_png("a_topology_aware_vmplacement/pics/last-state.png");
        }

        dbg!(iters);
        true
    }

    pub fn create_vms(
        &mut self,
        vm_type: usize,
        placement_group_id: usize,
        partition_group: i32,
        indexes: &[usize],
    ) -> Option<Vec<CreatedVm>> {
        assert!(vm_type < self.params.vm_specs.len());
        assert!(indexes[0] == self.created_vms.len());

        let mut res = vec![];
        let spec = self.params.vm_specs[vm_type];

        for i in 0..indexes.len() {
            res.push(CreatedVm {
                machine: MachineId {
                    dc: 0,
                    rack: 0,
                    inside_rack: 0,
                },
                numa_ids_mask: 0,
                spec,
                placement_group_id: 0,
            })
        }

        assert_eq!(res.len(), indexes.len());
        self.created_vms.extend(res.clone());
        self.is_vm_alive.extend(vec![true; res.len()]);

        // self.can_place_from_start2();
        // if true {
        //     out_line!("Created vms:", self.created_vms.len());
        //     dbg!(self.created_vms.len());
        //     return Some(res);
        // }

        if !self.can_place_from_start3(0, false) {
            // if self.can_place_from_start2() {
            //     dbg!("Good!", self.created_vms.len());
            //     return Some(res);
            // }
            dbg!("FAiled 3");

            for &seed in self.seeds_to_test.iter().rev() {
                if self.can_place_from_start3(seed, false) {
                    return Some(res);
                }
            }
            dbg!("Failed...");
            for seed in self.seeds_to_test.last().unwrap_or(&0) + 1..self.all_good_perms.len() + 1 {
                if seed % 10 == 0 {
                    dbg!("Trying...", seed, self.created_vms.len());
                }
                if self.can_place_from_start3(seed, false) {
                    dbg!("Found!!!", seed);
                    self.seeds_to_test.push(seed);
                    return Some(res);
                }
            }

            let vms_by_type = self.calc_num_vms_by_type();
            dbg!(vms_by_type);

            find_shuffling(&self.params, &vms_by_type);

            let old_len = self.created_vms.len() - res.len();
            self.created_vms.truncate(old_len);
            self.is_vm_alive.truncate(old_len);
            assert!(self.can_place_from_start3(*self.seeds_to_test.last_exn(), true));

            return None;
        }
        Some(res)
    }
}
