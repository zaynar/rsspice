use f2rust_std::{CharArray, CharArrayMut};
use std::ops::{Index, IndexMut};

const LBCELL: i32 = -5;

// Number of elements reserved for cell metadata
const METADATA: usize = (1 - LBCELL) as usize;

const MD_SIZE: usize = 4; // index of size (capacity)
const MD_CARD: usize = 5; // index of card (len)

mod internal {
    use std::fmt::Debug;

    /// Trait implemented for all types that Cell can be instantiated over
    pub trait CellValue: Clone + Debug {
        fn default() -> Self;
        fn get_usize(&self) -> usize;
        fn set_usize(&mut self, value: usize);
    }

    impl CellValue for i32 {
        fn default() -> Self {
            0
        }

        fn get_usize(&self) -> usize {
            *self as usize
        }

        fn set_usize(&mut self, value: usize) {
            *self = value as i32;
        }
    }

    impl CellValue for f64 {
        fn default() -> Self {
            0.0
        }

        fn get_usize(&self) -> usize {
            *self as usize
        }

        fn set_usize(&mut self, value: usize) {
            *self = value as f64;
        }
    }
}

/// Dynamic arrays of `i32` or `f64`.
///
/// [Cells](crate::required_reading::cells) are SPICELIB's equivalent to Rust's `Vec`.
/// Every cell has a size (`capacity`) and cardinality (`len`).
/// SPICELIB functions may alter a cell's cardinality, as they add or remove elements,
/// but they cannot alter its size.
///
/// In the SPICELIB documentation, cells are declared like `INTEGER X(LBCELL:100)`,
/// and the elements are accessed as `X(1)` to `X(100)` inclusive.
/// The equivalent in Rust is declared as `let x = Cell::with_capacity(100)`,
/// and the elements are `x[0]` to `x[99]` inclusive.
///
/// Unlike `Vec`, elements between `len` and `capacity` still have defined
/// values and can be safely accessed.
#[derive(Clone, Debug)]
pub struct Cell<T> {
    // First METADATA elements are [0, 0, 0, 0, size, card],
    // where size + METADATA == data.len(), and card <= size
    data: Vec<T>,
}

impl<T> Cell<T>
where
    T: internal::CellValue,
{
    /// Constructs a cell with the given capacity.
    ///
    /// Equivalent to `SSIZED`/`SSIZEI`.
    pub fn with_capacity(capacity: usize) -> Self {
        let mut data = vec![T::default(); METADATA + capacity];
        for i in 0..MD_SIZE {
            data[i].set_usize(0);
        }
        data[MD_SIZE].set_usize(capacity);
        data[MD_CARD].set_usize(0);
        Self { data }
    }

    /// Returns the cell's cardinality (the number of valid elements).
    ///
    /// Equivalent to `CARDD`/`CARDI`.
    pub fn len(&self) -> usize {
        self.data[MD_CARD].get_usize()
    }

    /// Returns the cell's size (the maximum cardinality).
    ///
    /// Equivalent to `SIZED`/`SIZEI`.
    pub fn capacity(&self) -> usize {
        self.data[MD_SIZE].get_usize()
    }

    /// Returns true if `len` is zero.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pushes a value onto the cell. If `capacity` is too low, it will automatically grow.
    ///
    /// Equivalent to `APPNDD`/`APPNDI`.
    pub fn push(&mut self, value: T) {
        let len = self.data[MD_CARD].get_usize();
        self.reserve(1);
        self.data[METADATA + len] = value;
        self.data[MD_CARD].set_usize(len + 1);

        self.validate();
    }

    /// Reserves capacity for at least `additional` more elements to be pushed.
    pub fn reserve(&mut self, additional: usize) {
        let len = self.len();
        if len + additional >= self.capacity() {
            self.data.resize(METADATA + len + additional, T::default());
            self.data[MD_SIZE].set_usize(len + additional);
        }

        self.validate();
    }

    /// Resizes the cell so that `len() == new_len`.
    ///
    /// If `new_len > len()`, copies of `value` will be inserted in the new slots.
    ///
    /// If `new_len > capacity()`, the capacity will be automatically increased.
    ///
    /// Equivalent to `SCARDD`/`SCARDI`.
    pub fn resize(&mut self, new_len: usize, value: T) {
        if new_len > self.capacity() {
            self.reserve(new_len - self.capacity());
        }
        for i in self.len()..new_len {
            self[i] = value.clone();
        }
        self.data[MD_CARD].set_usize(new_len);

        self.validate();
    }

    /// Returns the cell's raw data, including metadata elements, as a slice.
    pub fn as_raw_slice(&self) -> &[T] {
        self.validate();
        &self.data
    }

    /// Returns the cell's raw data, including metadata elements, as a mutable slice.
    pub fn as_raw_mut_slice(&mut self) -> &mut [T] {
        self.validate();
        &mut self.data
    }

    /// Detects internal errors in the data structure.
    pub(crate) fn validate(&self) {
        assert!(self.len() <= self.capacity());
        assert_eq!(self.data.len(), METADATA + self.capacity());
    }

    /// Returns an iterator over the cell's data, up to `len`.
    pub fn iter(&self) -> impl Iterator<Item = &T> {
        let len = self.len();
        self.data.iter().skip(METADATA).take(len)
    }

    /// Returns an iterator over the cell's data, up to `len`.
    pub fn into_iter(self) -> impl Iterator<Item = T> {
        let len = self.len();
        self.data.into_iter().skip(METADATA).take(len)
    }
}

