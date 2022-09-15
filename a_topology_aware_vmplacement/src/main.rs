//{"name":"A. Topology-Aware VM Placement","group":"Codeforces - ICPC 2022 Online Challenge powered by HUAWEI - Problem 2","url":"https://codeforces.com/contest/1724/problem/A","interactive":true,"timeLimit":15000,"tests":[{"input":"2 2 2 2\n8 16\n8 16\n3\n1 1 1\n1 4 2\n2 4 8\n1\n1 4 1\n1 0\n2\n3 1 1 1\n1 2 3\n2\n3 1 1 1\n4 5 6\n1\n2 0 0\n2 2\n2\n3 3 2 0\n7 8 9\n3\n3 3 5 6\n2\n2 3 1 4\n10 11\n2\n2 3 1 3\n12 13\n4\n","output":"1 1 1 1\n1 1 2 1\n1 2 1 1\n2 1 1 1\n2 1 2 1\n2 2 1 1\n1 2 1 1 2\n1 2 2 1 2\n1 2 2 1 2\n2 2 1 1 2\n2 2 2 1 2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATopologyAwareVMPlacement"}}}

use algo_lib::io::output::{output, set_global_output_to_file};
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

use crate::solver::Solver;
use crate::types::{Numa, VmSpec};

mod empty_solver;
mod solver;
mod types;

fn solve(input: &mut Input, test_case: usize, print_result: bool) -> Result {
    let num_dc = input.usize();
    let num_racks = input.usize();
    let num_machines_per_rack = input.usize();
    let num_numa_per_machine = input.usize();
    let numa = gen_vec(num_numa_per_machine, |_| Numa {
        cpu: input.read(),
        memory: input.read(),
    });

    dbg!(numa);

    let num_vm_types = input.usize();
    let vm_types = gen_vec(num_vm_types, |_| VmSpec {
        numa_cnt: input.read(),
        cpu: input.read(),
        memory: input.read(),
    });

    dbg!(num_vm_types);
    for spec in vm_types.iter() {
        dbg!(spec);
    }

    let mut solver = Solver::new(
        num_dc,
        num_racks,
        num_machines_per_rack,
        numa,
        vm_types.clone(),
    );
    let mut total_vms_created = 0;
    let mut total_queries = 0;

    loop {
        total_queries += 1;
        let query_type = input.usize();
        match query_type {
            1 => {
                // create placement group
                let idx = input.usize() - 1;
                let hard_rack_anti_affinity_partitions = input.usize();
                let soft_max_vms_per_machine = input.usize();
                // TODO: better type
                let network_affinity_type = input.usize();
                let rack_affinity_type = input.usize();

                solver.new_placement_group(
                    idx,
                    hard_rack_anti_affinity_partitions,
                    soft_max_vms_per_machine,
                    network_affinity_type,
                    rack_affinity_type,
                );
            }
            2 => {
                // create vm
                let num_vms = input.usize();

                let vm_type = input.usize() - 1;
                let placement_group_id = input.usize() - 1;
                // TODO: better type
                let partition_group = input.i32();
                let indexes = input.vec::<usize>(num_vms).sub_from_all(1);

                if let Some(res) =
                    solver.create_vms(vm_type, placement_group_id, partition_group, &indexes)
                {
                    if print_result {
                        for vm in res.iter() {
                            let numa_ids = vm.numa_ids.clone().add_to_all(1);
                            out_line!(
                                vm.machine.dc + 1,
                                vm.machine.rack + 1,
                                vm.machine.inside_rack + 1,
                                numa_ids
                            );
                        }
                    }
                } else {
                    dbg!("Can't create vms...", num_vms);
                    dbg!(solver.placement_groups[placement_group_id]);

                    if print_result {
                        out_line!(-1);
                    }
                    break;
                }

                total_vms_created += num_vms;

                // TODO: place VMs
                output().flush();
            }
            3 => {
                // vm deletion
                let num_vms = input.usize();
                let ids = input.vec::<usize>(num_vms).sub_from_all(1);
                solver.delete_vms(&ids);
            }
            4 => {
                // termination
                break;
            }
            _ => {
                unreachable!("Wrong op type: {}", query_type)
            }
        };
        // solver.step(total_queries);
    }
    dbg!(total_vms_created, total_queries);

    // solver.finish(test_case);

    Result {
        vms_created: total_vms_created,
        vms_without_soft: 0,
    }
}

#[derive(Debug)]
struct Result {
    vms_created: usize,
    vms_without_soft: usize,
}

fn read_baseline(test_id: usize) -> Result {
    let mut input = Input::new_file(format!(
        "./a_topology_aware_vmplacement/local_test_kit/sample/{:02}.a",
        test_id
    ));
    Result {
        vms_created: input.read(),
        vms_without_soft: input.read(),
    }
}

fn stress() {
    let mut stats = vec![];

    for test_id in 1..12 {
        dbg!(test_id);
        let mut input = Input::new_file(format!(
            "./a_topology_aware_vmplacement/local_test_kit/sample/{:02}",
            test_id
        ));
        let r = solve(&mut input, test_id, false);
        stats.push(r.vms_created);
        let baseline = read_baseline(test_id);
        // stats.push(baseline.vms_created);
        dbg!(baseline);
    }

    set_global_output_to_file("a_topology_aware_vmplacement/stats/current.txt");
    for &x in stats.iter() {
        out_line!(x);
    }
    output().flush();
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1, true);
    output().flush();
    input.skip_whitespace();
    input.peek().is_none()
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
mod tester;

fn main() {
    // tester::run_tests();
    // tester::run_single_test("1");
    tester::run_stress(stress);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
