use f2rust_std::{CharArray, CharArrayMut};

/// Vector of equal-length strings.
///
/// Similar to `Vec<String>`, but the characters are stored contiguously in memory
/// so they are compatible with FORTRAN CHARACTER arrays.
///
/// For SPICELIB APIs that output to a `&mut CharVec`, you must use `resize`
/// before the call to provide enough space for the results.
#[derive(Clone)]
pub struct CharVec {
    data: Vec<u8>,
    element_length: usize,
}

impl CharVec {
    /// Constructs a vector that can contain strings of the given maximum length.
    pub fn new(element_length: usize) -> Self {
        Self {
            data: Vec::new(),
            element_length,
        }
    }

    /// Returns the maximum string length that can be stored in this vector.
    pub fn element_length(&self) -> usize {
        self.element_length
    }

    /// Returns the number of elements in the vector.
    pub fn len(&self) -> usize {
        self.data.len() / self.element_length
    }

    /// Returns true if `len` is zero.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Pushes a string onto the vector.
    ///
    /// If the string is longer than `element_length`, it will be truncated.
    pub fn push(&mut self, value: &str) {
        self.data.extend(value.bytes().take(self.element_length));
        if value.len() < self.element_length {
            self.data
                .extend(std::iter::repeat_n(b' ', self.element_length - value.len()));
        }
    }

    /// Resizes the vector so that `len() == new_len`.
    ///
    /// If `new_len > len()`, blank strings will be inserted in the new slots.
    pub fn resize(&mut self, new_len: usize) {
        self.data.resize(new_len * self.element_length, b' ');
    }

    /// Converts to `CharArray` for use in the `raw` API.
    pub fn as_arg(&self) -> CharArray {
        CharArray::new(&self.data, self.element_length)
    }

    /// Converts to `CharArrayMut` for use in the `raw` API.
    pub fn as_arg_mut(&mut self) -> CharArrayMut {
        CharArrayMut::new(&mut self.data, self.element_length)
    }

    /// Gets a string from the vector.
    ///
    /// Trailing blanks will be stripped.
    /// Requires `index < len()`.
    pub fn get(&self, index: usize) -> &str {
        let b = &self.data[index * self.element_length..(index + 1) * self.element_length];

        // Trim trailing blanks
        let b = if let Some(end) = b.iter().rposition(|c| *c != b' ') {
            &b[..=end]
        } else {
            b""
        };

        std::str::from_utf8(b).expect("CharVec contains invalid UTF-8 bytes")
    }

    /// Stores a string in the vector.
    ///
    /// If the string is longer than `element_length`, it will be truncated.
    /// Requires `index < len()`.
    pub fn set(&mut self, index: usize, value: &str) {
        let b = &mut self.data[index * self.element_length..(index + 1) * self.element_length];

        f2rust_std::fstr::assign(b, value.as_bytes());
    }

    pub fn iter(&self) -> impl Iterator<Item = &str> {
        (0..self.len()).map(|i| self.get(i))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic() {
        let mut vec = CharVec::new(10);
        assert_eq!(vec.len(), 0);

        vec.push("Hello  ");
        assert_eq!(vec.len(), 1);
        assert_eq!(vec.get(0), "Hello");

        vec.resize(5);
        assert_eq!(vec.len(), 5);
        assert_eq!(vec.get(0), "Hello");
        assert_eq!(vec.get(4), "");

        vec.set(2, "world6789abcdefg");

        assert_eq!(
            vec.iter().collect::<Vec<_>>(),
            ["Hello", "", "world6789a", "", ""]
        );
    }
}
