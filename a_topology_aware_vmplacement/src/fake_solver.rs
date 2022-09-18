use std::{
    cmp::{max, min},
    collections::HashMap,
};

use algo_lib::misc::{min_max::FindMinMaxPos, rand::Random};
use algo_lib::{collections::index_of::IndexOf, dbg};

use crate::{
    state::State,
    types::{CreatedVm, MachineId, PlacementGroup, RackId, TestParams, VmSpec},
    usage_stats::{MachineUsedStats, NumaUsedStats},
};

pub struct FakeSolver {
    params: TestParams,
    pub placement_groups: Vec<PlacementGroup>,
    created_vms: Vec<CreatedVm>,
    is_vm_alive: Vec<bool>,
}

impl FakeSolver {
    pub fn new(params: TestParams) -> Self {
        Self {
            params,
            placement_groups: vec![],
            created_vms: vec![],
            is_vm_alive: vec![],
        }
    }
    pub fn new_placement_group(&mut self, idx: usize, placement_group: PlacementGroup) {
        assert!(self.placement_groups.len() == idx);
        self.placement_groups.push(placement_group);
    }

    pub fn delete_vms(&mut self, idxs: &[usize]) {
        for &id in idxs.iter() {
            self.is_vm_alive[id] = false;
        }
    }

    fn can_place_from_start(&self) -> bool {
        let mut vms_by_type = vec![0; self.params.vm_specs.len()];
        for i in 0..self.is_vm_alive.len() {
            if self.is_vm_alive[i] {
                let type_id = self
                    .params
                    .vm_specs
                    .index_of(&self.created_vms[i].spec)
                    .unwrap();
                vms_by_type[type_id] += 1;
            }
        }
        let mut machines_stats = self.params.gen_usage_stats();
        let mut best_state = State::new(self.params.clone());
        for id in (0..vms_by_type.len()).rev() {
            let spec = self.params.vm_specs[id];
            let mut more = vms_by_type[id];
            for m_id in 0..machines_stats.len() {
                let machine = self.params.get_machine_by_id(m_id);
                while more > 0 {
                    if let Some(placement) = machines_stats[m_id].can_place_vm(&spec, machine, 0) {
                        machines_stats[m_id].register_vm(&placement);
                        best_state.register_new_vms(&[placement]);
                        more -= 1;
                    } else {
                        break;
                    }
                }
            }
            if more > 0 {
                return false;
            }
        }

        true
    }

    pub fn create_vms(
        &mut self,
        vm_type: usize,
        placement_group_id: usize,
        partition_group: i32,
        indexes: &[usize],
    ) -> Option<Vec<CreatedVm>> {
        assert!(vm_type < self.params.vm_specs.len());
        assert!(indexes[0] == self.created_vms.len());

        let mut res = vec![];
        let spec = self.params.vm_specs[vm_type];

        for i in 0..indexes.len() {
            res.push(CreatedVm {
                machine: MachineId {
                    dc: 0,
                    rack: 0,
                    inside_rack: 0,
                },
                numa_ids: vec![],
                spec,
                placement_group_id: 0,
            })
        }

        assert_eq!(res.len(), indexes.len());
        self.created_vms.extend(res.clone());
        self.is_vm_alive.extend(vec![true; res.len()]);

        if !self.can_place_from_start() {
            return None;
        }
        Some(res)
    }
}
