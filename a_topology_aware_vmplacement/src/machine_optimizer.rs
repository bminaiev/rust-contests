use std::cmp::min;
use std::collections::HashSet;

use algo_lib::collections::array_2d::Array2D;
use algo_lib::collections::last_exn::LastExn;
use algo_lib::io::output::output;
use algo_lib::math::frac::Frac;
use algo_lib::misc::rand::Random;
use algo_lib::misc::rec_function::{Callable3, RecursiveFunction3};
use algo_lib::{dbg, out, out_line};

use algo_lib::{io::input::Input, misc::gen_vector::gen_vec};

use crate::types::{TestParams, VmSpec};
use crate::usage_stats::MachineUsedStats;
use crate::usage_stats::NumaUsedStats;

fn load_cnt(test_case: usize, params: &TestParams) -> Vec<usize> {
    let mut input = Input::new_file(format!(
        "a_topology_aware_vmplacement/local_test_kit/sample/{:02}-cnt.txt",
        test_case
    ));
    gen_vec(params.vm_specs.len(), |_| input.read())
}

#[derive(Clone)]
struct Placement {
    vm_type: usize,
    vm_spec: VmSpec,
    numa_ids: Vec<usize>,
}

fn apply_placement(machine_stat: &mut MachineUsedStats, placement: &Placement) -> bool {
    for &numa_id in placement.numa_ids.iter() {
        if machine_stat.numa[numa_id].can_place(&placement.vm_spec) {
            machine_stat.numa[numa_id].register_vm(&placement.vm_spec)
        } else {
            return false;
        }
    }
    true
}

