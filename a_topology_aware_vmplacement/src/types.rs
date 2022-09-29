use std::collections::BTreeMap;

use crate::usage_stats::MachineUsedStats;

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

pub struct PlacementGroupVms {
    pub id_to_part: BTreeMap<usize, i32>,
    cnt_by_machine: BTreeMap<MachineId, usize>,
}

impl PlacementGroupVms {
    pub fn new() -> Self {
        Self {
            id_to_part: BTreeMap::default(),
            cnt_by_machine: BTreeMap::default(),
        }
    }

    pub fn unregister_vm(&mut self, id: usize, machine_id: MachineId) {
        self.id_to_part.remove(&id);
        *self.cnt_by_machine.entry(machine_id).or_default() -= 1;
    }

    pub fn register_vm(&mut self, id: usize, part: i32, machine_id: MachineId) {
        self.id_to_part.insert(id, part);
        *self.cnt_by_machine.entry(machine_id).or_default() += 1;
    }

    pub fn any_vm_id(&self) -> Option<usize> {
        self.id_to_part.keys().next().map(|&x| x)
    }
}

pub fn are_soft_constraints_already_violated(
    pg: &PlacementGroup,
    info: &PlacementGroupVms,
    created_vms: &[CreatedVm],
) -> bool {
    if let Some(any_vm) = info.any_vm_id() {
        for &vm_id in info.id_to_part.keys() {
            if pg.network_affinity_type == 1
                && created_vms[vm_id].machine.dc != created_vms[any_vm].machine.dc
            {
                return true;
            }
            if pg.rack_affinity_type == 1
                && created_vms[vm_id].machine.get_rack() != created_vms[any_vm].machine.get_rack()
            {
                return true;
            }
        }
    }
    false
}
