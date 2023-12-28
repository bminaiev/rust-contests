let mut i = 1usize;
while input.peek().is_some() {
    solve(&mut input, &mut output, i);
    i += 1;
}