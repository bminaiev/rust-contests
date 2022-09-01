use crate::{
    graph::dsu_with_rollbacks::{DsuNodeTrait, DsuWithRollbacks},
    misc::group_by::GroupByTrait,
};

#[derive(Clone, Copy, Debug)]
struct EdgeChange {
    time: usize,
    fr: usize,
    to: usize,
    delta: i32,
}

// TODO: use [u32] instead of [usize] for performance!
#[derive(Clone, Copy, Debug)]
struct EdgePresent {
    fr: usize,
    to: usize,
    time_fr_incl: usize,
    time_to_excl: usize,
}

pub struct DynamicConnectivityOffline<T: DsuNodeTrait, Q: Copy + Default> {
    dsu: DsuWithRollbacks<T>,
    queries: Vec<Q>,
    events: Vec<EdgeChange>,
}

impl<T: DsuNodeTrait, Q: Copy + Default> DynamicConnectivityOffline<T, Q> {
    pub fn new(n: usize) -> Self {
        Self {
            dsu: DsuWithRollbacks::new(n),
            queries: Vec::new(),
            events: Vec::new(),
        }
    }

    pub fn add_query(&mut self, query: Q) {
        self.queries.push(query);
    }

    pub fn add_edge(&mut self, fr: usize, to: usize) {
        assert!(fr < self.dsu.len());
        assert!(to < self.dsu.len());
        self.events.push(EdgeChange {
            time: self.queries.len(),
            fr,
            to,
            delta: 1,
        });
    }

    pub fn remove_edge(&mut self, fr: usize, to: usize) {
        assert!(fr < self.dsu.len());
        assert!(to < self.dsu.len());
        self.events.push(EdgeChange {
            time: self.queries.len(),
            fr,
            to,
            delta: -1,
        });
    }

    fn gen_edge_events(&mut self) -> Vec<EdgePresent> {
        let mut res = vec![];
        self.events.sort_by_key(|e| (e.fr, e.to, e.time, -e.delta));
        for group in self
            .events
            .group_by_(|e1, e2| e1.fr == e2.fr && e1.to == e2.to)
        {
            let mut balance = 0;
            let mut cur_start = 0;
            for ev in group.iter() {
                balance += ev.delta;
                assert!(balance >= 0);
                if balance == 1 && ev.delta == 1 {
                    cur_start = ev.time;
                }
                if balance == 0 {
                    res.push(EdgePresent {
                        fr: ev.fr,
                        to: ev.to,
                        time_fr_incl: cur_start,
                        time_to_excl: ev.time,
                    })
                }
            }
            if balance > 0 {
                res.push(EdgePresent {
                    fr: group[0].fr,
                    to: group[0].to,
                    time_fr_incl: cur_start,
                    time_to_excl: self.queries.len(),
                })
            }
        }
        res
    }

    fn run_rec<F>(&mut self, l: usize, r: usize, events: Vec<EdgePresent>, callback: &mut F)
    where
        F: FnMut(&Q, &DsuWithRollbacks<T>),
    {
        let mut ev_left = vec![];
        let mut ev_right = vec![];

        let mid = (l + r) >> 1;

        let cur_time = self.dsu.get_current_time();
        for ev in events.iter() {
            if ev.time_fr_incl <= l && ev.time_to_excl >= r {
                self.dsu.unite(ev.fr, ev.to);
            } else {
                if ev.time_fr_incl < mid {
                    ev_left.push(*ev);
                }
                if ev.time_to_excl > mid {
                    ev_right.push(*ev);
                }
            }
        }
        if l + 1 == r {
            callback(&self.queries[l], &self.dsu);
        } else {
            self.run_rec(l, mid, ev_left, callback);
            self.run_rec(mid, r, ev_right, callback);
        }
        self.dsu.rollback(cur_time);
    }

    pub fn run<F>(&mut self, callback: &mut F)
    where
        F: FnMut(&Q, &DsuWithRollbacks<T>),
    {
        let events = self.gen_edge_events();
        self.run_rec(0, self.queries.len(), events, callback);
    }

    pub fn get_dsu_mut(&mut self) -> &mut DsuWithRollbacks<T> {
        &mut self.dsu
    }
}
