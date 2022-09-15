//{"name":"A. Topology-Aware VM Placement","group":"Codeforces - ICPC 2022 Online Challenge powered by HUAWEI - Problem 2","url":"https://codeforces.com/contest/1724/problem/A","interactive":true,"timeLimit":15000,"tests":[{"input":"2 2 2 2\n8 16\n8 16\n3\n1 1 1\n1 4 2\n2 4 8\n1\n1 4 1\n1 0\n2\n3 1 1 1\n1 2 3\n2\n3 1 1 1\n4 5 6\n1\n2 0 0\n2 2\n2\n3 3 2 0\n7 8 9\n3\n3 3 5 6\n2\n2 3 1 4\n10 11\n2\n2 3 1 3\n12 13\n4\n","output":"1 1 1 1\n1 1 2 1\n1 2 1 1\n2 1 1 1\n2 1 2 1\n2 2 1 1\n1 2 1 1 2\n1 2 2 1 2\n1 2 2 1 2\n2 2 1 1 2\n2 2 2 1 2\n-1\n"}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"ATopologyAwareVMPlacement"}}}

use std::cmp::max;

use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::gen_vector::gen_vec;
use algo_lib::misc::rand::Random;
use algo_lib::misc::vec_apply_delta::ApplyDelta;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

#[derive(Clone, Copy, Debug)]
struct Numa {
    cpu: u32,
    memory: u32,
}

#[derive(Clone, Copy, Debug)]
struct VmSpec {
    numa_cnt: usize,
    cpu: u32,
    memory: u32,
}

#[derive(Clone, Copy, Debug)]
struct PlacementGroup {
    hard_rack_anti_affinity_partitions: usize,
    soft_max_vms_per_machine: usize,
    network_affinity_type: usize,
    rack_affinity_type: usize,
}

#[derive(Clone, Copy, Debug)]
struct MachineId {
    dc: usize,
    rack: usize,
    inside_rack: usize,
}

#[derive(Clone)]
struct PlacementGroupMapping {
    possible_machines: Vec<Vec<MachineId>>,
}

#[derive(Clone, Copy, Debug)]
struct NumaUsedStats {
    free_cpu: u32,
    free_memory: u32,
}

impl NumaUsedStats {
    pub fn can_place(&self, vm: &VmSpec) -> bool {
        self.free_cpu >= vm.cpu && self.free_memory >= vm.memory
    }

    pub fn register_vm(&mut self, vm: &VmSpec) {
        assert!(self.can_place(vm));
        self.free_cpu -= vm.cpu;
        self.free_memory -= vm.memory;
    }

    pub fn unregister_vm(&mut self, vm: &VmSpec) {
        self.free_cpu += vm.cpu;
        self.free_memory += vm.memory;
    }
}

#[derive(Clone, Debug)]
struct MachineUsedStats {
    numa: Vec<NumaUsedStats>,
}

struct Solver {
    num_dc: usize,
    num_racks: usize,
    num_machines_per_rack: usize,
    numa: Vec<Numa>,
    vm_types: Vec<VmSpec>,
    placement_groups: Vec<PlacementGroup>,
    placement_group_mappings: Vec<PlacementGroupMapping>,
    machines: Vec<MachineId>,
    machines_stats: Vec<MachineUsedStats>,
    created_vms: Vec<CreatedVm>,
    created_vm_specs: Vec<VmSpec>,
    rnd: Random,
}

#[derive(Clone, Debug)]
struct CreatedVm {
    machine: MachineId,
    numa_ids: Vec<usize>,
}

