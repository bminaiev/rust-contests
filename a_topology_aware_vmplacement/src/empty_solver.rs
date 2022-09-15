use std::cmp::max;

use crate::types::{CreatedVm, MachineId, Numa, PlacementGroup, VmSpec};
use algo_lib::dbg;
use algo_lib::misc::rand::Random;
use marathon_utils::dynamic_plot::DynamicPlot;

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

pub struct EmptySolver {
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
    pic_vms_used: DynamicPlot,
    pic_cpu_used: DynamicPlot,
    pic_mem_used: DynamicPlot,
    pic_vm_types_used: DynamicPlot,
    pic_placement_groups: DynamicPlot,
    vms_alive: usize,
    cpu_used: i32,
    mem_used: i32,
    tot_placement_groups: i32,
    vms_used_by_type: Vec<usize>,
    vm_types_by_ids: Vec<usize>,
}

impl EmptySolver {
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
            vms_used_by_type: vec![0; vm_types.len()],
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
            pic_vms_used: DynamicPlot::new("Num VMs Used", "Time", "Cnt"),
            pic_cpu_used: DynamicPlot::new("Cpu Used", "Time", "Cpu"),
            pic_mem_used: DynamicPlot::new("Mem Used", "Time", "Mem"),
            pic_vm_types_used: DynamicPlot::new("VM types Used", "Time", "Cnt"),
            pic_placement_groups: DynamicPlot::new("Placement groups", "Time", "Groups"),
            vms_alive: 0,
            cpu_used: 0,
            mem_used: 0,
            tot_placement_groups: 0,

            vm_types_by_ids: vec![],
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
        self.tot_placement_groups += 1;
    }

    pub fn create_vms(
        &mut self,
        vm_type: usize,
        placement_group_id: usize,
        partition_group: i32,
        indexes: &[usize],
    ) -> Option<Vec<CreatedVm>> {
        let num_vms = indexes.len();
        self.vms_alive += num_vms;
        self.cpu_used += (num_vms as i32) * (self.vm_types[vm_type].cpu as i32);
        self.mem_used += (num_vms as i32) * (self.vm_types[vm_type].memory as i32);
        self.vm_types_by_ids.extend(vec![vm_type; num_vms]);
        self.vms_used_by_type[vm_type] += num_vms;
        Some(vec![])
    }

    pub fn delete_vms(&mut self, ids: &[usize]) {
        let num_vms = ids.len();
        self.vms_alive -= num_vms;
        for &id in ids.iter() {
            let vm_type = self.vm_types_by_ids[id];
            self.cpu_used -= self.vm_types[vm_type].cpu as i32;
            self.mem_used -= self.vm_types[vm_type].memory as i32;
            self.vms_used_by_type[vm_type] -= 1;
        }
    }

    pub fn finish(&self, test_case: usize) {
        let pic_base_dir = "a_topology_aware_vmplacement/pics";
        self.pic_vms_used
            .save_image(pic_base_dir, &format!("{}-vms_alive", test_case));
        self.pic_cpu_used
            .save_image(pic_base_dir, &format!("{}-cpu_used", test_case));
        self.pic_mem_used
            .save_image(pic_base_dir, &format!("{}-mem_used", test_case));
        self.pic_placement_groups
            .save_image(pic_base_dir, &format!("{}-placement_groups", test_case));
        self.pic_vm_types_used
            .save_image(pic_base_dir, &format!("{}-vms_types_alive", test_case));
        self.pic_vm_types_used.save_sub_image(
            pic_base_dir,
            &format!("{}-vms_types_alive.1", test_case),
            self.vm_types.len() - 1,
        );
        dbg!(&self.vms_used_by_type);
    }

    pub fn step(&mut self, total_queries: usize) {
        self.pic_vms_used.add_point(total_queries, self.vms_alive);
        self.pic_cpu_used.add_point(total_queries, self.cpu_used);
        self.pic_mem_used.add_point(total_queries, self.mem_used);
        self.pic_placement_groups
            .add_point(total_queries, self.tot_placement_groups);
        for i in 0..self.vms_used_by_type.len() {
            self.pic_vm_types_used.add_point_scecific_graph(
                total_queries,
                self.vms_used_by_type[i],
                i,
            );
        }
    }
}
