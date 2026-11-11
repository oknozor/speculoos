use crate::{slice, Spec};

/// Deprecated alias for slice length assertions on `Vec` subjects.
///
/// This trait is retained so that code importing `speculoos::vec::VecAssertions` directly
/// continues to compile. New code should use `SliceAssertions`, which also supports
/// `Box<[T]>`, `Rc<[T]>`, `Arc<[T]>`, arrays, and borrowed slices.
///
/// Note that this trait is not part of the prelude. Importing both this trait and the prelude
/// makes `has_length`, `is_empty`, and `is_not_empty` ambiguous for `Vec` subjects.
///
/// ```compile_fail
/// # use speculoos::prelude::*;
/// # #[allow(deprecated)]
/// # use speculoos::vec::VecAssertions;
/// assert_that(&vec![1, 2, 3]).has_length(3);
/// ```
///
/// Using this trait produces a deprecation warning:
///
/// ```compile_fail
/// #![deny(deprecated)]
/// # use speculoos::assert_that;
/// # use speculoos::vec::VecAssertions;
/// assert_that(&vec![1, 2, 3]).has_length(3);
/// ```
///
/// The trait is not re-exported from the prelude:
///
/// ```compile_fail
/// use speculoos::prelude::VecAssertions;
/// ```
#[deprecated(
    since = "0.13.0",
    note = "use `speculoos::slice::SliceAssertions` instead"
)]
pub trait VecAssertions {
    #[track_caller]
    fn has_length(&mut self, expected: usize);
    #[track_caller]
    fn is_empty(&mut self);
    #[track_caller]
    fn is_not_empty(&mut self);
}

#[allow(deprecated)]
impl<T> VecAssertions for Spec<'_, Vec<T>> {
    fn has_length(&mut self, expected: usize) {
        <Self as slice::SliceAssertions>::has_length(self, expected)
    }

    fn is_empty(&mut self) {
        <Self as slice::SliceAssertions>::is_empty(self)
    }

    fn is_not_empty(&mut self) {
        <Self as slice::SliceAssertions>::is_not_empty(self)
    }
}

#[allow(deprecated)]
impl<'s, T> VecAssertions for Spec<'s, &'s Vec<T>> {
    fn has_length(&mut self, expected: usize) {
        <Self as slice::SliceAssertions>::has_length(self, expected)
    }

    fn is_empty(&mut self) {
        <Self as slice::SliceAssertions>::is_empty(self)
    }

    fn is_not_empty(&mut self) {
        <Self as slice::SliceAssertions>::is_not_empty(self)
    }
}

#[cfg(test)]
mod tests {
    use super::super::prelude::*;

    #[test]
    fn should_not_panic_if_vec_length_matches_expected() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).has_length(3);
        assert_that(&&test_vec).has_length(3);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: slice to have length <1>\n\t but was: <3>")]
    fn should_panic_if_vec_length_does_not_match_expected() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).has_length(1);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: slice to have length <1>\n\t but was: <3>")]
    fn should_panic_if_ref_vec_length_does_not_match_expected() {
        let test_vec = vec![1, 2, 3];
        assert_that(&&test_vec).has_length(1);
    }

    #[test]
    fn should_not_panic_if_vec_was_expected_to_be_empty_and_is() {
        let test_vec: Vec<u8> = vec![];
        assert_that(&test_vec).is_empty();
        assert_that(&&test_vec).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty slice\
                   \n\t but was: a slice with length <1>")]
    fn should_panic_if_vec_was_expected_to_be_empty_and_is_not() {
        assert_that(&vec![1]).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty slice\
                   \n\t but was: a slice with length <1>")]
    fn should_panic_if_ref_vec_was_expected_to_be_empty_and_is_not() {
        assert_that(&&vec![1]).is_empty();
    }

    #[test]
    fn should_not_panic_if_vec_was_expected_to_be_not_empty_and_is() {
        let test_vec: Vec<u8> = vec![1];
        assert_that(&test_vec).is_not_empty();
        assert_that(&&test_vec).is_not_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: a non empty slice\
                   \n\t but was: an empty slice")]
    fn should_panic_if_vec_was_expected_to_be_not_empty_and_is_not() {
        let test_vec: Vec<u8> = vec![];
        assert_that(&test_vec).is_not_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: a non empty slice\
                   \n\t but was: an empty slice")]
    fn should_panic_if_ref_vec_was_expected_to_be_not_empty_and_is_not() {
        let test_vec: Vec<u8> = vec![];
        assert_that(&&test_vec).is_not_empty();
    }
}
