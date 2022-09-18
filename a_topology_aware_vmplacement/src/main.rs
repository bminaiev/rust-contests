//{"name":"A. Topology-Aware VM Placement","group":"Codeforces - ICPC 2022 Online Challenge powered by HUAWEI - Problem 2","url":"https://codeforces.com/contest/1724/problem/A","interactive":true,"timeLimit":15000,"tests":[{"input":"2 2 2 2\n8 16\n8 16\n3\n1 1 1\n1 4 2\n2 4 8\n1\n1 4 1\n1 0\n2\n3 1 1 1\n1 2 3\n2\n3 1 1 1\n4 5 6\n1\n2 0 0\n2 2\n2\n3 3 2 0\n7 8 9\n3\n3 3 5 6\n2\n2 3 1 4\n10 11\n2\n2 3 1 3\n12 13\n4\n","output":"1 1 1 1\n1 1 2 1\n1 2 1 1\n2 1 1 1\n2 1 2 1\n2 2 1 1\n1 2 1 1 2\n1 2 2 1 2\n1 2 2 1 2\n2 2 1 1 2\n2 2 2 1 2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATopologyAwareVMPlacement"}}}

use std::collections::{BTreeMap, HashMap};
use std::fmt::format;
use std::time::Instant;

use algo_lib::collections::index_of::IndexOf;
use algo_lib::io::output::{output, set_global_output_to_file};
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::gcd;
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

use crate::fake_solver::FakeSolver;
use crate::state::State;
use crate::types::{Numa, PlacementGroup, TestParams, VmSpec};

mod empty_solver;
mod fake_solver;
mod solver;
mod state;
mod types;
mod usage_stats;

#[derive(Clone)]
struct PG_stat {
    created: Vec<usize>,
    deleted: Vec<usize>,
    change_times: Vec<usize>,
    partitions: usize,
    rack_affinity: usize,
}

impl PG_stat {
    pub fn new(n: usize, partitions: usize, rack_affinity: usize) -> Self {
        Self {
            created: vec![0; n],
            deleted: vec![0; n],
            change_times: vec![],
            partitions,
            rack_affinity,
        }
    }
}

#[derive(Debug)]
struct ExpectedNextOperation {
    total_queries: usize,
    pg: usize,
    group_id: i32,
}

fn is_same_ratio(numa: &[VmSpec]) -> bool {
    let g = gcd(numa[0].cpu, numa[0].memory);
    for i in 1..numa.len() {
        let g2 = gcd(numa[i].cpu, numa[i].memory);
        if numa[i].cpu / g2 != numa[0].cpu / g || numa[i].memory / g2 != numa[0].memory / g {
            return false;
        }
    }
    true
}

