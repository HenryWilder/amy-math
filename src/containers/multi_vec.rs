macro_rules! impl_multivec {
    (
        $mod:ident::$MultiVec:ident<$(
            .$n:tt: [$i:literal]: [$col_n:ident, $col_n_mut:ident] -> $T:ident,
        )+>
    ) => {
        pub mod $mod {
            use std::{alloc::{self, Layout}, mem, ptr::{self, NonNull}};
            use paste::paste;

            /// A set of multiple [`Vec`]s stored adjacently with shared capacity and length
            pub struct $MultiVec<$($T),+> {
                ptrs: ($(NonNull<$T>),+),
                cap: usize,
                len: usize,
            }

            unsafe impl<$($T: Send),+> Send for $MultiVec<$($T),+> {}
            unsafe impl<$($T: Sync),+> Sync for $MultiVec<$($T),+> {}

            impl<$($T),+> Drop for $MultiVec<$($T),+> {
                fn drop(&mut self) {
                    if self.cap != 0 {
                        while let Some(_) = self.pop() { }
                        let layouts = [$(Layout::array::<$T>(self.cap).unwrap()),+];
                        $(unsafe { alloc::dealloc(self.ptrs.$n.as_ptr().cast::<u8>(), layouts[$i]); })+
                    }
                }
            }

            impl<$($T),+> $MultiVec<$($T),+> {
                /// Construct an empty MultiVec without allocating
                pub const fn new() -> Self {
                    assert!($(mem::size_of::<$T>() != 0)&&+, "implementing ZSTs later");
                    Self {
                        ptrs: ($(NonNull::<$T>::dangling()),+),
                        cap: 0,
                        len: 0,
                    }
                }

                fn grow(&mut self) {
                    let new_cap = if self.cap == 0 { 1 } else { 2 * self.cap };
                    let new_layouts = [$(Layout::array::<$T>(new_cap).unwrap()),+];

                    for new_layout in new_layouts {
                        assert!(new_layout.size() <= isize::MAX as usize, "allocation too large");
                    }

                    let new_ptrs = if self.cap == 0 {
                        new_layouts.map(|new_layout| unsafe { alloc::alloc(new_layout) })
                    } else {
                        [$(unsafe { alloc::realloc(self.ptrs.$n.as_ptr().cast::<u8>(), Layout::array::<$T>(self.cap).unwrap(), new_layouts[$i].size()) }),+]
                    };

                    self.ptrs = ($(match NonNull::new(new_ptrs[$i].cast::<$T>()) { Some(p) => p, None => alloc::handle_alloc_error(new_layouts[$i]), }),+);
                    self.cap = new_cap;
                }

                /// Append an item to the back
                pub fn push(&mut self, elems: ($($T),+)) {
                    if self.len == self.cap { self.grow(); }

                    $(unsafe { ptr::write::<$T>(self.ptrs.$n.as_ptr().add(self.len), elems.$n); })+

                    self.len += 1;
                }

                /// Remove and return the item at the back
                pub fn pop(&mut self) -> Option<($($T),+)> {
                    if self.len == 0 {
                        None
                    } else {
                        self.len -= 1;
                        Some((
                            $(unsafe { ptr::read::<$T>(self.ptrs.$n.as_ptr().add(self.len)) }),+
                        ))
                    }
                }

                $(
                paste!{
                    #[doc = "Column " $n " (all items of type `" $T "`) as a slice"]
                    pub fn $col_n<'a>(&'a self) -> &'a [$T] {
                        unsafe { std::slice::from_raw_parts::<'a, $T>(self.ptrs.$n.as_ptr(), self.len) }
                    }
                }

                paste!{
                    #[doc = "Column " $n " (all items of type `" $T "`) as a mutable slice"]
                    pub fn $col_n_mut<'a>(&'a mut self) -> &'a mut [$T] {
                        unsafe { std::slice::from_raw_parts_mut::<'a, $T>(self.ptrs.$n.as_ptr(), self.len) }
                    }
                }
                )+

                /// Get all columns as a tuple of slices
                pub fn cols<'a>(&'a self) -> ($(&'a [$T]),+) {
                    ($(self.$col_n()),+)
                }

                /// Get the nth row (one item of each type) as a tuple of references
                ///
                /// This is what you will normally iterate over
                pub fn row<'a>(&'a self, index: usize) -> ($(&'a $T),+) {
                    assert!(index <= self.len, "index out of bounds");
                    ($(unsafe { self.ptrs.$n.as_ptr().add(index).as_ref::<'a>() }.unwrap()),+)
                }

                /// Get the nth row (one item of each type) as a tuple of references
                ///
                /// This is what you will normally iterate over
                pub fn row_mut<'a>(&'a mut self, index: usize) -> ($(&'a mut $T),+) {
                    assert!(index <= self.len, "index out of bounds");
                    ($(unsafe { self.ptrs.$n.as_ptr().add(index).as_mut::<'a>() }.unwrap()),+)
                }

                /// Insert an item at an index
                pub fn insert(&mut self, index: usize, elems: ($($T),+)) {
                    // Note: `<=` because it's valid to insert after everything
                    // which would be equivalent to push.
                    assert!(index <= self.len, "index out of bounds");
                    if self.len == self.cap { self.grow(); }

                    $(unsafe {
                        ptr::copy(
                            self.ptrs.$n.as_ptr().add(index),
                            self.ptrs.$n.as_ptr().add(index + 1),
                            self.len - index,
                        );
                        ptr::write(self.ptrs.$n.as_ptr().add(index), elems.$n);
                    })+

                    self.len += 1;
                }

                /// Remove an item from an index
                pub fn remove(&mut self, index: usize) -> ($($T),+) {
                    // Note: `<` because it's *not* valid to remove after everything
                    assert!(index < self.len, "index out of bounds");
                    self.len -= 1;
                    let result = ($(unsafe { ptr::read(self.ptrs.$n.as_ptr().add(index)) }),+);
                    $(unsafe {
                        ptr::copy(
                            self.ptrs.$n.as_ptr().add(index + 1),
                            self.ptrs.$n.as_ptr().add(index),
                            self.len - index,
                        );
                    })+
                    result
                }
            }

            pub struct IntoIter<$($T),+> {
                bufs: ($(NonNull<$T>),+),
                cap: usize,
                starts: ($(*const $T),+),
                ends:   ($(*const $T),+),
            }

            impl<$($T),+> Iterator for IntoIter<$($T),+> {
                type Item = ($($T),+);
                fn next(&mut self) -> Option<Self::Item> {
                    if self.starts == self.ends {
                        None
                    } else {
                        let result = ($(unsafe { ptr::read(self.starts.$n) }),+);
                        $(self.starts.$n = unsafe { self.starts.$n.offset(1) };)+
                        Some(result)
                    }
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    let len: usize = [$((self.ends.$n as usize - self.starts.$n as usize) / mem::size_of::<$T>()),+]
                        .into_iter()
                        .max()
                        .expect("MultiVec must have at least two types");
                    (len, Some(len))
                }
            }

            impl<$($T),+> DoubleEndedIterator for IntoIter<$($T),+> {
                fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
                    todo!()
                }
            }

            impl<$($T),+> Drop for IntoIter<$($T),+> {
                fn drop(&mut self) {
                    if self.cap != 0 {
                        for _ in &mut *self {}
                        $(
                            let layout = Layout::array::<$T>(self.cap).unwrap();
                            unsafe { alloc::dealloc(self.bufs.$n.as_ptr().cast::<u8>(), layout); }
                        )+
                    }
                }
            }

            impl<$($T),+> IntoIterator for $MultiVec<$($T),+> {
                type Item = ($($T),+);
                type IntoIter = IntoIter<$($T),+>;
                fn into_iter(self) -> IntoIter<$($T),+> {
                    let vec = mem::ManuallyDrop::new(self);

                    let ptrs = vec.ptrs;
                    let cap  = vec.cap;
                    let len  = vec.len;

                    IntoIter {
                        bufs: ptrs,
                        cap,
                        starts: ($(ptrs.$n.as_ptr()),+),
                        ends: if cap == 0 {
                            ($(ptrs.$n.as_ptr()),+)
                        } else {
                            unsafe { ($(ptrs.$n.as_ptr().add(len)),+) }
                        },
                    }
                }
            }
        }
    };
}

// impl Iterator for i32 {
//     type Item;

//     fn next(&mut self) -> Option<Self::Item> {
//         todo!()
//     }

//     fn size_hint(&self) -> (usize, Option<usize>) {

//     }
// }

// fn test() {
//     let x = [].into_iter().max()
// }

impl_multivec!{
    multi_vec2::MultiVec2<
        .0: [0]: [col0, col0_mut] -> T0,
        .1: [1]: [col1, col1_mut] -> T1,
    >
}

impl_multivec!{
    multi_vec3::MultiVec3<
        .0: [0]: [col0, col0_mut] -> T0,
        .1: [1]: [col1, col1_mut] -> T1,
        .2: [2]: [col2, col2_mut] -> T2,
    >
}

impl_multivec!{
    multi_vec4::MultiVec4<
        .0: [0]: [col0, col0_mut] -> T0,
        .1: [1]: [col1, col1_mut] -> T1,
        .2: [2]: [col2, col2_mut] -> T2,
        .3: [3]: [col3, col3_mut] -> T3,
    >
}

impl_multivec!{
    multi_vec5::MultiVec5<
        .0: [0]: [col0, col0_mut] -> T0,
        .1: [1]: [col1, col1_mut] -> T1,
        .2: [2]: [col2, col2_mut] -> T2,
        .3: [3]: [col3, col3_mut] -> T3,
        .4: [4]: [col4, col4_mut] -> T4,
    >
}

impl_multivec!{
    multi_vec6::MultiVec6<
        .0: [0]: [col0, col0_mut] -> T0,
        .1: [1]: [col1, col1_mut] -> T1,
        .2: [2]: [col2, col2_mut] -> T2,
        .3: [3]: [col3, col3_mut] -> T3,
        .4: [4]: [col4, col4_mut] -> T4,
        .5: [5]: [col5, col5_mut] -> T5,
    >
}

#[cfg(test)]
mod tests {
    use crate::containers::tracking::Tracked;
    use super::*;

    #[cfg(test)]
    mod _2 {
        use super::{Tracked, multi_vec2::*};

        #[test]
        fn test0() {
            let tracer1 = Tracked::new();
            let tracer2 = Tracked::new();
            {
                let mut v = MultiVec2::new();
                v.push((tracer1.clone(), tracer2.clone()));
            }
            assert_eq!(tracer1.times_cloned(), tracer1.times_dropped(), "all clones should be dropped");
            assert_eq!(tracer2.times_cloned(), tracer2.times_dropped(), "all clones should be dropped");
        }
    }

}
