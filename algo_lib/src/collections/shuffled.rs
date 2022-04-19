use crate::misc::rand::Random;

pub trait ShuffledTrait {
    fn shuffled(&self, rnd: &mut Random) -> Self;
}

impl<T> ShuffledTrait for Vec<T>
where
    T: Clone + Ord,
{
    fn shuffled(&self, rnd: &mut Random) -> Self {
        let perm = rnd.gen_permutation(self.len());
        perm.into_iter().map(|id| self[id].clone()).collect()
    }
}