impl<T> Index<usize> for Cell<T>
where
    T: internal::CellValue,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[METADATA + index]
    }
}

impl<T> IndexMut<usize> for Cell<T>
where
    T: internal::CellValue,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[METADATA + index]
    }
}

/// Dynamic arrays of strings.
///
/// [Cells](crate::required_reading::cells) are SPICELIB's equivalent to Rust's `Vec`.
/// Every cell has a size (`capacity`) and cardinality (`len`).
/// SPICELIB functions may alter a cell's cardinality, as they add or remove elements,
/// but they cannot alter its size.
///
/// Character cells also have a fixed element length, indicating the maximum length of string
/// it can store.
///
/// In the SPICELIB documentation, cells are declared like `CHARACTER*(20) X(LBCELL:100)`,
/// and the elements are accessed as `X(1)` to `X(100)` inclusive.
/// The equivalent in Rust is declared as `let x = CharCell::with_capacity_and_length(100, 20)`,
/// and the elements are `x[0]` to `x[99]` inclusive.
///
/// Unlike `Vec`, elements between `len` and `capacity` still have defined
/// values and can be safely accessed.
#[derive(Clone, Debug)]
pub struct CharCell {
    data: Vec<u8>,
    element_length: usize,
}

impl CharCell {
    fn set_usize(&mut self, i: usize, value: usize) {
        // To emulate ENCHAR, copy the 5-byte BE encoding of value
        let s = (value as u64).to_be_bytes();
        self.data[i * self.element_length..i * self.element_length + 5].copy_from_slice(&s[3..8]);
    }

    fn get_usize(&self, i: usize) -> usize {
        let mut s = [0; 8];
        s[3..8].copy_from_slice(&self.data[i * self.element_length..i * self.element_length + 5]);
        u64::from_be_bytes(s) as usize
    }

    /// Constructs a cell with the given capacity, containing strings
    /// of the given maximum length.
    ///
    /// Equivalent to `SSIZEC`.
    pub fn with_capacity_and_length(capacity: usize, element_length: usize) -> Self {
        // Must have enough space to encode size/card
        assert!(
            element_length >= 5,
            "CharCell element length must exceed MINLEN (5)"
        );

        let data = vec![b' '; (METADATA + capacity) * element_length];
        let mut cell = Self {
            data,
            element_length,
        };
        for i in 0..MD_SIZE {
            cell.set_usize(i, 0);
        }
        cell.set_usize(MD_SIZE, capacity);
        cell.set_usize(MD_CARD, 0);
        cell
    }

    /// The maximum length of strings that can be stored in this cell.
    pub fn element_length(&self) -> usize {
        self.element_length
    }

    /// Returns the cell's cardinality (the number of valid elements).
    ///
    /// Equivalent to `CARDC`.
    pub fn len(&self) -> usize {
        self.get_usize(MD_CARD)
    }