fn solve(input: &mut Input, test_case: usize, print_result: bool) -> Result {
    let start = Instant::now();

    let num_dc = input.usize();
    let num_racks = input.usize();
    let num_machines_per_rack = input.usize();
    let num_numa_per_machine = input.usize();
    let numa = gen_vec(num_numa_per_machine, |_| Numa {
        cpu: input.read(),
        memory: input.read(),
    });
    // assert!(numa[0] == numa[1]);
    // if numa.len() == 4 {
    //     assert!(numa[2] == numa[3]);
    // }

    let known_numa_nodes = vec![
        Numa { cpu: 8, memory: 16 },
        Numa {
            cpu: 64,
            memory: 128,
        },
        Numa {
            cpu: 146,
            memory: 112,
        },
        Numa {
            cpu: 96,
            memory: 74,
        },
    ];
    // for n in numa.iter() {
    //     assert!(known_numa_nodes.index_of(n).is_some());
    // }

    let num_vm_types = input.usize();
    let vm_types = gen_vec(num_vm_types, |_| VmSpec {
        numa_cnt: input.read(),
        cpu: input.read(),
        memory: input.read(),
    });

    let same_cpu_memory_ratio = is_same_ratio(&vm_types);
    dbg!(same_cpu_memory_ratio);

    let params = TestParams {
        num_dc,
        num_racks,
        num_machines_per_rack,
        numa,
        vm_specs: vm_types,
    };

    // assert!(
    //     params.vm_specs.len() == 9 || params.vm_specs.len() == 3 || params.vm_specs.len() == 18
    // );
    dbg!(params);

    let mut solver = FakeSolver::new(params.clone());
    let mut state = State::new(params.clone());

    let mut total_vms_created = 0;
    let mut total_queries = 0;

    let mut log = vec![];

    let mut last_pg_used = HashMap::new();
    let mut pg_stats = BTreeMap::new();

    let mut expected_next_op: Option<ExpectedNextOperation> = None;

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

                let placement_group = PlacementGroup {
                    hard_rack_anti_affinity_partitions,
                    soft_max_vms_per_machine,
                    network_affinity_type,
                    rack_affinity_type,
                };
                pg_stats.insert(
                    idx,
                    PG_stat::new(
                        params.vm_specs.len(),
                        hard_rack_anti_affinity_partitions,
                        rack_affinity_type,
                    ),
                );

                solver.new_placement_group(idx, placement_group);
                state.new_placement_group(placement_group);
            }
            2 => {
                // create vm
                let num_vms = input.usize();

                let vm_type = input.usize() - 1;
                let placement_group_id = input.usize() - 1;
                // TODO: better type
                let partition_group = input.i32();

                // if let Some(opp) = expected_next_op {
                //     dbg!(opp);
                //     assert!(total_queries == opp.total_queries);
                //     assert!(placement_group_id == opp.pg);
                //     assert!(partition_group == opp.group_id);
                // }

                pg_stats.entry(placement_group_id).and_modify(|stats| {
                    stats.created[vm_type] += num_vms;
                    stats.change_times.push(total_queries)
                });

                let prev = last_pg_used.get(&placement_group_id).unwrap_or(&0);
                log.push(format!(
                    "{total_queries}. CREATE VMS: {num_vms} x {:?}, group_id={partition_group}, pg={placement_group_id}, prev_time={prev}",
                    params.vm_specs[vm_type]
                ));
                last_pg_used.insert(placement_group_id, total_queries);

                let indexes = input.vec::<usize>(num_vms).sub_from_all(1);

                if let Some(res) =
                    solver.create_vms(vm_type, placement_group_id, partition_group, &indexes)
                {
                    state.register_new_vms(&res);
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
                    dbg!(
                        "Can't create vms...",
                        num_vms,
                        partition_group,
                        params.vm_specs[vm_type]
                    );
                    dbg!(solver.placement_groups[placement_group_id]);
                    // state.analyze_failure(&format!(
                    //     "a_topology_aware_vmplacement/pics/{}-state-best.png",
                    //     test_case
                    // ));

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

                let mut by_type = vec![0; params.vm_specs.len()];
                for &id in ids.iter() {
                    by_type[params.vm_specs.index_of(&state.vms[id].spec).unwrap()] += 1;
                }
                let mut msg = format!("{total_queries}. DEL VMs: {num_vms}.");
                for type_id in 0..by_type.len() {
                    if by_type[type_id] != 0 {
                        msg += &format!(" {} x {:?}.", by_type[type_id], params.vm_specs[type_id]);
                    }
                }
                for &id in ids.iter() {
                    let created_vm = &state.vms[id];
                    let type_id = params.vm_specs.index_of(&created_vm.spec).unwrap();
                    pg_stats
                        .entry(created_vm.placement_group_id)
                        .and_modify(|stats| stats.deleted[type_id] += 1);
                }
                // let cnt_types = by_type.iter().filter(|&x| *x != 0).count();
                // assert!(cnt_types == 1);
                log.push(msg);

                solver.delete_vms(&ids);
                state.delete_vms(&ids);
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
    state.save_png(&format!(
        "a_topology_aware_vmplacement/pics/{}-state.png",
        test_case
    ));
    dbg!(total_vms_created, total_queries);

    // if are_all_pgs_are_one_type(&pg_stats) {
    //     while start.elapsed().as_millis() < 13000 {}
    // }

    //START MAIN
    save_log(test_case, log);
    save_pg_stats(test_case, &pg_stats, &params);
    save_vm_types(test_case, &state.get_num_vms_by_type());
    //END MAIN

    // solver.finish(test_case);

    Result {
        vms_created: total_vms_created,
        vms_without_soft: 0,
    }
}

fn save_log(test_case: usize, log: Vec<String>) {
    set_global_output_to_file(&format!(
        "a_topology_aware_vmplacement/logs/{}-log.txt",
        test_case
    ));
    for l in log.into_iter() {
        out_line!(l);
    }
    output().flush();
}

fn save_vm_types(test_case: usize, vm_types: &BTreeMap<VmSpec, usize>) {
    set_global_output_to_file(&format!(
        "a_topology_aware_vmplacement/logs/{}-vm-types.txt",
        test_case
    ));
    for (k, v) in vm_types.iter() {
        out_line!(format!("{:?} -> {v}", k));
    }
    output().flush();
}

fn are_all_pgs_are_one_type(pg_stats: &BTreeMap<usize, PG_stat>) -> bool {
    for (k, v) in pg_stats.iter() {
        if v.created.iter().filter(|&x| *x != 0).count() > 1 {
            return false;
        }
    }
    return true;
}

fn save_pg_stats(test_case: usize, pg_stats: &BTreeMap<usize, PG_stat>, params: &TestParams) {
    set_global_output_to_file(&format!(
        "a_topology_aware_vmplacement/logs/{}-pg-stats.txt",
        test_case
    ));
    for (k, v) in pg_stats.iter() {
        let mut msg = format!(
            "{k}. Pt={}, RA={}. Times: {:?} ",
            v.partitions, v.rack_affinity, v.change_times
        );
        for i in 0..v.created.len() {
            if v.created[i] != 0 {
                msg += &format!(
                    "{:?} * (+{}, -{}). ",
                    params.vm_specs[i], v.created[i], v.deleted[i],
                );
            }
        }
        out_line!(msg);
    }
    output().flush();
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
