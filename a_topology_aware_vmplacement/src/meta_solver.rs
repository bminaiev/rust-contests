use crate::{
    greedy_solver::GreedySolver,
    random_solver::RandomSolver,
    solver::Solver,
    types::{CreatedVm, PlacementGroup, TestParams, VmSpec},
};

pub enum MetaSolver {
    GreedySolver(GreedySolver),
    RandomSolver(RandomSolver),
    Solver(Solver),
}

fn is_same_ratio(numa: &[VmSpec]) -> bool {
    for i in 1..numa.len() {
        if numa[i].cpu * numa[0].memory != numa[0].cpu * numa[i].memory {
            return false;
        }
    }
    true
}

impl MetaSolver {
    pub fn new(params: TestParams) -> Self {
        if true {
            MetaSolver::Solver(Solver::new(params))
        } else {
            if is_same_ratio(&params.vm_specs) {
                MetaSolver::GreedySolver(GreedySolver::new(params))
            } else {
                MetaSolver::RandomSolver(RandomSolver::new(params))
            }
        }
    }

    pub fn new_placement_group(&mut self, placement_group: PlacementGroup) {
        match self {
            MetaSolver::GreedySolver(solver) => solver.new_placement_group(placement_group),
            MetaSolver::RandomSolver(solver) => solver.new_placement_group(placement_group),
            MetaSolver::Solver(solver) => solver.new_placement_group(placement_group),
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