    /// Returns the cell's size (the maximum cardinality).
    ///
    /// Equivalent to `SIZEC`.
    pub fn capacity(&self) -> usize {
        self.get_usize(MD_SIZE)
    }

    /// Returns true if `len` is zero.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pushes a value onto the cell.
    ///
    /// If `capacity` is too low, it will automatically grow.
    /// If the string is longer than `element_length`, it will be truncated.
    ///
    /// Equivalent to `APPNDC`.
    pub fn push(&mut self, value: &str) {
        let len = self.get_usize(MD_CARD);
        self.reserve(1);
        self.set(len, value);
        self.set_usize(MD_CARD, len + 1);

        self.validate();
    }

    /// Reserves capacity for at least `additional` more elements to be pushed.
    pub fn reserve(&mut self, additional: usize) {
        let len = self.len();
        if len + additional >= self.capacity() {
            self.data
                .resize((METADATA + len + additional) * self.element_length, b' ');
            self.set_usize(MD_SIZE, len + additional);
        }

        self.validate();
    }

    /// Resizes the cell so that `len() == new_len`.
    ///
    /// If `new_len > len()`, blank strings will be inserted in the new slots.
    ///
    /// If `new_len > capacity()`, the capacity will be automatically increased.
    ///
    /// Equivalent to `SCARDC`.
    pub fn resize(&mut self, new_len: usize, value: &str) {
        if new_len > self.capacity() {
            self.reserve(new_len - self.capacity());
        }
        for i in self.len()..new_len {
            self.set(i, value);
        }
        self.set_usize(MD_CARD, new_len);

        self.validate();
    }

    /// Converts to `CharArray` for use in the `raw` API.
    pub fn as_arg(&self) -> CharArray {
        self.validate();
        CharArray::new(&self.data, self.element_length)
    }

    /// Converts to `CharArrayMut` for use in the `raw` API.
    pub fn as_arg_mut(&mut self) -> CharArrayMut {
        self.validate();
        CharArrayMut::new(&mut self.data, self.element_length)
    }

    /// Detects internal errors in the data structure.
    pub(crate) fn validate(&self) {
        assert!(self.len() <= self.capacity());
        assert_eq!(
            self.data.len(),
            (METADATA + self.capacity()) * self.element_length
        );
    }

    /// Gets a string from the cell.
    ///
    /// Trailing blanks will be stripped.
    /// Requires `index < capacity()`.
    pub fn get(&self, index: usize) -> &str {
        let b = &self.data[(METADATA + index) * self.element_length
            ..(METADATA + index + 1) * self.element_length];

        // Trim trailing blanks
        let b = if let Some(end) = b.iter().rposition(|c| *c != b' ') {
            &b[..=end]
        } else {
            b""
        };

        std::str::from_utf8(b).expect("CharCell contains invalid UTF-8 bytes")
    }

    /// Stores a string in the cell.
    ///
    /// If the string is longer than `element_length`, it will be truncated.
    /// Requires `index < capacity()`.
    pub fn set(&mut self, index: usize, value: &str) {
        let b = &mut self.data[(METADATA + index) * self.element_length
            ..(METADATA + index + 1) * self.element_length];

        f2rust_std::fstr::assign(b, value.as_bytes());
    }

