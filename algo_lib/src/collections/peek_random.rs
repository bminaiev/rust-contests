use crate::misc::rand::Random;

pub trait PeekRandom<T> {
    fn peek_random(&self, rnd: &mut Random) -> Option<&T>;
    fn peek_random_exn(&self, rnd: &mut Random) -> &T;
}

impl<T> PeekRandom<T> for &[T] {
    fn peek_random(&self, rnd: &mut Random) -> Option<&T> {
        if self.is_empty() {
            return None;
        } else {
            Some(&self[rnd.gen_index(self)])
        }
    }

    fn peek_random_exn(&self, rnd: &mut Random) -> &T {
        self.peek_random(rnd).unwrap()
    }
}

impl<T> PeekRandom<T> for Vec<T> {
    fn peek_random(&self, rnd: &mut Random) -> Option<&T> {
        if self.is_empty() {
            return None;
        } else {
            Some(&self[rnd.gen_index(self)])
        }
    }

    fn peek_random_exn(&self, rnd: &mut Random) -> &T {
        self.peek_random(rnd).unwrap()
    }
}