impl Solver {
    pub fn new(
        num_dc: usize,
        num_racks: usize,
        num_machines_per_rack: usize,
        numa: Vec<Numa>,
        vm_types: Vec<VmSpec>,
    ) -> Self {
        let mut machines = vec![];
        for dc in 0..num_dc {
            for rack in 0..num_racks {
                for inside_rack in 0..num_machines_per_rack {
                    machines.push(MachineId {
                        dc,
                        rack,
                        inside_rack,
                    });
                }
            }
        }
        let machines_stats = vec![
            MachineUsedStats {
                numa: numa
                    .iter()
                    .map(|numa| NumaUsedStats {
                        free_cpu: numa.cpu,
                        free_memory: numa.memory
                    })
                    .collect()
            };
            machines.len()
        ];
        Self {
            num_dc,
            num_racks,
            num_machines_per_rack,
            numa,
            vm_types,
            placement_groups: vec![],
            placement_group_mappings: vec![],
            machines,
            created_vms: vec![],
            created_vm_specs: vec![],
            machines_stats,
            rnd: Random::new(787788),
        }
    }
    pub fn new_placement_group(
        &mut self,
        idx: usize,
        hard_rack_anti_affinity_partitions: usize,
        soft_max_vms_per_machine: usize,
        network_affinity_type: usize,
        rack_affinity_type: usize,
    ) {
        assert!(self.placement_groups.len() == idx);
        self.placement_groups.push(PlacementGroup {
            hard_rack_anti_affinity_partitions,
            soft_max_vms_per_machine,
            network_affinity_type,
            rack_affinity_type,
        });
        let num_groups = max(1, hard_rack_anti_affinity_partitions);
        if hard_rack_anti_affinity_partitions != 0 {
            assert!(rack_affinity_type == 0);
        }
        let mut possible_machines = vec![vec![]; num_groups];
        if rack_affinity_type == 2 {
            // all should go to one rack
            assert!(possible_machines.len() == 1);
            let dc_id = self.rnd.gen(0..self.num_dc);
            let rack_id = self.rnd.gen(0..self.num_racks);
            for m in self.machines.iter() {
                if m.dc == dc_id && m.rack == rack_id {
                    possible_machines[0].push(m.clone());
                }
            }
        } else {
            #[derive(Clone)]
            struct AvailableRack {
                dc: usize,
                rack: usize,
            }
            let mut racks = vec![];
            if network_affinity_type == 2 {
                let dc = self.rnd.gen(0..self.num_dc);
                for rack_id in 0..self.num_racks {
                    racks.push(AvailableRack { dc, rack: rack_id });
                }
            } else {
                for dc in 0..self.num_dc {
                    for rack_id in 0..self.num_racks {
                        racks.push(AvailableRack { dc, rack: rack_id });
                    }
                }
            }
            let mut group_id = 0;
            for idx in self.rnd.gen_permutation(racks.len()) {
                let rack = &racks[idx];
                for inside_rack in 0..self.num_machines_per_rack {
                    possible_machines[group_id]
                        .push(self.machines[self.get_machine_id(rack.dc, rack.rack, inside_rack)]);
                }
                group_id = (group_id + 1) % possible_machines.len();
            }
        }
        self.placement_group_mappings
            .push(PlacementGroupMapping { possible_machines });
    }

    fn get_machine_id(&self, dc: usize, rack: usize, inside_rack: usize) -> usize {
        inside_rack
            + rack * self.num_machines_per_rack
            + dc * self.num_machines_per_rack * self.num_racks
    }

    fn get_machine_id2(&self, machine: &MachineId) -> usize {
        self.get_machine_id(machine.dc, machine.rack, machine.inside_rack)
    }

    pub fn delete_vms(&mut self, idxs: &[usize]) {
        for id in idxs.iter() {
            let vm = self.created_vms[*id].clone();
            self.unregister_vm(&vm, &self.created_vm_specs[*id].clone());
        }
    }

