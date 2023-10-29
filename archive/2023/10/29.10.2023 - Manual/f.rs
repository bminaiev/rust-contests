//{"name":"f","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""},{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"f"}}}

use algo_lib::collections::array_2d::Array2D;
use algo_lib::io::output::{output, set_global_output_to_stdout};
use algo_lib::io::task_io_settings::TaskIoType;
use algo_lib::io::task_runner::run_task;
use algo_lib::io::{input::Input, task_io_settings::TaskIoSettings};
use algo_lib::math::gcd::gcd;
use algo_lib::misc::rand::Random;
#[allow(unused)]
use algo_lib::{dbg, out, out_line};

fn good_ij(field: &Array2D<bool>, i: usize, j: usize) -> bool {
    let n = field.len();
    let m = field[0].len();
    let shifts = [(0, 1), (1, 0), (0, m - 1), (n - 1, 0)];
    let mut cnt = 0;
    for &(di, dj) in shifts.iter() {
        let ni = (i + di) % n;
        let nj = (j + dj) % m;
        if field[ni][nj] {
            cnt += 1;
        }
    }
    if cnt != 3 && field[i][j] {
        return false;
    }
    true
}

fn good_ij_cnt(field: &Array2D<bool>, i: usize, j: usize) -> usize {
    let n = field.len();
    let m = field[0].len();
    let shifts = [(0, 1), (1, 0), (0, m - 1), (n - 1, 0)];
    let mut cnt = 0;
    for &(di, dj) in shifts.iter() {
        let ni = (i + di) % n;
        let nj = (j + dj) % m;
        if field[ni][nj] {
            cnt += 1;
        }
    }
    cnt
}

fn good(field: &Array2D<bool>) -> bool {
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if !good_ij(field, i, j) {
                return false;
            }
        }
    }
    true
}

fn good_cnt(field: &Array2D<bool>) -> usize {
    let mut res = 0;
    for i in 0..field.len() {
        for j in 0..field[i].len() {
            if good_ij(field, i, j) {
                res += 1;
            }
        }
    }
    res
}

fn print(field: &Array2D<bool>) {
    out_line!(field.len(), field[0].len());
    for i in 0..field.len() {
        for j in 0..field[0].len() {
            let x = field[i][j];
            if x {
                out!("#");
            } else {
                out!(".");
            }
        }
        out_line!()
    }
}

