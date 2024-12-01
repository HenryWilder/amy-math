macro_rules! impl_multivec {
    (
        $mod:ident::$MultiVec:ident<$(
            .$n:tt: [$i:literal] -> $T:ident,
        )+>
    ) => {
        pub mod $mod {
            use std::{alloc::{self, Layout}, mem, ptr::{self, NonNull}};
            use paste::paste;

            struct RawMultiVec<$($T),+> {
                ptrs: ($(NonNull<$T>),+),
                cap: usize,
            }

            unsafe impl<$($T: Send),+> Send for RawMultiVec<$($T),+> {}
            unsafe impl<$($T: Sync),+> Sync for RawMultiVec<$($T),+> {}

            impl<$($T),+> RawMultiVec<$($T),+> {
                pub const fn new() -> Self {
                    assert!($(mem::size_of::<$T>() != 0)&&+, "TODO: implement ZST support");
                    Self {
                        ptrs: ($(NonNull::<$T>::dangling()),+),
                        cap: 0,
                    }
                }

                pub fn grow(&mut self) {
                    let new_cap = if self.cap == 0 { 1 } else { 2 * self.cap };
                    let new_layouts = [$(Layout::array::<$T>(new_cap).unwrap()),+];

                    assert!($(new_layouts[$i].size() <= isize::MAX as usize)&&+, "allocation too large");

                    let new_ptrs = if self.cap == 0 {
                        new_layouts.map(|new_layout| unsafe { alloc::alloc(new_layout) })
                    } else {
                        [$(unsafe {
                            alloc::realloc(
                                self.ptrs.$n.as_ptr().cast::<u8>(),
                                Layout::array::<$T>(self.cap).unwrap(),
                                new_layouts[$i].size(),
                            )
                        }),+]
                    };

                    self.ptrs = ($(match NonNull::new(new_ptrs[$i].cast::<$T>()) { Some(p) => p, None => alloc::handle_alloc_error(new_layouts[$i]), }),+);
                    self.cap = new_cap;
                }
            }

            impl<$($T),+> Drop for RawMultiVec<$($T),+> {
                fn drop(&mut self) {
                    if self.cap != 0 {
                        $(
                            let layout = Layout::array::<$T>(self.cap).unwrap();
                            unsafe { alloc::dealloc(self.ptrs.$n.as_ptr().cast::<u8>(), layout); }
                        )+
                    }
                }
            }

            /// A set of multiple [`Vec`]s stored adjacently with shared capacity and length
            pub struct $MultiVec<$($T),+> {
                buf: RawMultiVec<$($T),+>,
                len: usize,
            }

            impl<$($T),+> $MultiVec<$($T),+> {
                /// Construct an empty MultiVec without allocating
                pub const fn new() -> Self {
                    assert!($(mem::size_of::<$T>() != 0)&&+, "implementing ZSTs later");
                    Self {
                        buf: RawMultiVec::new(),
                        len: 0,
                    }
                }

                pub fn len(&self) -> usize {
                    self.len
                }

                /// Append to the vec with an iterator over tuples of columns
                pub fn push(&mut self, elems: ($($T),+)) {
                    if self.len == self.buf.cap { self.buf.grow(); }

                    $(unsafe { ptr::write::<$T>(self.buf.ptrs.$n.as_ptr().add(self.len), elems.$n); })+

                    self.len += 1;
                }

                /// Append an item to the end of the vec
                pub fn pop(&mut self) -> Option<($($T),+)> {
                    if self.len == 0 {
                        None
                    } else {
                        self.len -= 1;
                        Some((
                            $(unsafe { ptr::read::<$T>(self.buf.ptrs.$n.as_ptr().add(self.len)) }),+
                        ))
                    }
                }

                /// Get the nth row (one item of each type) as a tuple of references
                pub fn insert(&mut self, index: usize, row: ($($T),+)) {
                    // Note: `<=` because it's valid to insert after everything
                    // which would be equivalent to push.
                    assert!(index <= self.len, "index out of bounds");
                    if self.len == self.buf.cap { self.buf.grow(); }

                    $(unsafe {
                        ptr::copy(
                            self.buf.ptrs.$n.as_ptr().add(index),
                            self.buf.ptrs.$n.as_ptr().add(index + 1),
                            self.len - index,
                        );
                        ptr::write(self.buf.ptrs.$n.as_ptr().add(index), row.$n);
                    })+

                    self.len += 1;
                }

                /// Insert an item at an index
                pub fn remove(&mut self, index: usize) -> ($($T),+) {
                    // Note: `<` because it's *not* valid to remove after everything
                    assert!(index < self.len, "index out of bounds");
                    self.len -= 1;
                    let result = ($(unsafe { ptr::read(self.buf.ptrs.$n.as_ptr().add(index)) }),+);
                    $(unsafe {
                        ptr::copy(
                            self.buf.ptrs.$n.as_ptr().add(index + 1),
                            self.buf.ptrs.$n.as_ptr().add(index),
                            self.len - index,
                        );
                    })+
                    result
                }

                /// Remove the last item from the vec
                pub fn row<'a>(&'a self, index: usize) -> ($(&'a $T),+) {
                    assert!(index < self.len, "index out of bounds");
                    ($(unsafe { self.buf.ptrs.$n.as_ptr().add(index).as_ref::<'a>() }.unwrap()),+)
                }

                /// Get the nth row (one item of each type) as a tuple of references
                pub fn row_mut<'a>(&'a mut self, index: usize) -> ($(&'a mut $T),+) {
                    assert!(index < self.len, "index out of bounds");
                    ($(unsafe { self.buf.ptrs.$n.as_ptr().add(index).as_mut::<'a>() }.unwrap()),+)
                }

                $(
                paste!{
                    #[doc = "Column " $n " (all items of type `" $T "`) as a slice"]
                    pub fn [<col $n>]<'a>(&'a self) -> &'a [$T] {
                        unsafe { std::slice::from_raw_parts::<'a, $T>(self.buf.ptrs.$n.as_ptr(), self.len) }
                    }
                }

                paste!{
                    #[doc = "Column " $n " (all items of type `" $T "`) as a mutable slice"]
                    pub fn [<col $n _mut>]<'a>(&'a mut self) -> &'a mut [$T] {
                        unsafe { std::slice::from_raw_parts_mut::<'a, $T>(self.buf.ptrs.$n.as_ptr(), self.len) }
                    }
                }
                )+

                /// Get all columns as a tuple of slices
                pub fn cols<'a>(&'a self) -> ($(&'a [$T]),+) {
                    paste!(($(self.[<col $n>]()),+))
                }

                /// Remove an item from an index
                pub fn extend_rows<T: IntoIterator<Item = ($($T),+)>>(&mut self, iter: T) {
                    for row in iter {
                        self.push(row);
                    }
                }

                /// Extend with a tuple of iterators over columns
                pub fn extend_cols(&mut self, iters: ($(impl IntoIterator<Item = $T>),+)) {
                    // todo: make this more efficient
                    let mut iters = ($(iters.$n.into_iter()),+);
                    loop {
                        let row = ($({
                            let item = iters.$n.next();
                            match item {
                                Some(x) => x,
                                None => break,
                            }
                        }),+);
                        self.push(row);
                    }
                }
            }

            impl<$($T),+> Drop for $MultiVec<$($T),+> {
                fn drop(&mut self) {
                    while let Some(_) = self.pop() {}
                }
            }

            pub struct IntoIter<$($T),+> {
                _buf: RawMultiVec<$($T),+>,
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

                #[inline]
                fn count(self) -> usize {
                    self.len()
                }
            }

            impl<$($T),+> DoubleEndedIterator for IntoIter<$($T),+> {
                fn next_back(&mut self) -> Option<<Self as Iterator>::Item> {
                    if $(self.starts.$n == self.ends.$n)||+ {
                        None
                    } else {
                        unsafe {
                            $(self.ends.$n = self.ends.$n.offset(-1);)+
                            Some(($(ptr::read(self.ends.$n)),+))
                        }
                    }
                }
            }

            impl<$($T),+> ExactSizeIterator for IntoIter<$($T),+> {}

            impl<$($T),+> Drop for IntoIter<$($T),+> {
                fn drop(&mut self) {
                    for _ in &mut *self {}
                }
            }

            impl<$($T),+> IntoIterator for $MultiVec<$($T),+> {
                type Item = ($($T),+);
                type IntoIter = IntoIter<$($T),+>;
                fn into_iter(self) -> IntoIter<$($T),+> {
                    let buf = unsafe { ptr::read(&self.buf) };
                    let len = self.len;
                    mem::forget(self);

                    IntoIter {
                        starts: ($(buf.ptrs.$n.as_ptr()),+),
                        ends: if buf.cap == 0 {
                            ($(buf.ptrs.$n.as_ptr()),+)
                        } else {
                            unsafe { ($(buf.ptrs.$n.as_ptr().add(len)),+) }
                        },
                        _buf: buf,
                    }
                }
            }

            impl<$($T),+> FromIterator<($($T),+)> for $MultiVec<$($T),+> {
                fn from_iter<T: IntoIterator<Item = ($($T),+)>>(iter: T) -> Self {
                    let mut result = Self::new();
                    result.extend_rows(iter);
                    result
                }
            }

            paste!{
                impl<$($T),+, $([<I $n>]: IntoIterator<Item = $T>),+> From<($([<I $n>]),+)> for $MultiVec<$($T),+> {
                    fn from(iter: ($([<I $n>]),+)) -> Self {
                        let mut result = Self::new();
                        result.extend_cols(iter);
                        result
                    }
                }
            }
        }
    };
}

impl_multivec!{
    multi_vec2::MultiVec2<
        .0: [0] -> T0,
        .1: [1] -> T1,
    >
}

impl_multivec!{
    multi_vec3::MultiVec3<
        .0: [0] -> T0,
        .1: [1] -> T1,
        .2: [2] -> T2,
    >
}

impl_multivec!{
    multi_vec4::MultiVec4<
        .0: [0] -> T0,
        .1: [1] -> T1,
        .2: [2] -> T2,
        .3: [3] -> T3,
    >
}

impl_multivec!{
    multi_vec5::MultiVec5<
        .0: [0] -> T0,
        .1: [1] -> T1,
        .2: [2] -> T2,
        .3: [3] -> T3,
        .4: [4] -> T4,
    >
}

impl_multivec!{
    multi_vec6::MultiVec6<
        .0: [0] -> T0,
        .1: [1] -> T1,
        .2: [2] -> T2,
        .3: [3] -> T3,
        .4: [4] -> T4,
        .5: [5] -> T5,
    >
}

#[cfg(test)]
mod tests {
    use super::*;

    mod _2 {
        use super::multi_vec2::*;

        #[test]
        #[should_panic(expected = "index out of bounds")]
        fn test0() {
            type Col0 = i32;
            type Col1 = f32;
            let v = MultiVec2::<Col0, Col1>::new();
            _ = v.row(0).0;
        }

        #[test]
        fn test1() {
            type Col0 = i32;
            type Col1 = u8;
            const R0C0: Col0 = 24754;
            const R0C1: Col1 = b'V';

            let mut v = MultiVec2::<Col0, Col1>::new();
            v.push((R0C0, R0C1));
            assert_eq!(v.row(0), (&R0C0, &R0C1), "row[0] should match pushed value");
            assert_eq!(v.col0()[0], R0C0, "col0 should match pushed value");
            assert_eq!(v.col1()[0], R0C1, "col1 should match pushed value");
        }

        #[test]
        fn test_extend_rows() {
            type Col0 = i32;
            type Col1 = u8;
            const COL0: [Col0; 3] = [4, 7, 2];
            const COL1: [Col1; 3] = [b'A', b'9', 3];

            let v = MultiVec2::<Col0, Col1>::from((COL0, COL1));
            assert_eq!(v.row(0), (&COL0[0], &COL1[0]));
            assert_eq!(v.row(1), (&COL0[1], &COL1[1]));
            assert_eq!(v.row(2), (&COL0[2], &COL1[2]));
        }
    }

    mod _6 {
        use super::{multi_vec6::*};

        #[test]
        fn test0() {
            let mut v = MultiVec6::<u8, i8, f32, u16, bool, &str>::new();
            assert_eq!(v.len(), 0);

            v.push((1, -3, 7.0, 2, true, "apple"));
            assert_eq!(v.len(), 1);
            assert_eq!(v.row(0), (&1, &-3, &7.0, &2, &true, &"apple"));

            v.push((9, 102, -46.2, 67, false, "orange"));
            assert_eq!(v.len(), 2);
            assert_eq!(v.row(1), (&9, &102, &-46.2, &67, &false, &"orange"));

            assert_eq!(v.col0(), &[1, 9]);
            assert_eq!(v.col1(), &[-3, 102]);
            assert_eq!(v.col2(), &[7.0, -46.2]);
            assert_eq!(v.col3(), &[2, 67]);
            assert_eq!(v.col4(), &[true, false]);
            assert_eq!(v.col5(), &["apple", "orange"]);
        }
    }
}
