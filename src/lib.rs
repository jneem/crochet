use std::any::Any;
use std::collections::HashMap;
use std::panic::Location;

//mod cx;
mod key;
mod tree;

//pub use cx::Cx;
pub use tree::{MutCursor, MutIterItem, Mutation, MutationIter, Tree};

// Leaving the following code around as a sketch for the memoization.
// Delete when implemented in the main tree.

/*

#[derive(Default)]
pub struct Cx {
    store: HashMap<Key, Box<dyn Any>>,
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub enum Key {
    // This should always be a valid location, but we'll have to
    // newtype to keep construction private.
    Caller(*const Location<'static>),
    // Will also have an index variant for collections
}

impl Cx {
    #[track_caller]
    pub fn foo(&self) {
        let caller = Location::caller();
        let caller_ptr: *const Location = caller;
        let caller_usize: usize = caller_ptr as usize;
        println!(
            "foo; location = {:?} {:?}, {:x}",
            caller, caller_ptr, caller_usize
        );
    }

    #[track_caller]
    pub fn state<T: Any>(&mut self, default: impl FnOnce() -> T) -> &mut T {
        let key = Key::Caller(Location::caller());
        self.store
            .entry(key)
            .or_insert_with(|| Box::new(default()))
            .downcast_mut()
            .unwrap()
    }
}

*/
