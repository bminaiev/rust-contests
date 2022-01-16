#[cfg(test)]
mod tests {
    use crate::misc::nodes_arena::{ArenaContainer, ArenaRef, NodesArena};

    #[test]
    fn simple() {
        #[derive(Clone, Debug)]
        struct T(i32);

        static mut ARENA_: Option<NodesArena<T>> = None;
        fn arena() -> &'static mut NodesArena<T> {
            unsafe {
                if ARENA_.is_none() {
                    ARENA_ = Some(NodesArena::new());
                }
                match &mut ARENA_ {
                    None => unreachable!(),
                    Some(arena_not_opt) => return arena_not_opt,
                }
            }
        }

        struct Arena;
        impl ArenaContainer<T> for Arena {
            fn arena() -> &'static mut NodesArena<T> {
                arena()
            }
        }

        impl Arena {
            pub fn alloc(element: T) -> ArenaRef<Self, T> {
                arena().alloc::<Self>(element)
            }
        }

        {
            let refer3 = {
                let refer = Arena::alloc(T(123));
                let elem = refer.get();
                println!("{:?}", elem);
                let refer2 = Arena::alloc(T(787788));
                println!("{:?}, {:?}", refer.get(), refer2.get());
                let sum = refer.get().0 + refer2.get().0;
                assert_eq!(sum, 123 + 787788);
                let zz = refer.get();
                let old_zz = zz.0;
                let _ids: Vec<_> = (0..100)
                    .map(|_| Arena::alloc(T(refer.get().0 + refer2.get().0 + zz.0)))
                    .collect();
                assert_eq!(old_zz, zz.0);
                Arena::alloc(T(zz.0));
                refer.clone()
            };
            println!("{:?}", refer3.get());
        }
        {
            let _refer4 = Arena::alloc(T(100));
        }
        assert_eq!(arena().total_elements(), 103);
    }
}
