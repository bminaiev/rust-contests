#[allow(unused)]
use algo_lib::io::output::{output, set_global_output_to_file};
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::{dbg, out, out_line};

use crate::meta_solver::MetaSolver;
use crate::types::{Numa, PlGroup, TestParams, VmSpec};

mod additional_stats;
mod empty_solver;
mod fake_solver;
mod graph_solver;
mod greedy_solver;
mod machine_optimizer;
mod meta_solver;
mod random_solver;
mod solver;
mod state;
mod types;
mod usage_stats;

fn solve(input: &mut Input, _: usize, print_result: bool) {
    let num_dc = input.usize();
    let num_racks = input.usize();
    let num_machines_per_rack = input.usize();
    let num_numa_per_machine = input.usize();
    let numa = gen_vec(num_numa_per_machine, |_| Numa {
        cpu: input.read(),
        memory: input.read(),
    });

    let num_vm_types = input.usize();
    let vm_types = gen_vec(num_vm_types, |_| VmSpec {
        numa_cnt: input.read(),
        cpu: input.read(),
        memory: input.read(),
    });

    let read_pg = |input: &mut Input| -> PlGroup {
        let _ = input.usize();
        let hraap = input.usize();
        let soft_max_vms_per_machine = input.usize();
        let network_affinity_type = input.usize();
        let rack_affinity_type = input.usize();

        PlGroup {
            hraap,
            soft_max_vms_per_machine,
            network_affinity_type,
            rack_affinity_type,
        }
    };

    assert!(input.usize() == 1);
    let pg1 = read_pg(input);

    let params = TestParams::new(num_dc, num_racks, num_machines_per_rack, numa, vm_types);
    let mut solver = MetaSolver::new(params.clone(), &pg1);
    solver.new_pg(pg1);

    loop {
        let query_type = input.usize();
        match query_type {
            1 => {
                solver.new_pg(read_pg(input));
            }
            2 => {
                let num_vms = input.usize();

                let vm_type = input.usize() - 1;
                let placement_group_id = input.usize() - 1;
                let partition_group = input.i32();

                input.vec::<usize>(num_vms);
                if let Some(res) =
                    solver.create_vms(vm_type, placement_group_id, partition_group, num_vms)
                {
                    assert_eq!(res.len(), num_vms);
                    if print_result {
                        for vm in res.iter() {
                            let mut numa_ids = vec![];
                            for i in 0..4 {
                                if ((1 << i) & vm.numa_ids_mask) != 0 {
                                    numa_ids.push(i + 1);
                                }
                            }
                            out_line!(
                                vm.machine.dc + 1,
                                vm.machine.rack + 1,
                                vm.machine.inside_rack + 1,
                                numa_ids
                            );
                        }
                    }
                } else {
                    if print_result {
                        out_line!(-1);
                    }
                    break;
                }

                output().flush();
            }
            3 => {
                let num_vms = input.usize();
                let ids = gen_vec(num_vms, |_| input.usize() - 1);

                solver.delete_vms(&ids);
            }
            4 => {
                break;
            }
            _ => (),
        };
    }
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1, true);
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: true,
        input: TaskIoType::Std,
        output: TaskIoType::Std,
    };

    run_task(io, run)
}

//START MAIN

fn stress() {
    for test_id in 11..12 {
        dbg!(test_id);
        let mut input = Input::new_file(format!(
            "./a_topology_aware_vmplacement/local_test_kit/sample/{:02}",
            test_id
        ));
        solve(&mut input, test_id, false);
    }

    output().flush();
}

mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
