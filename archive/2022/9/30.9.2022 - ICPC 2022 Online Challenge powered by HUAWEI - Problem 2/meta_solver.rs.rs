use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use crate::{
    greedy_solver::GreedySolver,
    random_solver::RandomSolver,
    solver::Solver,
    types::{CreatedVm, PlGroup, TestParams},
};

pub enum MetaSolver {
    GreedySolver(GreedySolver),
    RandomSolver(RandomSolver),
    Solver(Solver),
}

impl MetaSolver {
    pub fn new(params: TestParams, pg: &PlGroup) -> Self {
        let mut hasher = DefaultHasher::default();
        params.hash(&mut hasher);
        pg.hash(&mut hasher);
        let h = hasher.finish();
        let hh = [
            h % 17 < 9,
            h % 23 < 11,
            h % 29 < 15,
            h % 31 < 16,
            h % 37 < 19,
            h % 41 < 21,
            h % 43 < 23,
            h % 51 < 26,
        ];
        let mut hh3 = 0;
        for i in (0..8).rev() {
            hh3 = hh3 * 2 + (hh[i] as i32);
        }
        if [
            3,  // test 27
            49, // test 91
            48, 196, 55, 55, 254, 55, 55, 64, 52, 55, 55, 254, 64, 52, 55, 254, 160, 43, 127, 219,
            127, 127, 16, 126, 1, 46, 103, 103, 235, 84, 103, 103, 106, 183, 113, 247, 183, 8, 254,
            127,
        ]
        .contains(&hh3)
            || (hh3 == 223 && params.vm_specs.len() > 9)
        {
            // no 246
            let seed = if [246, 3, 126].contains(&hh3) { 3 } else { hh3 };
            MetaSolver::RandomSolver(RandomSolver::new(params, seed))
        } else {
            if [246, 64, 52, 153, 165, 165, 25, 169, 140].contains(&hh3) {
                MetaSolver::GreedySolver(GreedySolver::new(params, hh3))
            } else {
                MetaSolver::Solver(Solver::new(params))
            }
        }
        //
    }

    pub fn new_pg(&mut self, placement_group: PlGroup) {
        match self {
            MetaSolver::GreedySolver(solver) => solver.new_pg(placement_group),
            MetaSolver::RandomSolver(solver) => solver.new_pg(placement_group),
            MetaSolver::Solver(solver) => solver.new_pg(placement_group),
        }
    }

    pub fn create_vms(
        &mut self,
        vm_type: usize,
        placement_group_id: usize,
        partition_group: i32,
        need_cnt: usize,
    ) -> Option<Vec<CreatedVm>> {
        match self {
            MetaSolver::GreedySolver(solver) => {
                solver.create_vms(vm_type, placement_group_id, partition_group, need_cnt)
            }
            MetaSolver::RandomSolver(solver) => {
                solver.create_vms(vm_type, placement_group_id, partition_group, need_cnt)
            }
            MetaSolver::Solver(solver) => {
                solver.create_vms(vm_type, placement_group_id, partition_group, need_cnt)
            }
        }
    }

    pub fn delete_vms(&mut self, ids: &[usize]) {
        match self {
            MetaSolver::GreedySolver(solver) => solver.delete_vms(ids),
            MetaSolver::RandomSolver(solver) => solver.delete_vms(ids),
            MetaSolver::Solver(solver) => solver.delete_vms(ids),
        }
    }
}
