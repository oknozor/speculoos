use super::{AssertionFailure, Spec};

use std::rc::Rc;
use std::sync::Arc;

pub trait SliceAssertions {
    #[track_caller]
    fn has_length(&mut self, expected: usize);
    #[track_caller]
    fn is_empty(&mut self);
    #[track_caller]
    fn is_not_empty(&mut self);
}

impl<S> SliceAssertions for Spec<'_, S>
where
    S: SliceLike,
{
    /// Asserts that the length of the subject slice is equal to the provided length. The subject
    /// type must be of any type that implements `SliceLike`, such as `Box<[T]>`, `Rc<[T]>`,
    /// `Arc<[T]>`, or `[T]`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_slice: Box<[u8]> = Box::new([1, 2, 3, 4]);
    /// assert_that(&test_slice).has_length(4);
    /// ```
    fn has_length(&mut self, expected: usize) {
        let length = self.subject.as_slice().len();
        if length != expected {
            AssertionFailure::from_spec(self)
                .with_expected(format!("slice to have length <{}>", expected))
                .with_actual(format!("<{}>", length))
                .fail();
        }
    }

    /// Asserts that the subject slice is empty. The subject type must be of any type that
    /// implements `SliceLike`, such as `Box<[T]>`, `Rc<[T]>`, `Arc<[T]>`, or `[T]`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_slice: Box<[u8]> = Box::new([]);
    /// assert_that(&test_slice).is_empty();
    /// ```
    fn is_empty(&mut self) {
        let subject = self.subject.as_slice();

        if !subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected("an empty slice".to_string())
                .with_actual(format!("a slice with length <{:?}>", subject.len()))
                .fail();
        }
    }

    /// Asserts that the subject slice is not empty. The subject type must be of any type that
    /// implements `SliceLike`, such as `Box<[T]>`, `Rc<[T]>`, `Arc<[T]>`, or `[T]`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_slice: Box<[u8]> = Box::new([1]);
    /// assert_that(&test_slice).is_not_empty();
    /// ```
    fn is_not_empty(&mut self) {
        let subject = self.subject.as_slice();

        if subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected("a non empty slice".to_string())
                .with_actual("an empty slice".to_string())
                .fail();
        }
    }
}

pub trait SliceLike {
    type Item;

    fn as_slice(&self) -> &[Self::Item];
}

impl<T> SliceLike for [T] {
    type Item = T;

    fn as_slice(&self) -> &[T] {
        self
    }
}

impl<T, const N: usize> SliceLike for [T; N] {
    type Item = T;

    fn as_slice(&self) -> &[T] {
        self
    }
}

impl<T> SliceLike for &'_ [T] {
    type Item = T;

    fn as_slice(&self) -> &[T] {
        self
    }
}

impl<T> SliceLike for Vec<T> {
    type Item = T;

    fn as_slice(&self) -> &[T] {
        self.as_slice()
    }
}

impl<T> SliceLike for Box<[T]> {
    type Item = T;

    fn as_slice(&self) -> &[T] {
        self
    }
}

impl<T> SliceLike for Rc<[T]> {
    type Item = T;

    fn as_slice(&self) -> &[T] {
        self
    }
}

impl<T> SliceLike for Arc<[T]> {
    type Item = T;

    fn as_slice(&self) -> &[T] {
        self
    }
}

impl<S> SliceLike for &S
where
    S: SliceLike,
{
    type Item = S::Item;

    fn as_slice(&self) -> &[Self::Item] {
        S::as_slice(self)
    }
}

#[cfg(test)]
mod tests {
    use std::rc::Rc;
    use std::sync::Arc;

    use super::super::prelude::*;

    #[test]
    fn should_support_boxed_slices() {
        let test_slice: Box<[u8]> = Box::new([1, 2, 3]);
        assert_that(&test_slice).has_length(3);
        assert_that(&test_slice).is_not_empty();
    }

    #[test]
    fn should_support_reference_counted_slices() {
        let test_slice: Rc<[u8]> = Rc::new([1, 2, 3]);
        assert_that(&test_slice).has_length(3);
        assert_that(&test_slice).is_not_empty();
    }

    #[test]
    fn should_support_atomically_reference_counted_slices() {
        let test_slice: Arc<[u8]> = Arc::new([1, 2, 3]);
        assert_that(&test_slice).has_length(3);
        assert_that(&test_slice).is_not_empty();
    }

    #[test]
    fn should_support_borrowed_slices() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec.as_slice()).has_length(3);
        assert_that(&test_vec.as_slice()).is_not_empty();
    }

    #[test]
    fn should_support_vectors() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).has_length(3);
        assert_that(&test_vec).is_not_empty();
    }

    #[test]
    fn should_not_panic_if_slice_length_matches_expected() {
        let test_slice: Box<[u8]> = Box::new([1, 2, 3]);
        assert_that(&test_slice).has_length(3);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: slice to have length <1>\n\t but was: <3>")]
    fn should_panic_if_slice_length_does_not_match_expected() {
        let test_slice: Box<[u8]> = Box::new([1, 2, 3]);
        assert_that(&test_slice).has_length(1);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: slice to have length <1>\n\t but was: <3>")]
    fn should_panic_if_ref_slice_length_does_not_match_expected() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec.as_slice()).has_length(1);
    }

    #[test]
    fn should_not_panic_if_slice_was_expected_to_be_empty_and_is() {
        let test_slice: Box<[u8]> = Box::new([]);
        assert_that(&test_slice).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty slice\
                   \n\t but was: a slice with length <1>")]
    fn should_panic_if_slice_was_expected_to_be_empty_and_is_not() {
        let test_slice: Box<[u8]> = Box::new([1]);
        assert_that(&test_slice).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty slice\
                   \n\t but was: a slice with length <1>")]
    fn should_panic_if_ref_slice_was_expected_to_be_empty_and_is_not() {
        let test_vec = vec![1];
        assert_that(&test_vec.as_slice()).is_empty();
    }

    #[test]
    fn should_not_panic_if_slice_was_expected_to_be_not_empty_and_is() {
        let test_slice: Box<[u8]> = Box::new([1]);
        assert_that(&test_slice).is_not_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: a non empty slice\
                   \n\t but was: an empty slice")]
    fn should_panic_if_slice_was_expected_to_be_not_empty_and_is_not() {
        let test_slice: Box<[u8]> = Box::new([]);
        assert_that(&test_slice).is_not_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: a non empty slice\
                   \n\t but was: an empty slice")]
    fn should_panic_if_ref_slice_was_expected_to_be_not_empty_and_is_not() {
        let test_vec: Vec<u8> = vec![];
        assert_that(&test_vec.as_slice()).is_not_empty();
    }
}
