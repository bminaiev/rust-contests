//{"name":"A. AMPPZ in the times of disease","group":"Yandex - Stage 8: Grand Prix of Poland","url":"https://official.contest.yandex.ru/opencupXXII/contest/32038/problems/","interactive":false,"timeLimit":10000,"tests":[{"input":"3\n3 2\n0 0\n0 1\n0 3\n4 4\n0 0\n0 1\n1 0\n1 1\n8 3\n3 1\n4 1\n1 6\n2 6\n6 5\n6 7\n3 2\n4 2\n","output":"1 1 2\n4 1 3 2\n2 2 1 1 3 3 2 2\n"}],"testType":"multiNumber","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"AAMPPZInTheTimesOfDisease"}}}

use algo_lib::geometry::point::PointT;
use algo_lib::graph::dsu::Dsu;
use algo_lib::io::input::Input;
use algo_lib::io::output::output;
use algo_lib::misc::min_max::{position_of_min_by, UpdateMinMax};
use algo_lib::{dbg, out, out_line};


type Point = PointT<i64>;

struct Group {
    any_point: Point,
    group_id: usize,
    min_dist_to_another_group: i64,
}

fn solve_task(pts: &[Point], k: usize) -> Vec<usize> {
    let n = pts.len();

    let mut dsu = Dsu::new(n);
    let mut groups: Vec<_> = (0..k)
        .map(|id| Group {
            any_point: pts[id],
            group_id: dsu.get(id),
            min_dist_to_another_group: i64::MAX,
        })
        .collect();
    let update_min_dist = |groups: &mut Vec<Group>, idx: usize| {
        groups[idx].min_dist_to_another_group = groups
            .iter()
            .enumerate()
            .map(|(another_idx, group)| {
                if another_idx == idx {
                    i64::MAX
                } else {
                    groups[idx].any_point.dist2(&group.any_point)
                }
            })
            .min()
            .unwrap();
    };
    for i in 0..k {
        update_min_dist(&mut groups, i);
    }

    for i in k..n {
        groups.push(Group {
            any_point: pts[i],
            group_id: dsu.get(i),
            min_dist_to_another_group: i64::MAX,
        });
        for j in 0..k {
            let d2 = groups[j].any_point.dist2(&groups[k].any_point);
            groups[j].min_dist_to_another_group.update_min(d2);
            groups[k].min_dist_to_another_group.update_min(d2);
        }

        let min_pos = position_of_min_by(groups.len(), |idx| groups[idx].min_dist_to_another_group);
        let min_pos2 = position_of_min_by(groups.len(), |idx| {
            if idx == min_pos {
                i64::MAX
            } else {
                groups[min_pos].any_point.dist2(&groups[idx].any_point)
            }
        });
        assert_ne!(min_pos, min_pos2);
        assert!(dsu.unite(groups[min_pos].group_id, groups[min_pos2].group_id));
        groups.swap_remove(min_pos2);
        assert_ne!(min_pos, k);
        update_min_dist(&mut groups, min_pos);
        assert_eq!(groups.len(), k);
    }
    let comps = dsu.calc_components();
    assert_eq!(comps.len(), k);
    let mut res = vec![0; n];
    for (id, comp) in comps.iter().enumerate() {
        for &v in comp.iter() {
            res[v] = id + 1;
        }
    }
    res
}

fn solve(input: &mut Input, _test_case: usize) {
    let n = input.usize();
    let k = input.usize();
    let pts: Vec<_> = (0..n)
        .map(|_| {
            let x = input.i64();
            let y = input.i64();
            Point::new(x, y)
        })
        .collect();
    let res = solve_task(&pts, k);
    out_line!(res);
}

pub(crate) fn run(mut input: Input) -> bool {
    let t = input.read();
    for i in 0usize..t {
        solve(&mut input, i + 1);
    }
    output().flush();
    input.skip_whitespace();
    !input.peek().is_some()
}

//START MAIN
mod tester;

fn main() {
    tester::run_tests();
}
//END MAIN
