#[derive(Clone, Copy, Debug)]
pub struct Numa {
    pub cpu: u32,
    pub memory: u32,
}

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct MachineId {
    pub dc: usize,
    pub rack: usize,
    pub inside_rack: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct RackId {
    pub dc: usize,
    pub rack: usize,
}

#[derive(Clone, Debug)]
pub struct CreatedVm {
    pub machine: MachineId,
    pub numa_ids: Vec<usize>,
    pub spec: VmSpec,
}

#[derive(Clone, Debug)]
pub struct TestParams {
    pub num_dc: usize,
    pub num_racks: usize,
    pub num_machines_per_rack: usize,
    pub numa: Vec<Numa>,
    pub vm_specs: Vec<VmSpec>,
}

impl TestParams {
    pub fn total_machines(&self) -> usize {
        self.num_dc * self.num_racks * self.num_machines_per_rack
    }

    pub fn get_machine_id(&self, machine: &MachineId) -> usize {
        machine.inside_rack
            + machine.rack * self.num_machines_per_rack
            + machine.dc * (self.num_machines_per_rack * self.num_racks)
    }

    pub(crate) fn get_machine_id2(&self, dc: usize, rack: usize, inside_rack: usize) -> usize {
        inside_rack
            + rack * self.num_machines_per_rack
            + dc * (self.num_machines_per_rack * self.num_racks)
    }
}
