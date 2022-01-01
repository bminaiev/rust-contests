use crate::misc::nodes_arena::{ArenaContainer, NodesArena};

#[test]
fn simple() {
    #[derive(Clone, Debug)]
    struct T(i32);

    static mut arena_: Option<NodesArena<T>> = None;
    fn arena() -> &'static mut NodesArena<T> {
        unsafe {
            if arena_.is_none() {
                arena_ = Some(NodesArena::new());
            }
            match &mut arena_ {
                None => unreachable!(),
                Some(arena_not_opt) => return arena_not_opt,
            }
        }
    }

    struct ArenaContainerImpl;
    impl ArenaContainer<T> for ArenaContainerImpl {
        fn arena() -> &'static mut NodesArena<T> {
            arena()
        }
    }

    {
        let refer3 = {
            let refer = arena().alloc::<ArenaContainerImpl>(T(123));
            let elem = &arena()[&refer];
            println!("{:?}", elem);
            let refer2 = arena().alloc::<ArenaContainerImpl>(T(787788));
            println!("{:?}, {:?}", arena()[&refer], arena()[&refer2]);
            refer.clone()
        };
        println!("{:?}", arena()[&refer3]);
    }
    {
        let _refer4 = arena().alloc::<ArenaContainerImpl>(T(100));
    }
    assert_eq!(arena().total_elements(), 2);
}
