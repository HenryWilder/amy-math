pub mod multi_vec;

/// Helpers for testing/debugging custom collections
#[cfg(test)]
pub mod tracking {
    use std::{cell::Cell, sync::Arc};

    #[derive(Debug)]
    pub struct Tracked {
        clones: Arc<Cell<usize>>,
        drops:  Arc<Cell<usize>>,
    }

    impl Tracked {
        pub fn new() -> Self {
            Self {
                clones: Arc::new(Cell::new(0)),
                drops:  Arc::new(Cell::new(0)),
            }
        }

        pub fn times_cloned (&self) -> usize { self.clones.get() }
        pub fn times_dropped(&self) -> usize { self.drops .get() }
    }

    impl Clone for Tracked {
        fn clone(&self) -> Self {
            self.clones.set(self.clones.get() + 1);
            Self {
                clones: self.clones.clone(),
                drops:  self.drops .clone(),
            }
        }
    }

    impl Drop for Tracked {
        fn drop(&mut self) {
            self.drops.set(self.drops.get() + 1);
        }
    }
}