fn gen_placements(params: &TestParams) -> Vec<Placement> {
    let mut placements = vec![];
    let vm_specs = &params.vm_specs;
    let numa_cnt_in_vm = params.numa.len();
    for i in 0..vm_specs.len() {
        for mask in 0i32..1 << numa_cnt_in_vm {
            if mask.count_ones() as usize == vm_specs[i].numa_cnt {
                let mut numa_ids = vec![];
                for j in 0..numa_cnt_in_vm {
                    if (1 << j) & mask != 0 {
                        numa_ids.push(j);
                    }
                }
                placements.push(Placement {
                    vm_type: i,
                    vm_spec: vm_specs[i].clone(),
                    numa_ids,
                });
            }
        }
    }
    placements
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Possibility {
    Good(i64),
    Reduce(usize, usize),
}

impl Possibility {
    pub fn is_good(&self) -> bool {
        match self {
            Possibility::Good(_) => true,
            Possibility::Reduce(_, _) => false,
        }
    }

    pub fn get_score(&self) -> i64 {
        match self {
            Possibility::Good(score) => *score,
            Possibility::Reduce(_, _) => std::i64::MAX,
        }
    }
}

fn gen_possibles(params: &TestParams, need_cnt: &[usize], scores: &[i64]) -> Array2D<Possibility> {
    out_line!("need_cnt:", format!("{:?}", need_cnt));
    let max_cpu = params.numa.iter().map(|numa| numa.cpu).max().unwrap() as usize;
    let max_memory = params.numa.iter().map(|numa| numa.memory).max().unwrap() as usize;

    let mut possible = Array2D::new(Possibility::Reduce(0, 0), max_cpu + 1, max_memory + 1);
    possible[0][0] = Possibility::Good(std::i64::MAX);
    for cpu in 0..=max_cpu {
        for memory in 0..=max_memory {
            if possible[cpu][memory] == Possibility::Reduce(0, 0) {
                if cpu > 0 {
                    if let Possibility::Reduce(_, _) = possible[cpu - 1][memory] {
                        possible[cpu][memory] = possible[cpu - 1][memory];
                    } else {
                        possible[cpu][memory] = Possibility::Reduce(cpu - 1, memory);
                    }
                }
                if memory > 0 {
                    if let Possibility::Reduce(x, y) = possible[cpu][memory - 1] {
                        if let Possibility::Reduce(x2, y2) = possible[cpu][memory] {
                            if x2 + y2 < x + y {
                                possible[cpu][memory] = possible[cpu][memory - 1];
                            }
                        } else {
                            unreachable!();
                        }
                    } else {
                        possible[cpu][memory] = Possibility::Reduce(cpu, memory - 1);
                    }
                }
                continue;
            }
            if let Possibility::Good(cur_score) = possible[cpu][memory] {
                for (it, vm_spec) in params.vm_specs.iter().enumerate() {
                    // TODO: this is bad, potentially
                    if vm_spec.numa_cnt <= 2 && need_cnt[it] > 0 {
                        let next_cpu = cpu + (vm_spec.cpu as usize);
                        let next_memory = memory + (vm_spec.memory as usize);
                        if next_cpu <= max_cpu && next_memory <= max_memory {
                            let next_score = min(cur_score, scores[it]);
                            if possible[next_cpu][next_memory].get_score() > next_score {
                                possible[next_cpu][next_memory] = Possibility::Good(next_score);
                            }
                        }
                    }
                }
            }
        }
    }
    possible
}

fn gen_machine(
    mut placements: Vec<Placement>,
    seed: u64,
    params: &TestParams,
    old_used: &[usize],
    need_cnt: &[usize],
) -> Vec<usize> {
    let cur_need_cnt = gen_vec(old_used.len(), |id| {
        if need_cnt[id] <= old_used[id] {
            0
        } else {
            need_cnt[id] - old_used[id]
        }
    });
    if !cur_need_cnt.iter().any(|&x| x > 0) {
        return vec![0; need_cnt.len()];
    }
    const MAX_COEF: i64 = 1_000_000_000;
    let scores = gen_vec(old_used.len(), |i| {
        if need_cnt[i] == 0 {
            MAX_COEF
        } else {
            (old_used[i] as i64) * MAX_COEF / need_cnt[i] as i64
        }
    });
    // dbg!(scores);
    // for i in 0..params.vm_specs.len() {
    //     if old_used[i] < need_cnt[i] {
    //         out_line!(format!(
    //             "{:?} -> {}, coef = {}",
    //             params.vm_specs[i],
    //             need_cnt[i] - old_used[i],
    //             scores[i]
    //         ));
    //     }
    // }
    let possible = gen_possibles(params, &cur_need_cnt, &scores);
    placements.sort_by_key(|pl| {
        const MAX_COEF: i64 = 1_000_000_000;
        let done_part = scores[pl.vm_type];
        (
            // pl.numa_ids.len(),
            std::i64::MAX - done_part,
            pl.vm_spec.memory,
            std::usize::MAX,
        )
    });
    placements.reverse();

    let mut machine = MachineUsedStats::new(params);
    let mut used_cnt = vec![0; params.vm_specs.len()];

    loop {
        for numa in machine.numa.iter_mut() {
            let mut best = (0, 0);

            for x in 0..=numa.free_cpu {
                for y in 0..=numa.free_memory {
                    if (
                        possible[x as usize][y as usize].get_score(),
                        best.0 + best.1,
                    ) < (
                        possible[best.0 as usize][best.1 as usize].get_score(),
                        x + y,
                    ) {
                        best = (x, y);
                    }
                }
            }
            if best.0 != numa.free_cpu || best.1 != numa.free_memory {
                // dbg!("Sub:", numa, best);
                // out_line!(format!(
                //     "Sub: {:?} {:?}, {:?}",
                //     numa, best, possible[best.0 as usize][best.1 as usize]
                // ));
                numa.free_cpu = best.0;
                numa.free_memory = best.1;
            }

            // if let Possibility::Reduce(x, y) =
            //     possible[numa.free_cpu as usize][numa.free_memory as usize]
            // {
            //     // dbg!("Reduce", numa, x, y);
            //     numa.free_cpu = x as u32;
            //     numa.free_memory = y as u32;
            // }
            // assert!(
            //     possible[numa.free_cpu as usize][numa.free_memory as usize] == Possibility::Good
            // );
        }
        if !machine.numa.iter().any(|n| n.free_cpu != 0) {
            break;
        }
        let mut iter = 0;
        let mut found = false;
        while iter < placements.len() {
            let pl = &placements[iter];
            let mut ok = used_cnt[pl.vm_type] < need_cnt[pl.vm_type];
            for &numa_id in pl.numa_ids.iter() {
                if machine.numa[numa_id].can_place(&pl.vm_spec) {
                    let left_cpu = machine.numa[numa_id].free_cpu - pl.vm_spec.cpu;
                    let left_memory = machine.numa[numa_id].free_memory - pl.vm_spec.memory;
                    if !possible[left_cpu as usize][left_memory as usize].is_good() {
                        ok = false;
                    }
                } else {
                    ok = false;
                }
            }
            if ok {
                found = true;
                used_cnt[pl.vm_type] += 1;
                assert!(apply_placement(&mut machine, pl));
            } else {
                iter += 1;
            }
        }
        if !found {
            // something about two numa nodes?
            assert!(used_cnt.iter().any(|&x| x > 0));
            break;
        }
    }

    used_cnt
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
struct State {
    counts: Vec<(NumaUsedStats, usize)>,
}

impl State {
    pub fn new_counts(mut counts: Vec<(NumaUsedStats, usize)>) -> Self {
        counts.sort();
        let mut new_counts: Vec<(NumaUsedStats, usize)> = vec![];
        for c in counts.into_iter() {
            if c.0.free_cpu == 0 && c.0.free_memory == 0 {
                continue;
            }
            if new_counts.is_empty() || new_counts.last_exn().0 != c.0 {
                new_counts.push(c);
            } else {
                let pos = new_counts.len() - 1;
                new_counts[pos].1 += c.1;
            }
        }
        Self { counts: new_counts }
    }

    pub fn new(params: &TestParams) -> Self {
        let ms = MachineUsedStats::new(params);
        assert!(ms.numa[0] == ms.numa[1] && ms.numa[2] == ms.numa[3]);
        // 4070 -> can't find
        // 4080 -> can find
        let cnt = 4070; //params.total_machines();
        Self::new_counts(vec![(ms.numa[0], cnt), (ms.numa[2], cnt)])
    }

    pub fn place_vms(&self, pos: usize, cnt: usize, vm_spec: &VmSpec) -> Self {
        let mut counts = self.counts.clone();
        if cnt != counts[pos].1 {
            counts.push((counts[pos].0, counts[pos].1 - cnt));
            counts[pos].1 = cnt;
        }
        counts[pos].0.register_vm(vm_spec);
        Self::new_counts(counts)
    }

    pub fn twice(&self) -> Self {
        let counts = self.counts.iter().map(|c| (c.0, c.1 * 2)).collect();
        Self::new_counts(counts)
    }

    pub fn clamp_(&self, cpu_div_memory_min: Frac, cpu_div_memory_max: Frac) -> Self {
        let mut new_counts = vec![];
        for (numa, cnt) in self.counts.iter() {
            let max_cpu =
                numa.free_memory * cpu_div_memory_max.num as u32 / cpu_div_memory_max.denum as u32;
            let cpu = min(max_cpu, numa.free_cpu);
            let max_memory =
                numa.free_cpu * cpu_div_memory_min.denum as u32 / cpu_div_memory_min.num as u32;
            let memory = min(max_memory, numa.free_memory);
            new_counts.push((
                NumaUsedStats {
                    free_memory: memory,
                    free_cpu: cpu,
                },
                *cnt,
            ));
        }
        Self::new_counts(new_counts)
    }

    pub fn sum_cpu(&self) -> u32 {
        let mut res = 0;
        for (k, v) in self.counts.iter() {
            res += k.free_cpu * (*v as u32);
        }
        res
    }

    pub fn sum_memory(&self) -> u32 {
        let mut res = 0;
        for (k, v) in self.counts.iter() {
            res += k.free_memory * (*v as u32);
        }
        res
    }
}

#[derive(Clone, Copy, Default)]
struct Stats {
    cpu: u32,
    memory: u32,
}

fn rec_search_numa(
    params: &TestParams,
    need_cnt: &[usize],
    mut states: Vec<State>,
    expected_numa_cnt: usize,
) {
    // rec: (state, iter, done_here)
    let ids: Vec<usize> = (0..params.vm_specs.len())
        .rev()
        .filter(|x| need_cnt[*x] > 0 && params.vm_specs[*x].numa_cnt == expected_numa_cnt)
        .collect();
    // let mut possible_states = BTreeSet::new();
    let mut rnd = Random::new(333);

    dbg!("START SEARCH!!!!!!!!!!!!!!!!!!!!!");

    let clamp_state = |state: State, iter: usize| -> State {
        if expected_numa_cnt == 1 {
            let mut cpu_div_mem = vec![];
            for i in iter..ids.len() {
                let spec = params.vm_specs[ids[i]];
                cpu_div_mem.push(Frac::new(spec.cpu as i64, spec.memory as i64))
            }
            cpu_div_mem.sort();
            assert!(!cpu_div_mem.is_empty());
            state.clamp_(cpu_div_mem[0], *cpu_div_mem.last_exn())
        } else {
            state
        }
    };

    for iter in 0..ids.len() {
        let mut next_states = vec![];
        states = states.into_iter().map(|s| clamp_state(s, iter)).collect();
        dbg!(iter, states.len());
        let mut need_more = Stats::default();
        for &id in ids[iter..].iter() {
            need_more.cpu += params.vm_specs[id].cpu * (need_cnt[id] as u32);
            need_more.memory += params.vm_specs[id].memory * (need_cnt[id] as u32);
        }
        states = states
            .into_iter()
            .filter(|s| s.sum_cpu() >= need_more.cpu && s.sum_memory() >= need_more.memory)
            .collect();
        dbg!("after removing", states.len());
        states.sort_by_key(|s| s.sum_cpu() + s.sum_memory());
        rnd.shuffle(&mut states);
        // states.reverse();
        states.truncate(500);
        let vm_type = ids[iter];

        let mut seen = HashSet::new();
        for start_state in states.into_iter() {
            RecursiveFunction3::new(|f, state: State, done: usize, random_splits: usize| {
                let more = need_cnt[vm_type] - done;
                if more == 0 {
                    next_states.push(state);
                } else {
                    let key = (state.clone(), done, random_splits);
                    if !seen.insert(key) {
                        return;
                    }
                    for pos in 0..state.counts.len() {
                        let (k, v) = state.counts[pos];
                        let vm_spec = params.vm_specs[vm_type];
                        if k.can_place(&vm_spec) {
                            let max_cnt_here = min(v, more);
                            for it in 0..2 {
                                let mut cnt_here = max_cnt_here;
                                if it == 1 {
                                    cnt_here = rnd.gen(1..cnt_here + 1);
                                }
                                if it == 1 && (random_splits >= 1 || done != 0) {
                                    continue;
                                }
                                let new_state = state.place_vms(pos, cnt_here, &vm_spec);
                                f.call(new_state, done + cnt_here, random_splits + it);
                            }
                        }
                    }
                }
            })
            .call(start_state, 0, 0);
        }

        states = next_states;
    }

    if expected_numa_cnt == 2 {
        let mut next_lvl = states.iter().map(|s| s.twice()).collect();
        rec_search_numa(params, need_cnt, next_lvl, 1);
    } else {
        dbg!(states.len());
        assert!(!states.is_empty());
    }
}

pub fn find_shuffling(params: &TestParams, need_cnt: &[usize]) -> bool {
    let mut sum_available_cpus = 0;
    let mut sum_available_mem = 0;
    for numa in params.numa.iter() {
        sum_available_cpus += numa.cpu;
        sum_available_mem += numa.memory;
    }
    sum_available_cpus *= params.total_machines() as u32;
    sum_available_mem *= params.total_machines() as u32;

    let mut need_cpu = 0;
    let mut need_mem = 0;
    for i in 0..need_cnt.len() {
        let vm_spec = params.vm_specs[i];
        need_cpu += vm_spec.cpu as usize * (need_cnt[i] * vm_spec.numa_cnt);
        need_mem += vm_spec.memory as usize * (need_cnt[i] * vm_spec.numa_cnt);
    }

    dbg!(sum_available_cpus, sum_available_mem);
    dbg!(need_cpu, need_mem);

    rec_search_numa(params, &need_cnt, vec![State::new(params)], 2);
    true
}

pub fn find_shuffling_io(test_case: usize, params: &TestParams) -> bool {
    let mut need_cnt = load_cnt(test_case, params);
    find_shuffling(params, &need_cnt)
}
