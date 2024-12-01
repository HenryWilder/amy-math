use paste::paste;

pub trait MultiVecColumns {
    type ColRef<'a> where Self: 'a;
    type ColMut<'a> where Self: 'a;
    type ItemRef<'a> where Self: 'a;
    type ItemMut<'a> where Self: 'a;
    fn col<'a>(&'a self, index: usize) -> Self::ColRef<'a>;
    fn col_mut<'a>(&'a mut self, index: usize) -> Self::ColMut<'a>;
    fn item<'a>(&'a self, row: usize, col: usize) -> Self::ItemRef<'a>;
    fn item_mut<'a>(&'a mut self, row: usize, col: usize) -> Self::ItemMut<'a>;
}

macro_rules! impl_multivec {
    (
        $count:tt = $($n:tt),+
    ) => {
        paste!{
            pub mod [<multi_vec $count>] {
                use std::{alloc::{self, Layout}, mem, ptr::{self, NonNull}};
                pub use super::MultiVecColumns;

                struct RawMultiVec<$([<T $n>]),+> {
                    ptrs: ($(NonNull<[<T $n>]>),+),
                    cap: usize,
                }

                unsafe impl<$([<T $n>]: Send),+> Send for RawMultiVec<$([<T $n>]),+> {}
                unsafe impl<$([<T $n>]: Sync),+> Sync for RawMultiVec<$([<T $n>]),+> {}

                impl<$([<T $n>]),+> RawMultiVec<$([<T $n>]),+> {
                    pub const fn new() -> Self {
                        assert!($(mem::size_of::<[<T $n>]>() != 0)&&+, "TODO: implement ZST support");
                        Self {
                            ptrs: ($(NonNull::<[<T $n>]>::dangling()),+),
                            cap: 0,
                        }
                    }

                    pub fn grow(&mut self) {
                        let new_cap = if self.cap == 0 { 1 } else { 2 * self.cap };
                        let new_layouts = [$(Layout::array::<[<T $n>]>(new_cap).unwrap()),+];

                        assert!($(new_layouts[$n].size() <= isize::MAX as usize)&&+, "allocation too large");

                        let new_ptrs = if self.cap == 0 {
                            new_layouts.map(|new_layout| unsafe { alloc::alloc(new_layout) })
                        } else {
                            [$(unsafe {
                                alloc::realloc(
                                    self.ptrs.$n.as_ptr().cast::<u8>(),
                                    Layout::array::<[<T $n>]>(self.cap).unwrap(),
                                    new_layouts[$n].size(),
                                )
                            }),+]
                        };

                        self.ptrs = ($(match NonNull::new(new_ptrs[$n].cast::<[<T $n>]>()) { Some(p) => p, None => alloc::handle_alloc_error(new_layouts[$n]), }),+);
                        self.cap = new_cap;
                    }
                }

                impl<$([<T $n>]),+> Drop for RawMultiVec<$([<T $n>]),+> {
                    fn drop(&mut self) {
                        if self.cap != 0 {
                            $(
                                let layout = Layout::array::<[<T $n>]>(self.cap).unwrap();
                                unsafe { alloc::dealloc(self.ptrs.$n.as_ptr().cast::<u8>(), layout); }
                            )+
                        }
                    }
                }

                #[doc = "A set of " $count " [`Vec`]s stored adjacently with shared capacity and length"]
                pub struct [<MultiVec $count>]<$([<T $n>]),+> {
                    buf: RawMultiVec<$([<T $n>]),+>,
                    len: usize,
                }

                impl<$([<T $n>]),+> [<MultiVec $count>]<$([<T $n>]),+> {
                    /// Construct an empty MultiVec without allocating
                    pub const fn new() -> Self {
                        assert!($(mem::size_of::<[<T $n>]>() != 0)&&+, "implementing ZSTs later");
                        Self {
                            buf: RawMultiVec::new(),
                            len: 0,
                        }
                    }

                    /// Get the number of rows in the vec
                    pub fn len(&self) -> usize {
                        self.len
                    }

                    /// Append a row to the end of the vec
                    pub fn push(&mut self, $([<col $n>]: [<T $n>]),+) {
                        if self.len == self.buf.cap { self.buf.grow(); }

                        $(unsafe { ptr::write::<[<T $n>]>(self.buf.ptrs.$n.as_ptr().add(self.len), [<col $n>]); })+

                        self.len += 1;
                    }

                    /// Remove the last item from the vec
                    pub fn pop(&mut self) -> Option<($([<T $n>]),+)> {
                        if self.len == 0 {
                            None
                        } else {
                            self.len -= 1;
                            Some((
                                $(unsafe { ptr::read::<[<T $n>]>(self.buf.ptrs.$n.as_ptr().add(self.len)) }),+
                            ))
                        }
                    }

                    /// Insert an item at an index
                    pub fn insert(&mut self, index: usize, $([<col $n>]: [<T $n>]),+) {
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
                            ptr::write(self.buf.ptrs.$n.as_ptr().add(index), [<col $n>]);
                        })+

                        self.len += 1;
                    }

                    /// Remove an item from an index
                    pub fn remove(&mut self, index: usize) -> ($([<T $n>]),+) {
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

                    /// Get the nth row (one item of each type) as a tuple of references
                    pub fn row<'a>(&'a self, index: usize) -> ($(&'a [<T $n>]),+) {
                        assert!(index < self.len, "index out of bounds");
                        ($(unsafe { self.buf.ptrs.$n.as_ptr().add(index).as_ref::<'a>() }.unwrap()),+)
                    }

                    /// Get the nth row (one item of each type) as a tuple of mutable references
                    pub fn row_mut<'a>(&'a mut self, index: usize) -> ($(&'a mut [<T $n>]),+) {
                        assert!(index < self.len, "index out of bounds");
                        ($(unsafe { self.buf.ptrs.$n.as_ptr().add(index).as_mut::<'a>() }.unwrap()),+)
                    }

                    $(
                    #[doc = "Column " $n " (all items of type `" [<T $n>] "`) as a slice"]
                    pub fn [<col $n>]<'a>(&'a self) -> &'a [[<T $n>]] {
                        unsafe { std::slice::from_raw_parts::<'a, [<T $n>]>(self.buf.ptrs.$n.as_ptr(), self.len) }
                    }

                    #[doc = "Column " $n " (all items of type `" [<T $n>] "`) as a mutable slice"]
                    pub fn [<col $n _mut>]<'a>(&'a mut self) -> &'a mut [[<T $n>]] {
                        unsafe { std::slice::from_raw_parts_mut::<'a, [<T $n>]>(self.buf.ptrs.$n.as_ptr(), self.len) }
                    }
                    )+

                    /// Get all columns as a tuple of slices
                    pub fn cols<'a>(&'a self) -> ($(&'a [[<T $n>]]),+) {
                        ($(self.[<col $n>]()),+)
                    }

                    /// Extend with an iterator over tuples of rows
                    pub fn extend_rows<T: IntoIterator<Item = ($([<T $n>]),+)>>(&mut self, iter: T) {
                        for row in iter {
                            self.push($(row.$n),+);
                        }
                    }

                    /// Extend with a tuple of iterators over columns
                    pub fn extend_cols(&mut self, iters: ($(impl IntoIterator<Item = [<T $n>]>),+)) {
                        // todo: make this more efficient
                        let mut iters = ($(iters.$n.into_iter()),+);
                        loop {
                            self.push($({
                                let item = iters.$n.next();
                                match item {
                                    Some(x) => x,
                                    None => break,
                                }
                            }),+);
                        }
                    }
                }

                impl<$([<T $n>]),+> Drop for [<MultiVec $count>]<$([<T $n>]),+> {
                    fn drop(&mut self) {
                        while let Some(_) = self.pop() {}
                    }
                }

                pub struct IntoIter<$([<T $n>]),+> {
                    _buf: RawMultiVec<$([<T $n>]),+>,
                    starts: ($(*const [<T $n>]),+),
                    ends:   ($(*const [<T $n>]),+),
                }

                impl<$([<T $n>]),+> Iterator for IntoIter<$([<T $n>]),+> {
                    type Item = ($([<T $n>]),+);

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
                        let len: usize = [$((self.ends.$n as usize - self.starts.$n as usize) / mem::size_of::<[<T $n>]>()),+]
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

                impl<$([<T $n>]),+> DoubleEndedIterator for IntoIter<$([<T $n>]),+> {
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

                impl<$([<T $n>]),+> ExactSizeIterator for IntoIter<$([<T $n>]),+> {}

                impl<$([<T $n>]),+> Drop for IntoIter<$([<T $n>]),+> {
                    fn drop(&mut self) {
                        for _ in &mut *self {}
                    }
                }

                impl<$([<T $n>]),+> IntoIterator for [<MultiVec $count>]<$([<T $n>]),+> {
                    type Item = ($([<T $n>]),+);

                    type IntoIter = IntoIter<$([<T $n>]),+>;

                    fn into_iter(self) -> IntoIter<$([<T $n>]),+> {
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

                impl<$([<T $n>]),+> FromIterator<($([<T $n>]),+)> for [<MultiVec $count>]<$([<T $n>]),+> {
                    fn from_iter<T: IntoIterator<Item = ($([<T $n>]),+)>>(iter: T) -> Self {
                        let mut result = Self::new();
                        result.extend_rows(iter);
                        result
                    }
                }

                impl<$([<T $n>]),+, $([<I $n>]: IntoIterator<Item = [<T $n>]>),+> From<($([<I $n>]),+)> for [<MultiVec $count>]<$([<T $n>]),+> {
                    fn from(iter: ($([<I $n>]),+)) -> Self {
                        let mut result = Self::new();
                        result.extend_cols(iter);
                        result
                    }
                }

                #[doc = "An enum of each type in a table-cell of [`" MultiVec $count "`]"]
                pub enum [<MultiVec $count Item>]<$([<T $n>]),+> {
                    $(
                        #[doc = "The type stored in column " $n]
                        [<Col $n>]([<T $n>])
                    ),+
                }

                impl<$([<T $n>]: std::fmt::Debug),+> std::fmt::Debug for [<MultiVec $count Item>]<$([<T $n>]),+> {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        match self {
                            $(Self::[<Col $n>](x) => f.debug_tuple(stringify!([<Col $n>])).field(x).finish(),)+
                        }
                    }
                }
                impl<$([<T $n>]: Clone),+> Clone for [<MultiVec $count Item>]<$([<T $n>]),+> {
                    fn clone(&self) -> Self {
                        match self {
                            $(Self::[<Col $n>](x) => Self::[<Col $n>](x.clone()),)+
                        }
                    }
                }
                impl<$([<T $n>]: Copy),+> Copy for [<MultiVec $count Item>]<$([<T $n>]),+> {}
                impl<$([<T $n>]: PartialEq),+> PartialEq for [<MultiVec $count Item>]<$([<T $n>]),+> {
                    fn eq(&self, rhs: &Self) -> bool {
                        match (self, rhs) {
                            $((Self::[<Col $n>](a), Self::[<Col $n>](b)) => a.eq(b),)+
                            _ => false,
                        }
                    }
                }

                impl<$([<T $n>]),+> MultiVecColumns for [<MultiVec $count>]<$([<T $n>]),+> {
                    type ColRef<'a> = [<MultiVec $count Item>]<$(&'a [[<T $n>]]),+>
                    where Self: 'a, $([<T $n>]: 'a),+;

                    type ColMut<'a> = [<MultiVec $count Item>]<$(&'a mut [[<T $n>]]),+>
                    where Self: 'a, $([<T $n>]: 'a),+;

                    type ItemRef<'a> = [<MultiVec $count Item>]<$(&'a [<T $n>]),+>
                    where Self: 'a, $([<T $n>]: 'a),+;

                    type ItemMut<'a> = [<MultiVec $count Item>]<$(&'a mut [<T $n>]),+>
                    where Self: 'a, $([<T $n>]: 'a),+;

                    fn col<'a>(&'a self, index: usize) -> Self::ColRef<'a> {
                        match index {
                            $($n => Self::ColRef::[<Col $n>](self.[<col $n>]()),)+
                            _ => panic!("index out of range"),
                        }
                    }

                    fn col_mut<'a>(&'a mut self, index: usize) -> Self::ColMut<'a> {
                        match index {
                            $($n => Self::ColMut::[<Col $n>](self.[<col $n _mut>]()),)+
                            _ => panic!("index out of range"),
                        }
                    }

                    fn item<'a>(&'a self, row: usize, col: usize) -> Self::ItemRef<'a> {
                        match col {
                            $($n => Self::ItemRef::[<Col $n>](&self.[<col $n>]()[row]),)+
                            _ => panic!("index out of range"),
                        }
                    }

                    fn item_mut<'a>(&'a mut self, row: usize, col: usize) -> Self::ItemMut<'a> {
                        match col {
                            $($n => Self::ItemMut::[<Col $n>](&mut self.[<col $n _mut>]()[row]),)+
                            _ => panic!("index out of range"),
                        }
                    }
                }
            }
        }
    };
}

