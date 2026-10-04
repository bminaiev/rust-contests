use algo_lib::collections::permutation::Permutation;
#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

#[derive(Clone)]
struct Node {
    sum: i64,
    root: i64,
    children: Vec<Node>,
}

impl Node {
    fn print(&self, out: &mut Vec<i64>) {
        let mut all = vec![self];
        while !all.is_empty() {
            let mut next = vec![];
            for n in all.iter() {
                out.push(n.root);
                for c in n.children.iter() {
                    next.push(c);
                }
            }
            all = next;
        }
    }

    fn is_good(&self) -> bool {
        if self.children.is_empty() {
            true
        } else {
            for c in self.children.iter() {
                if !c.is_good() {
                    return false;
                }
            }
            self.root == (self.children[0].sum - self.children[1].sum).abs()
        }
    }
}

fn build(n: usize) -> Node {
    let mx = 1 << n;
    let mut all_nodes = vec![];
    for root in mx / 2..mx {
        all_nodes.push(Node {
            sum: root,
            root,
            children: vec![],
        });
    }
    while all_nodes.len() > 1 {
        all_nodes.sort_by_key(|n| n.sum);
        let mut next_nodes = vec![];
        for i in 0..all_nodes.len() / 2 {
            let n1 = all_nodes[i].clone();
            let n2 = all_nodes[all_nodes.len() - 1 - i].clone();
            let root = (n1.sum - n2.sum).abs();
            next_nodes.push(Node {
                sum: root + n1.sum + n2.sum,
                root,
                children: vec![n1, n2],
            });
        }
        all_nodes = next_nodes;
    }
    all_nodes[0].clone()
}

fn solve(input: &mut Input, out: &mut Output) {
    let n = input.usize();
    let root = build(n);
    assert!(root.is_good());
    let mut res = vec![];
    root.print(&mut res);
    out.println(res);
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output);
    output.flush();
    true
}

#[cfg(feature = "local")]
fn main() {
    const PROBLEM_NAME: &str = "f_two_unbalanced_subtrees";
    #[allow(unused)]
    use algo_lib::misc::dragon::run_dragon;
    use algo_lib::tester::helper::*;

    run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    // run_stress(stress);
    // run_locally(run);
    // run_dragon(solve, "W", 1..2);
}
//END MAIN

#[cfg(not(feature = "local"))]
fn main() {
    let input = algo_lib::io::input::Input::new_stdin();
    let mut output = algo_lib::io::output::Output::new_stdout();
    run(input, output);
}
