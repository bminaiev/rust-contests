use crate::collections::last_exn::LastExn;
use crate::math::modulo::Mod9;
use crate::misc::num_traits::{ConvI32, Number};
use std::ops::{Index, Range};

pub struct HashContext<M>
where
    M: Number,
{
    pub powers: Vec<M>,
    #[allow(unused)]
    multiplier: M,
}

pub struct HashString<'a, Hash, S>
where
    Hash: Number,
    S: Number,
{
    string: Vec<S>,
    prefix_hash: Vec<Hash>,
    ctx: &'a HashContext<Hash>,
}

impl<'a, Hash, S> Index<usize> for HashString<'a, Hash, S>
where
    Hash: Number,
    S: Number,
{
    type Output = S;

    fn index(&self, index: usize) -> &Self::Output {
        &self.string[index]
    }
}

impl<'a, Hash, S> HashString<'a, Hash, S>
where
    Hash: Number,
    S: Number,
{
    pub fn calc_hash(&self, range: Range<usize>) -> Hash {
        self.prefix_hash[range.end] - self.prefix_hash[range.start] * self.ctx.powers[range.len()]
    }
}

pub fn default_hash_context(max_len: usize) -> HashContext<Mod9> {
    HashContext::new(max_len, Mod9::from_i32(239017))
}

impl<Hash> HashContext<Hash>
where
    Hash: Number,
{
    pub fn new(max_len: usize, multiplier: Hash) -> Self {
        let mut powers = Vec::with_capacity(max_len + 1);
        powers.push(Hash::ONE);
        for i in 1..=max_len {
            powers.push(powers[i - 1] * multiplier);
        }
        Self { powers, multiplier }
    }

    pub fn make_string<S>(&self, s: &[S]) -> HashString<Hash, S>
    where
        S: Number,
    {
        let string = s.to_vec();
        let mut prefix_hash = Vec::with_capacity(s.len() + 1);
        prefix_hash.push(Hash::ZERO);
        for &symbol in s.iter() {
            let next_hash =
                *prefix_hash.last_exn() * self.multiplier + (Hash::from_i32(symbol.to_i32()));
            prefix_hash.push(next_hash);
        }
        HashString {
            string,
            prefix_hash,
            ctx: self,
        }
    }
}