    fn can_place_vm(&self, machine_id: usize, vm: &VmSpec) -> Option<CreatedVm> {
        if vm.numa_cnt == 1 {
            for numa_id in 0..self.numa.len() {
                if self.machines_stats[machine_id].numa[numa_id].can_place(&vm) {
                    return Some(CreatedVm {
                        machine: self.machines[machine_id],
                        numa_ids: vec![numa_id],
                    });
                }
            }
        } else {
            assert_eq!(vm.numa_cnt, 2);
            for numa_id1 in 0..self.numa.len() {
                if self.machines_stats[machine_id].numa[numa_id1].can_place(&vm) {
                    for numa_id2 in numa_id1 + 1..self.numa.len() {
                        if self.machines_stats[machine_id].numa[numa_id2].can_place(&vm) {
                            return Some(CreatedVm {
                                machine: self.machines[machine_id],
                                numa_ids: vec![numa_id1, numa_id2],
                            });
                        }
                    }
                }
            }
        }
        None
    }

    fn register_vm(&mut self, vm: &CreatedVm, spec: &VmSpec) {
        let m_id = self.get_machine_id2(&vm.machine);
        for &numa_id in vm.numa_ids.iter() {
            self.machines_stats[m_id].numa[numa_id].register_vm(spec);
        }
    }

    fn unregister_vm(&mut self, vm: &CreatedVm, spec: &VmSpec) {
        let m_id = self.get_machine_id2(&vm.machine);
        for &numa_id in vm.numa_ids.iter() {
            self.machines_stats[m_id].numa[numa_id].unregister_vm(spec);
        }
    }

    pub fn create_vms(
        &mut self,
        vm_type: usize,
        placement_group_id: usize,
        partition_group: i32,
        indexes: &[usize],
    ) -> Option<Vec<CreatedVm>> {
        assert!(vm_type < self.vm_types.len());
        assert!(indexes[0] == self.created_vms.len());

        // TODO: make it faster...
        let mapping = self.placement_group_mappings[placement_group_id].clone();
        let mut res = vec![];

        let spec = self.vm_types[vm_type];

        if partition_group == -1 {
            assert_eq!(indexes.len(), mapping.possible_machines.len());
            for i in 0..indexes.len() {
                // TODO: remove clone
                let machines: Vec<MachineId> = mapping.possible_machines[i].clone();
                for m in machines.iter() {
                    if let Some(placement) = self.can_place_vm(self.get_machine_id2(m), &spec) {
                        self.register_vm(&placement, &spec);
                        res.push(placement);
                        break;
                    }
                }
                if res.len() != i + 1 {
                    return None;
                }
            }
        } else {
            let group_id = if partition_group == 0 {
                0
            } else {
                partition_group as usize - 1
            };
            for i in 0..indexes.len() {
                let machines = &mapping.possible_machines[group_id];
                for m in machines.iter() {
                    if let Some(placement) = self.can_place_vm(self.get_machine_id2(m), &spec) {
                        self.register_vm(&placement, &spec);
                        res.push(placement);
                        break;
                    }
                }
                if res.len() != i + 1 {
                    return None;
                }
            }
        }

        if res.len() != indexes.len() {
            return None;
        }
        // for vm in res.iter() {
        //     self.register_vm(vm, &spec);
        // }
        self.created_vm_specs.extend(vec![spec; res.len()]);
        self.created_vms.extend(res.clone());

        Some(res)
    }
}

fn solve(input: &mut Input, _test_case: usize, print_result: bool) {
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
    let mut solver = Solver::new(num_dc, num_racks, num_machines_per_rack, numa, vm_types);
    let mut total_vms_created = 0;
    let mut total_vms_deleted = 0;
    loop {
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
                total_vms_deleted += num_vms;
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
    }
    dbg!(total_vms_created);
    dbg!(total_vms_deleted);
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
    for test_id in 1..12 {
        dbg!(test_id);
        let mut input = Input::new_file(format!(
            "./a_topology_aware_vmplacement/local_test_kit/sample/{:02}",
            test_id
        ));
        solve(&mut input, test_id, false);
        let baseline = read_baseline(test_id);
        dbg!(baseline);
    }
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
