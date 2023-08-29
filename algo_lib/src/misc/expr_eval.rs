fn process_op(st: &mut Vec<i64>, ops: &mut Vec<u8>) {
    let op = ops.pop().unwrap();
    let r = st.pop().unwrap();
    let l = st.pop().unwrap();
    let res = match op {
        b'+' => l + r,
        b'-' => l - r,
        b'*' => l * r,
        b'/' => l / r,
        _ => panic!("Unknown op {}", op),
    };
    st.push(res);
}

fn is_op(c: u8) -> bool {
    matches!(c, b'+' | b'-' | b'*' | b'/')
}

fn priority(c: u8) -> i32 {
    match c {
        b'+' | b'-' => 1,
        b'*' | b'/' => 2,
        b'(' => -1,
        _ => panic!("Unknown op {}", c as char),
    }
}

pub fn eval_expression(s: &[u8]) -> i64 {
    let mut st = vec![];
    let mut ops = vec![];
    let mut iter = 0;
    while iter < s.len() {
        let c = s[iter];
        if c == b'(' {
            ops.push(c);
        } else if c == b')' {
            while *ops.last().unwrap() != b'(' {
                process_op(&mut st, &mut ops)
            }
            ops.pop();
        } else if is_op(c) {
            while !ops.is_empty() && priority(*ops.last().unwrap()) >= priority(c) {
                process_op(&mut st, &mut ops);
            }
            ops.push(c);
        } else {
            let mut num = 0;
            while iter < s.len() && s[iter].is_ascii_alphanumeric() {
                num = num * 10 + (s[iter] - b'0') as i64;
                iter += 1;
            }
            iter -= 1;
            st.push(num);
        }
        iter += 1;
    }

    while !ops.is_empty() {
        process_op(&mut st, &mut ops);
    }

    st[0]
}