impl_multivec!{ 2 = 0, 1 }
impl_multivec!{ 3 = 0, 1, 2 }
impl_multivec!{ 4 = 0, 1, 2, 3 }
impl_multivec!{ 5 = 0, 1, 2, 3, 4 }
impl_multivec!{ 6 = 0, 1, 2, 3, 4, 5 }

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
            v.push(R0C0, R0C1);
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
        use super::multi_vec6::*;

        #[test]
        fn test0() {
            let mut v = MultiVec6::<u8, i8, f32, u16, bool, &str>::new();
            assert_eq!(v.len(), 0);

            v.push(1, -3, 7.0, 2, true, "apple");
            assert_eq!(v.len(), 1);
            assert_eq!(v.row(0), (&1, &-3, &7.0, &2, &true, &"apple"));

            v.push(9, 102, -46.2, 67, false, "orange");
            assert_eq!(v.len(), 2);
            assert_eq!(v.row(1), (&9, &102, &-46.2, &67, &false, &"orange"));

            assert_eq!(v.col0(), &[1, 9]);
            assert_eq!(v.col1(), &[-3, 102]);
            assert_eq!(v.col2(), &[7.0, -46.2]);
            assert_eq!(v.col3(), &[2, 67]);
            assert_eq!(v.col4(), &[true, false]);
            assert_eq!(v.col5(), &["apple", "orange"]);

            assert_eq!(v.item(0, 3), MultiVec6Item::Col3(&2));
        }
    }
}
