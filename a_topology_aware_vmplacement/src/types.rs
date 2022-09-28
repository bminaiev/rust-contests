use algo_lib::misc::gen_vector::gen_vec;

use crate::usage_stats::{MachineUsedStats, NumaUsedStats};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Numa {
    pub cpu: u32,
    pub memory: u32,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct VmSpec {
    pub numa_cnt: usize,
    pub cpu: u32,
    pub memory: u32,
}

#[derive(Clone, Copy, Debug)]
pub struct PlacementGroup {
    pub hard_rack_anti_affinity_partitions: usize,
    pub soft_max_vms_per_machine: usize,
    pub network_affinity_type: usize,
    pub rack_affinity_type: usize,
}

impl PlacementGroup {
    pub fn has_soft_constraints(&self) -> bool {
        self.network_affinity_type == 1
            || self.rack_affinity_type == 1
            || self.soft_max_vms_per_machine > 0
    }
}

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct MachineId {
    pub dc: usize,
    pub rack: usize,
    pub inside_rack: usize,
}

impl MachineId {
    pub fn get_rack(&self) -> RackId {
        RackId {
            dc: self.dc,
            rack: self.rack,
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash, PartialOrd, Ord)]
pub struct RackId {
    pub dc: usize,
    pub rack: usize,
}

impl RackId {
    pub const FAKE: Self = Self {
        dc: std::usize::MAX,
        rack: std::usize::MAX,
    };
}

#[derive(Clone, Debug)]
pub struct CreatedVm {
    pub machine: MachineId,
    pub numa_ids_mask: u32,
    pub spec: VmSpec,
    pub placement_group_id: usize,
}

#[derive(Clone, Debug)]
pub struct TestParams {
    pub num_dc: usize,
    pub num_racks: usize,
    pub num_machines_per_rack: usize,
    pub numa: Vec<Numa>,
    pub vm_specs: Vec<VmSpec>,
    pub machine_ids: Vec<MachineId>,
}

impl TestParams {
    pub fn new(
        num_dc: usize,
        num_racks: usize,
        num_machines_per_rack: usize,
        numa: Vec<Numa>,
        vm_specs: Vec<VmSpec>,
    ) -> Self {
        let mut machine_ids = vec![];
        for dc in 0..num_dc {
            for rack in 0..num_racks {
                for inside_rack in 0..num_machines_per_rack {
                    machine_ids.push(MachineId {
                        dc,
                        rack,
                        inside_rack,
                    });
                }
            }
        }
        Self {
            num_dc,
            num_racks,
            num_machines_per_rack,
            numa,
            vm_specs,
            machine_ids,
        }
    }

    pub fn total_machines(&self) -> usize {
        self.num_dc * self.num_racks * self.num_machines_per_rack
    }

    pub fn get_machine_id(&self, machine: &MachineId) -> usize {
        machine.inside_rack
            + machine.rack * self.num_machines_per_rack
            + machine.dc * (self.num_machines_per_rack * self.num_racks)
    }

    pub fn get_machine_by_id(&self, id: usize) -> MachineId {
        self.machine_ids[id]
        // MachineId {
        //     dc: id / self.num_machines_per_rack / self.num_racks,
        //     rack: (id / self.num_machines_per_rack) % self.num_racks,
        //     inside_rack: id % (self.num_machines_per_rack),
        // }
    }

    pub(crate) fn get_machine_id2(&self, dc: usize, rack: usize, inside_rack: usize) -> usize {
        inside_rack
            + rack * self.num_machines_per_rack
            + dc * (self.num_machines_per_rack * self.num_racks)
    }

    pub fn gen_usage_stats(&self) -> Vec<MachineUsedStats> {
        vec![MachineUsedStats::new(self); self.total_machines()]
    }
}
