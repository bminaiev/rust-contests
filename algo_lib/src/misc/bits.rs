pub fn index_of_highest_set_bit(value: usize) -> usize {
    const BITS_PER_BYTE: usize = 8;
    std::mem::size_of::<usize>() * BITS_PER_BYTE - (value.leading_zeros() as usize) - 1
}
