use rand::{prelude::IteratorRandom, thread_rng};

pub trait RandomChooser {
    // fn choose_multiple<C, I>(&self, _iter: C, amount: usize) -> Vec<I>
    // where
    //     C: Iterator<Item = I>;
    fn choose_multiple(&self, _vec: Vec<usize>, amount: usize) -> Vec<usize>;
}

pub struct RandRandomChooser;

impl RandomChooser for RandRandomChooser {
    fn choose_multiple(&self, _vec: Vec<usize>, amount: usize) -> Vec<usize> {
        _vec.into_iter().choose_multiple(&mut thread_rng(), amount)
    }
}
