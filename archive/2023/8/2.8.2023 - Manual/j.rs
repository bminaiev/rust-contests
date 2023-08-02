//{"name":"j","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"j"}}}

use std::cmp::{min, Reverse};
use std::collections::{BTreeSet, HashSet};

use algo_lib::graph::dsu::Dsu;
use algo_lib::io::output::output;
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::misc::group_by::GroupByTrait;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

trait Interactor {
    fn n(&self) -> usize;
    fn ask(&mut self, a: Vec<usize>) -> Vec<bool>;
    fn response(&mut self, res: Option<usize>);
}

struct MyInteractor {
    a: Vec<usize>,
    queries: usize,
    ok: bool,
}

impl MyInteractor {
    pub fn new(a: Vec<usize>) -> Self {
        Self {
            a,
            queries: 0,
            ok: false,
        }
    }
}

impl Interactor for MyInteractor {
    fn n(&self) -> usize {
        self.a.len()
    }

    fn ask(&mut self, query: Vec<usize>) -> Vec<bool> {
        self.queries += 1;
        assert!(self.queries <= 3);
        assert_eq!(query.len(), self.a.len());
        let mut res = vec![false; query.len()];
        for i in 0..query.len() {
            res[i] = self.a[i] == self.a[query[i]];
        }
        res
    }

    fn response(&mut self, res: Option<usize>) {
        let mut sorted = self.a.clone();
        sorted.sort();
        let mut correct_res = None;
        for w in sorted.group_by_(|&x, &y| x == y) {
            if w.len() * 2 > self.a.len() {
                correct_res = Some(w[0]);
                // for i in 0..self.a.len() {
                //     if self.a[i] == w[0] {
                //         dbg!(i);
                //     }
                // }
                break;
            }
        }
        if res.is_none() && correct_res.is_none() {
            self.ok = true;
        } else {
            if let Some(res) = res {
                if Some(self.a[res]) == correct_res {
                    self.ok = true;
                    // dbg!("OK!");
                }
            }
            if !self.ok {
                dbg!("expected", correct_res, "got", res);
            }
        }
    }
}

fn solve_inter(inter: &mut impl Interactor) {
    let n = inter.n();
    let mut rnd = Random::new(87788);
    let mut dsu = Dsu::new(n);
    let mut known_diff: BTreeSet<(usize, usize)> = BTreeSet::new();
    for it in 0..3 {
        let mut a = vec![0; n];
        let mut asking = HashSet::new();
        let mut is_bad_query = |i: usize, j: usize, dsu: &mut Dsu| -> bool {
            if dsu.get(i) == dsu.get(j) {
                return true;
            }
            let x = dsu.get(i);
            let y = dsu.get(j);
            if known_diff.contains(&(x, y)) {
                return true;
            }
            if asking.contains(&(x, y)) {
                return true;
            }
            asking.insert((x, y));
            asking.insert((y, x));
            false
        };

        if it == 2 {
            let mut comps = dsu.calc_components();
            comps.sort_by_key(|c| Reverse(c.len()));
            // dbg!(comps);
            for from_c in comps.iter().rev() {
                for &i in from_c.iter() {
                    let mut check = 0;
                    for c in comps.iter() {
                        check += 1;
                        if check > 100 {
                            break;
                        }
                        if !is_bad_query(i, c[0], &mut dsu) {
                            a[i] = c[0];
                            break;
                        }
                    }
                }
            }
        } else if it == 0 {
            let mut perm = (0..n).collect::<Vec<_>>();
            rnd.shuffle(&mut perm);
            for i in 0..n {
                a[perm[i]] = perm[(i + 1) % n];
            }
        } else {
            // let cnt_roots = n; //1 + (n as f64).sqrt() as usize;
            // let mut roots = vec![];
            // for _ in 0..cnt_roots {
            //     roots.push(rnd.gen(0..n));
            // }
            let mut comps = dsu.calc_components();
            comps.sort_by_key(|c| Reverse(c.len()));
            for i in 0..n {
                // let mut ok = false;
                // for _tries in 0..100 {
                //     a[i] = roots[rnd.gen(0..cnt_roots)];
                //     if !is_bad_query(i, a[i], &mut dsu) {
                //         ok = true;
                //         break;
                //     }
                // }
                // if !ok {
                let mut ok = false;
                if dsu.get(i) == i {
                    let mut check = 0;
                    for c in comps.iter() {
                        check += 1;
                        if check > 10 {
                            break;
                        }
                        if !is_bad_query(i, c[0], &mut dsu) {
                            a[i] = c[0];
                            ok = true;
                            break;
                        }
                    }
                }
                if !ok {
                    for _tries in 0..100 {
                        a[i] = rnd.gen(0..n);
                        if !is_bad_query(i, a[i], &mut dsu) {
                            break;
                        }
                    }
                }
                // }
            }
        }
        let res = inter.ask(a.clone());
        for i in 0..n {
            if res[i] {
                dsu.unite(i, a[i]);
            } else {
                let x = dsu.get(i);
                let y = dsu.get(a[i]);
                known_diff.insert((x, y));
                known_diff.insert((y, x));
            }
        }
        let mut new_diff = BTreeSet::new();
        for (x, y) in known_diff.into_iter() {
            new_diff.insert((dsu.get(x), dsu.get(y)));
        }
        known_diff = new_diff;
    }
    let comps = dsu.calc_components();
    // dbg!(comps);
    for c in comps.iter() {
        if c.len() * 2 > n {
            inter.response(Some(c[0]));
            return;
        }
    }
    inter.response(None);
}

