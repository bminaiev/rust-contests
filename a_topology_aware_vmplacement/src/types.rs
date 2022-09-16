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
}
