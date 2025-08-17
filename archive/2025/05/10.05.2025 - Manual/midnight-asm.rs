//{"name":"midnight-asm","group":"Manual","url":"","interactive":false,"timeLimit":2000,"tests":[{"input":"","output":""}],"testType":"single","input":{"type":"stdin","fileName":null,"pattern":null},"output":{"type":"stdout","fileName":null,"pattern":null},"languages":{"java":{"taskClass":"midnight-asm"}}}

#[allow(unused)]
use algo_lib::dbg;
use algo_lib::io::input::Input;
use algo_lib::io::output::Output;

fn count_strings(total_a: usize, total_b: usize, total_c: usize) -> u32 {
    // dp[d][j][k] will hold the number of ways to reach a state with:
    //   - d: the current layer (d A's used) -- we use two layers (current and next)
    //   - j: number of B's used (0..=total_b)
    //   - k: number of C's used (0..=total_c)
    // and ending with a specific letter: index 0 = A, 1 = B, 2 = C.
    //
    // We allocate two layers: dp_current for the current number of A's,
    // and dp_next for the next layer (when we add an A).
    let mut dp_current = vec![vec![[0u32; 3]; total_c + 1]; total_b + 1];
    let mut dp_next = vec![vec![[0u32; 3]; total_c + 1]; total_b + 1];

    // The initial state is "none". From it we can start by adding any letter.
    // We handle it manually:
    // If we add A (provided total_a > 0), that uses one A so it goes to the next layer.
    if total_a > 0 {
        dp_next[0][0][0] = 1;
    }
    // If we add B (provided total_b > 0), that stays in the same layer (0 A's used)
    if total_b > 0 {
        dp_current[1][0][1] = 1;
    }
    // If we add C (provided total_c > 0), that stays in the same layer.
    if total_c > 0 {
        dp_current[0][1][2] = 1;
    }

    // The result will be the total ways when we have used all letters.
    // We process layer by layer (i.e. by the number of A's used).
    let mut result: u32 = 0;

    // Loop for each layer: `curr_a` is the number of A's used in dp_current.
    for curr_a in 0..=total_a {
        // Process transitions within the current layer.
        // We iterate in order so that transitions which add a B or C (which increase j or k)
        // are computed in a monotonic fashion.
        for j in 0..=total_b {
            for k in 0..=total_c {
                // For each possible last letter in the state.
                for letter in 0..3 {
                    let ways = dp_current[j][k][letter];
                    if ways == 0 {
                        continue;
                    }
                    // Transition 1: Add letter A if possible.
                    // Only allowed if the last letter is not A and we still have A's left.
                    if letter != 0 && curr_a < total_a {
                        // Adding A increases the count of A's (i.e. moves to the next layer),
                        // while B and C counts remain the same. The new last letter becomes A.
                        dp_next[j][k][0] = dp_next[j][k][0].wrapping_add(ways);
                    }
                    // Transition 2: Add letter B if possible.
                    // Can add B if the last letter is not B and we haven't used all B's.
                    if letter != 1 && j < total_b {
                        dp_current[j + 1][k][1] = dp_current[j + 1][k][1].wrapping_add(ways);
                    }
                    // Transition 3: Add letter C if possible.
                    if letter != 2 && k < total_c {
                        dp_current[j][k + 1][2] = dp_current[j][k + 1][2].wrapping_add(ways);
                    }
                }
            }
        }

        // If we have used all A's in this layer, then we are at the final layer.
        if curr_a == total_a {
            // The final answer is the sum over all states with exactly total_b B's and total_c C's,
            // regardless of the last letter.
            result = dp_current[total_b][total_c][0]
                .wrapping_add(dp_current[total_b][total_c][1])
                .wrapping_add(dp_current[total_b][total_c][2]);
        }

        // Prepare for the next layer: move dp_next (transitions that added A)
        // into dp_current for the next iteration, and clear dp_next.
        if curr_a < total_a {
            // We swap dp_next into a new dp_current.
            let mut new_dp = vec![vec![[0u32; 3]; total_c + 1]; total_b + 1];
            std::mem::swap(&mut dp_next, &mut new_dp);
            // Clear dp_next (which is now in new_dp) so that it is zeroed.
            for j in 0..=total_b {
                for k in 0..=total_c {
                    dp_next[j][k] = [0u32; 3];
                }
            }
            dp_current = new_dp;
        }
    }

    result
}

fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    let a = input.usize();
    let b = input.usize();
    let c = input.usize();
    dbg!(a, b, c);

    let result = count_strings(a, b, c);
    out.print(result);
}

fn stress() {
    for tc in 1..=10 {
        dbg!(tc);
        let mut input = Input::new_file(format!("./tasks/midnight-asm/tests/{tc:02}"));
        let mut output = Output::new_file(format!("./tasks/midnight-asm/out/{tc:02}.out"));
        solve(&mut input, &mut output, tc);
        output.flush();
    }
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    solve(&mut input, &mut output, 1);
    output.flush();
    true
}

//START MAIN

fn main() {
    const PROBLEM_NAME: &str = "midnight-asm";
    use algo_lib::tester::helper::*;

    // run_tests(PROBLEM_NAME, run);
    // run_single_test(PROBLEM_NAME, run, "1");
    run_stress(stress);
    // run_locally(run);
}
//END MAIN
