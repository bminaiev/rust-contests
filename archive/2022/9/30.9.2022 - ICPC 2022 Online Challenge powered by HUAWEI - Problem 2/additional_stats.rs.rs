use std::collections::BTreeMap;

use algo_lib::{
    collections::array_2d::Array2D,
    dbg,
    io::output::{output, set_global_output_to_file},
    out, out_line,
};

use crate::{
    state::State,
    types::{TestParams, VmSpec},
};
use algo_lib::collections::index_of::IndexOf;
pub fn save_log(test_case: usize, log: Vec<String>) {
    set_global_output_to_file(&format!(
        "a_topology_aware_vmplacement/logs/{}-log.txt",
        test_case
    ));
    for l in log.into_iter() {
        out_line!(l);
    }
    output().flush();
}

pub fn save_vm_types(test_case: usize, vm_types: &BTreeMap<VmSpec, usize>) {
    set_global_output_to_file(&format!(
        "a_topology_aware_vmplacement/logs/{}-vm-types.txt",
        test_case
    ));
    for (k, v) in vm_types.iter() {
        out_line!(format!("{:?} -> {v}", k));
    }
    output().flush();
}

pub fn save_vms_del_time(test_case: usize, state: &State) {
    set_global_output_to_file(&format!(
        "a_topology_aware_vmplacement/logs/{}-vm-del-times.txt",
        test_case
    ));
    let mut diffs = vec![];
    let max_time = state.create_del_times.iter().map(|x| x.0).max().unwrap();
    let mut cnt = Array2D::new(0, state.params.vm_specs.len(), max_time + 1);

    for i in 0..state.vms.len() {
        let spec = state.vms[i].spec;
        if i > 0 && state.create_del_times[i].0 != state.create_del_times[i - 1].0 {
            out_line!();
        }
        let vm_type = state.params.vm_specs.index_of(&state.vms[i].spec).unwrap();
        cnt[vm_type][state.create_del_times[i].0] += 1;
        diffs.push(state.create_del_times[i].1.unwrap_or(max_time) - state.create_del_times[i].0);
        if let Some(d) = state.create_del_times[i].1 {
            cnt[vm_type][state.create_del_times[i].1.unwrap()] -= 1;
        }
        out_line!(format!(
            "spec={:?}, created={}, deteled={:?}",
            spec, state.create_del_times[i].0, state.create_del_times[i].1
        ));
        // if i + 1 < state.vms.len() && state.create_del_times[i].0 == state.create_del_times[i + 1].0
        // {
        //     if let Some(d1) = state.create_del_times[i].1 {
        //         let d2 = state.create_del_times[i + 1].1.unwrap();
        //         assert!(d1 >= d2);
        //     }
        // }
    }
    // diffs.sort();

    // for i in 0..diffs.len() {
    //     if i % 10 == 0 {
    //         out_line!(diffs[i]);
    //     }
    // }

    let mut sum = vec![0; state.params.vm_specs.len()];
    for i in 0..max_time {
        for j in 0..sum.len() {
            sum[j] += cnt[j][i];
        }
        if i % 100 == 0 {
            out!(i, '\t');
            for j in 0..sum.len() {
                out!(sum[j], '\t');
            }
            out_line!();
        }
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

#[derive(Clone)]
pub struct PG_stat {
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

pub fn save_pg_stats(test_case: usize, pg_stats: &BTreeMap<usize, PG_stat>, params: &TestParams) {
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