fn stress() {
    let mut cnt_fails = 0;
    for it_gl in 34125.. {
        if it_gl % 10000 == 0 {
            dbg!(it_gl);
        }
        let mut rnd = Random::new(it_gl);
        let n = rnd.gen(10..50);
        let mut a = vec![0; n];
        let half = n / 2 + rnd.gen(0..2);
        let mut sizes = vec![half];
        let mut more = n - half;
        let cap = rnd.gen(2..n / 2);
        while more > 0 {
            let sz = rnd.gen(1..min(cap, more + 1));
            sizes.push(sz);
            more -= sz;
        }
        let mut it = 0;
        let mut id = 0;
        for &sz in sizes.iter() {
            for _ in 0..sz {
                a[it] = id;
                it += 1;
            }
            id += 1;
        }
        rnd.shuffle(&mut a);
        // let max_n = rnd.gen(2..n + 2);
        // for i in 0..n {
        //     a[i] = rnd.gen(1..max_n);
        // }
        let mut interactor = MyInteractor::new(a.clone());
        solve_inter(&mut interactor);
        if !interactor.ok {
            cnt_fails += 1;
            dbg!("FAIL!", n, cnt_fails, it_gl / cnt_fails, it_gl);
            dbg!(sizes);
            // panic!();
        }
    }
}

struct RealInteractor<'a> {
    input: &'a mut Input,
    n: usize,
}

impl<'a> RealInteractor<'a> {
    pub fn new(input: &'a mut Input) -> Self {
        let n = input.usize();
        Self { input, n }
    }
}

impl<'a> Interactor for RealInteractor<'a> {
    fn n(&self) -> usize {
        self.n
    }

    fn ask(&mut self, a: Vec<usize>) -> Vec<bool> {
        out!("?");
        for &x in a.iter() {
            out!("", x + 1);
        }
        out_line!();
        output().flush();
        let s = self.input.string();
        s.iter().map(|x| *x == b'1').collect()
    }

    fn response(&mut self, res: Option<usize>) {
        if let Some(res) = res {
            out_line!("!", res + 1);
        } else {
            out_line!("! -1");
        }
        output().flush();
    }
}

fn solve(input: &mut Input, _test_case: usize) {
    let mut interactor = RealInteractor::new(input);
    solve_inter(&mut interactor);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    true
}

#[allow(unused)]
pub fn submit() -> bool {
    let io = TaskIoSettings {
        is_interactive: false,
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
