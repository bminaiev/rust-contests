use std::cell::Cell;
use std::marker::PhantomData;
use std::ops::Index;

pub struct Node<T>
where
    T: Clone,
{
    ref_count: Cell<u32>,
    element: T,
}

pub struct NodesArena<T>
where
    T: Clone,
{
    elements: Vec<Node<T>>,
    empty: Vec<u32>,
}

pub trait ArenaContainer<T>
where
    T: Clone,
{
    fn arena() -> &'static mut NodesArena<T>;
}

pub struct ArenaRef<AC, T>(u32, PhantomData<AC>, PhantomData<T>)
where
    AC: ArenaContainer<T>,
    T: Clone + 'static;

impl<AC, T> Index<&ArenaRef<AC, T>> for NodesArena<T>
where
    AC: ArenaContainer<T>,
    T: Clone,
{
    type Output = T;

    fn index(&self, index: &ArenaRef<AC, T>) -> &Self::Output {
        assert_ne!(self.elements[index.0 as usize].ref_count.get(), 0);
        &self.elements[index.0 as usize].element
    }
}

impl<T> NodesArena<T>
where
    T: Clone,
{
    pub fn new() -> Self {
        Self {
            elements: vec![],
            empty: vec![],
        }
    }

    pub fn alloc<AC>(&mut self, element: T) -> ArenaRef<AC, T>
    where
        AC: ArenaContainer<T>,
    {
        if let Some(pos) = self.empty.pop() {
            assert_eq!(self.elements[pos as usize].ref_count.get(), 0);
            self.elements[pos as usize].ref_count.set(1);
            self.elements[pos as usize].element = element;
            ArenaRef(pos, PhantomData, PhantomData)
        } else {
            self.elements.push(Node {
                element,
                ref_count: Cell::new(1),
            });
            ArenaRef((self.elements.len() - 1) as u32, PhantomData, PhantomData)
        }
    }

    pub fn total_elements(&self) -> usize {
        self.elements.len()
    }
}

impl<AC, T> Drop for ArenaRef<AC, T>
where
    AC: ArenaContainer<T>,
    T: Clone + 'static,
{
    fn drop(&mut self) {
        let arena = AC::arena();
        let old_ref_counter = arena.elements[self.0 as usize].ref_count.get();
        *(arena.elements[self.0 as usize].ref_count.get_mut()) -= 1;
        if old_ref_counter == 1 {
            arena.empty.push(self.0);
        }
    }
}

impl<AC, T> Clone for ArenaRef<AC, T>
where
    AC: ArenaContainer<T>,
    T: Clone,
{
    fn clone(&self) -> Self {
        let arena = AC::arena();
        *(arena.elements[self.0 as usize].ref_count.get_mut()) += 1;
        Self(self.0, self.1, self.2)
    }
}