fn stress(np: usize, nq: usize) {
    if np == 7 && nq == 9 {
        let mut base = Array2D::new(true, 18, 6);
        base[0][5] = false;
        base[1][1] = false;
        base[1][2] = false;
        base[2][4] = false;
        for i in 1..6 {
            for row in i * 3..i * 3 + 3 {
                for c in 0..6 {
                    base[row][c] = base[row - 3][(c + 5) % 6];
                }
            }
        }
        assert!(good(&base));
        print(&base);
        return;
    }
    let mut seen = Array2D::new(false, 11, 11);
    for n in 1..10 {
        for m in n..10 {
            if n * m <= 20 {
                for mask in 0i32..(1 << (n * m)) {
                    let mut field = Array2D::new(false, n, m);
                    for i in 0..n * m {
                        let x = i / m;
                        let y = i % m;
                        if ((1 << i) & mask) != 0 {
                            field[x][y] = true;
                        }
                    }
                    let mut p = mask.count_ones() as usize;
                    let mut q = n * m;
                    // dbg!(mask, p, q, n, m);
                    let g = gcd(p, q);
                    p /= g;
                    q /= g;
                    if (p != np || q != nq) && nq != 0 {
                        continue;
                    }
                    if !good(&field) {
                        continue;
                    }
                    if q <= 10 && !seen[p][q] {
                        seen[p][q] = true;
                        if nq != 0 {
                            print(&field);
                            return;
                        } else {
                            dbg!(p, q, n, m);
                        }
                    }
                }
            }
        }
    }
    // dbg!("START!");
    for gl in [760, 6314, 35994, 488759, 1486983, 2344386] {
        let mut rnd = Random::new(gl);
        let n = rnd.gen(1..20);
        let m = rnd.gen(1..20);
        let mut field = Array2D::new(false, n, m);
        for _ in 0..10 {
            rnd.gen_double();
        }
        let p = rnd.gen_double();
        for i in 0..n {
            for j in 0..m {
                field[i][j] = rnd.gen_double() < p;
            }
        }
        loop {
            let mut ok = true;
            let perm = rnd.gen_permutation(n * m);
            for ix in perm.iter() {
                let i = ix / m;
                let j = ix % m;
                if !good_ij(&field, i, j) {
                    field[i][j] = false;
                    ok = false;
                }
            }
            if ok {
                let mut p = 0;
                let mut q = n * m;
                for i in 0..n {
                    for j in 0..m {
                        if field[i][j] {
                            p += 1;
                        }
                    }
                }
                let g = gcd(p, q);
                p /= g;
                q /= g;
                if (p != np || q != nq) && nq != 0 {
                    break;
                }
                if q <= 10 && !seen[p][q] {
                    seen[p][q] = true;
                    if nq != 0 {
                        print(&field);
                        return;
                    } else {
                        dbg!(p, q, n, m, gl);
                    }
                }
                break;
            }
        }
    }
    // dbg!("####");
    let mut good_seed = vec![];
    // [361, 782, 1143, 1410, 1554, 1868, 3109, 4627, 50500, 106284]
    for gl in [34, 98, 382, 438, 444, 518, 788, 1190, 9653, 211186] {
        let mut rnd = Random::new(gl);
        let mut n = rnd.gen(1..50);
        let mut m = rnd.gen(1..50);
        if rnd.gen_bool() {
            n = 1;
            m = rnd.gen(1..1000);
        }
        let mut field = Array2D::new(false, n, m);
        for _ in 0..10 {
            rnd.gen_double();
        }
        let p = rnd.gen_double();
        for i in 0..n {
            for j in 0..m {
                field[i][j] = rnd.gen_double() < p;
            }
        }
        loop {
            let mut ok = true;
            let perm = rnd.gen_permutation(n * m);
            for ix in perm.iter() {
                let i = ix / m;
                let j = ix % m;
                if !good_ij(&field, i, j) {
                    field[i][j] = false;
                    ok = false;
                }
            }
            if ok {
                let mut p = 0;
                let mut q = n * m;
                for i in 0..n {
                    for j in 0..m {
                        if field[i][j] {
                            p += 1;
                        }
                    }
                }
                let g = gcd(p, q);
                p /= g;
                q /= g;
                if (p != np || q != nq) && nq != 0 {
                    break;
                }
                if q <= 10 && !seen[p][q] {
                    seen[p][q] = true;
                    if nq != 0 {
                        print(&field);
                        return;
                    } else {
                        dbg!(p, q, n, m, gl);
                        good_seed.push(gl);
                        dbg!(good_seed);
                    }
                }
                break;
            }
        }
    }
    // dbg!("$#%#$%");
    good_seed.clear();

    for gl in [18034582] {
        if gl % 100000 == 0 {
            dbg!(gl);
        }
        let mut rnd = Random::new(gl);
        let sz = rnd.gen(2..100);
        let mut n = rnd.gen(1..sz);
        let mut m = rnd.gen(1..sz);
        if rnd.gen_bool() {
            n = 7;
            // m = 7;
        }
        let mut field = Array2D::new(false, n, m);
        for _ in 0..10 {
            rnd.gen_double();
        }
        let p = rnd.gen_double();
        for i in 0..n {
            for j in 0..m {
                field[i][j] = rnd.gen_double() < p;
            }
        }
        loop {
            let mut ok = true;
            let mut fail = false;
            let perm = rnd.gen_permutation(n * m);
            for ix in perm.iter() {
                let i = ix / m;
                let j = ix % m;
                if !good_ij(&field, i, j) {
                    if good_ij_cnt(&field, i, j) == 4 {
                        fail = true
                    }
                    let shifts = [(0, 1), (1, 0), (0, m - 1), (n - 1, 0)];
                    let (di, dj) = shifts[rnd.gen(0..4)];
                    let ni = (i + di) % n;
                    let nj = (j + dj) % m;
                    if !field[ni][nj] {
                        field[ni][nj] = true;
                    }
                    ok = false;
                }
            }
            if fail {
                break;
            }
            if ok {
                let mut p = 0;
                let mut q = n * m;
                for i in 0..n {
                    for j in 0..m {
                        if field[i][j] {
                            p += 1;
                        }
                    }
                }
                let g = gcd(p, q);
                p /= g;
                q /= g;
                if (p != np || q != nq) && nq != 0 {
                    break;
                }
                if q <= 10 && !seen[p][q] {
                    seen[p][q] = true;
                    if nq != 0 {
                        print(&field);
                        return;
                    } else {
                        dbg!(p, q, n, m, gl);
                        good_seed.push(gl);
                        dbg!(good_seed);
                    }
                }
                break;
            }
        }
    }

    for gl in Random::new_time_seed().gen_u64().. {
        if gl % 1000 == 0 {
            dbg!(gl);
        }
        let mut rnd = Random::new(gl);
        let sz = rnd.gen(2..30);
        let n = sz;
        let m = rnd.gen(2..30);

        let mut field = Array2D::new(false, n, m);
        for _ in 0..10 {
            rnd.gen_double();
        }
        let p = rnd.gen_double();
        for i in 0..n {
            for j in 0..m {
                field[i][j] = rnd.gen_double() < p;
            }
        }
        loop {
            let mut cur_good = good_cnt(&field);
            let perm = rnd.gen_permutation(n * m);
            let mut found = false;
            for need in [true, false].iter() {
                for &p in perm.iter() {
                    let x = p / m;
                    let y = p % m;
                    field[x][y] = !field[x][y];
                    let new_good = good_cnt(&field);
                    if new_good > cur_good && (!need || field[x][y]) {
                        found = true;
                        cur_good = new_good;
                    } else {
                        field[x][y] = !field[x][y];
                    }
                }
            }
            if !found {
                break;
            }
        }

        // dbg!(good_cnt(&field));
        let nfield = field.clone();
        if good(&nfield) {
            let n = nfield.len();
            let mut p = 0;
            let mut q = n * m;
            for i in 0..n {
                for j in 0..m {
                    if nfield[i][j] {
                        p += 1;
                    }
                }
            }
            let g = gcd(p, q);
            p /= g;
            q /= g;
            if (p != np || q != nq) && nq != 0 {
                break;
            }
            if q <= 10 && !seen[p][q] {
                seen[p][q] = true;
                if nq != 0 {
                    print(&nfield);
                    return;
                } else {
                    dbg!(p, q, n, m, gl);
                    good_seed.push(gl);
                    dbg!(good_seed);
                    unreachable!();
                }
            }
        }
    }

    if nq == 0 {
        for p in 0..=10 {
            for q in p + 1..=10 {
                if gcd(p, q) == 1 {
                    if !seen[p][q] {
                        let coef = p as f64 / q as f64;
                        if coef <= 4.0 / 5.0 {
                            dbg!(p, q, "???");
                        }
                    }
                }
            }
        }
    }
    if nq != 0 {
        let need = np as f64 / nq as f64;
        assert!(need > 4.0 / 5.0 || (np == 7 && nq == 9));
    }
    out_line!("-1 -1");
}

fn solve(input: &mut Input, _test_case: usize) {
    let np = input.usize();
    let nq = input.usize();
    stress(np, nq);
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input, 1);
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
    tester::run_single_test("3");
    // tester::run_stress(stress);
    // set_global_output_to_stdout();
    // stress(0, 0);
    // tester::run_locally();
    // tester::run_with_last_downloaded_file();
}
//END MAIN
