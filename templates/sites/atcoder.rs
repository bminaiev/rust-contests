fn solve(input: &mut Input) {
    $CARET
}

pub(crate) fn run(mut input: Input) -> bool {
    solve(&mut input);
    output().flush();
    true
}
