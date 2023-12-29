fn solve(input: &mut Input, out: &mut Output, _test_case: usize) {
    $CARET
}

pub(crate) fn run(mut input: Input, mut output: Output) -> bool {
    $INVOKE
    output.flush();
    true
}