    /// Returns an iterator over the cell's data, up to `len`.
    pub fn iter(&self) -> impl Iterator<Item = &str> {
        (0..self.len()).map(|i| self.get(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Result, SpiceContext};

    #[test]
    fn test_i() {
        let mut cell = Cell::with_capacity(20);
        cell.validate();
        assert_eq!(cell.capacity(), 20);
        assert_eq!(cell.len(), 0);
        assert_eq!(cell[0], 0);

        cell[0] += 100;
        assert_eq!(cell[0], 100);

        cell.push(200);
        assert_eq!(cell[0], 200);
        assert_eq!(cell.len(), 1);

        cell.resize(19, 300);
        assert_eq!(cell.capacity(), 20);
        assert_eq!(cell.len(), 19);
        assert_eq!(cell[18], 300);

        cell.push(400);
        assert_eq!(cell.capacity(), 20);
        assert_eq!(cell.len(), 20);
        assert_eq!(cell[19], 400);

        cell.push(500);
        assert_eq!(cell[20], 500);
        assert_eq!(cell.capacity(), 21);
        assert_eq!(cell.len(), 21);

        cell.resize(10, 0);
        assert_eq!(cell.capacity(), 21);
        assert_eq!(cell.len(), 10);

        assert_eq!(
            cell.into_iter().collect::<Vec<_>>(),
            [200, 300, 300, 300, 300, 300, 300, 300, 300, 300]
        );
    }

    #[test]
    fn test_d() {
        let mut cell = Cell::with_capacity(20);
        assert_eq!(cell.capacity(), 20);
        assert_eq!(cell.len(), 0);
        assert_eq!(cell[0], 0.0);

        cell[0] += 100.0;
        assert_eq!(cell[0], 100.0);

        cell.push(200.0);
        assert_eq!(cell[0], 200.0);
        assert_eq!(cell.len(), 1);

        assert_eq!(cell.into_iter().collect::<Vec<_>>(), [200.0]);
    }

    #[test]
    fn test_c() {
        let mut cell = CharCell::with_capacity_and_length(20, 15);
        assert_eq!(cell.capacity(), 20);
        assert_eq!(cell.len(), 0);
        assert_eq!(cell.element_length(), 15);
        assert_eq!(cell.get(0), "");

        cell.set(0, "aaa");
        assert_eq!(cell.get(0), "aaa");

        cell.push("b");
        assert_eq!(cell.get(0), "b");
        assert_eq!(cell.len(), 1);

        cell.resize(19, "c");
        assert_eq!(cell.capacity(), 20);
        assert_eq!(cell.len(), 19);
        assert_eq!(cell.get(18), "c");

        cell.push("d");
        assert_eq!(cell.capacity(), 20);
        assert_eq!(cell.len(), 20);
        assert_eq!(cell.get(19), "d");

        cell.push("e");
        assert_eq!(cell.get(20), "e");
        assert_eq!(cell.capacity(), 21);
        assert_eq!(cell.len(), 21);

        cell.resize(10, "");
        assert_eq!(cell.capacity(), 21);
        assert_eq!(cell.len(), 10);

        assert_eq!(
            cell.iter().collect::<Vec<_>>(),
            ["b", "c", "c", "c", "c", "c", "c", "c", "c", "c"]
        );
    }

    #[test]
    fn test_interop_i() -> Result<()> {
        let mut spice = SpiceContext::new();
        let mut cell = Cell::with_capacity(20);

        assert_eq!(spice.cardi(&cell)?, 0);
        assert_eq!(spice.sizei(&cell)?, 20);
        spice.appndi(100, &mut cell)?;
        assert_eq!(cell.len(), 1);
        assert_eq!(cell[0], 100);

        Ok(())
    }

    #[test]
    fn test_interop_d() -> Result<()> {
        let mut spice = SpiceContext::new();
        let mut cell = Cell::with_capacity(20);

        assert_eq!(spice.cardd(&cell)?, 0);
        assert_eq!(spice.sized(&cell)?, 20);
        spice.appndd(100.0, &mut cell)?;
        assert_eq!(cell.len(), 1);
        assert_eq!(cell[0], 100.0);

        Ok(())
    }

    #[test]
    fn test_interop_c() -> Result<()> {
        let mut spice = SpiceContext::new();
        let mut cell = CharCell::with_capacity_and_length(20, 15);

        assert_eq!(cell.capacity(), 20);
        assert_eq!(cell.len(), 0);

        assert_eq!(spice.cardc(&cell)?, 0);
        assert_eq!(spice.sizec(&cell)?, 20);
        spice.appndc("Hello world", &mut cell)?;
        assert_eq!(cell.len(), 1);
        assert_eq!(cell.get(0), "Hello world");

        cell.set(0, "Test");
        assert_eq!(cell.get(0), "Test");

        spice.ssizec(0x12345678, &mut cell)?;
        assert_eq!(cell.capacity(), 0x12345678);

        Ok(())
    }
}
