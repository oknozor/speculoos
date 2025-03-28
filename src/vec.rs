use super::{AssertionFailure, Spec};

pub trait VecAssertions {
    #[track_caller]
    fn has_length(&mut self, expected: usize);
    #[track_caller]
    fn is_empty(&mut self);
    #[track_caller]
    fn is_not_empty(&mut self);
}

impl<T> VecAssertions for Spec<'_, Vec<T>> {
    /// Asserts that the length of the subject vector is equal to the provided length. The subject
    /// type must be of `Vec`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that!(vec![1, 2, 3, 4]).has_length(4);
    /// ```
    fn has_length(&mut self, expected: usize) {
        let length = self.subject.len();
        if length != expected {
            AssertionFailure::from_spec(self)
                .with_expected(format!("vec to have length <{}>", expected))
                .with_actual(format!("<{}>", length))
                .fail();
        }
    }

    /// Asserts that the subject vector is empty. The subject type must be of `Vec`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_vec: Vec<u8> = vec![];
    /// assert_that!(test_vec).is_empty();
    /// ```
    fn is_empty(&mut self) {
        let subject = self.subject;

        if !subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected("an empty vec".to_string())
                .with_actual(format!("a vec with length <{:?}>", subject.len()))
                .fail();
        }
    }

    /// Asserts that the subject vector is not empty. The subject type must be of `Vec`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_vec: Vec<u8> = vec![1];
    /// assert_that!(test_vec).is_not_empty();
    /// ```
    fn is_not_empty(&mut self) {
        let subject = self.subject;

        if subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected("a non empty vec".to_string())
                .with_actual("an empty vec".to_string())
                .fail();
        }
    }
}

impl<'s, T> VecAssertions for Spec<'s, &'s Vec<T>> {
    /// Asserts that the length of the subject vector is equal to the provided length. The subject
    /// type must be of `&Vec` with a matching lifetime.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&&vec![1, 2, 3, 4]).has_length(4);
    /// ```
    fn has_length(&mut self, expected: usize) {
        let length = self.subject.len();
        if length != expected {
            AssertionFailure::from_spec(self)
                .with_expected(format!("vec to have length <{}>", expected))
                .with_actual(format!("<{}>", length))
                .fail();
        }
    }

    /// Asserts that the subject vector is empty. The subject type must be of `&Vec` with a
    /// matching lifetime.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_vec: &Vec<u8> = &vec![];
    /// assert_that(&test_vec).is_empty();
    /// ```
    fn is_empty(&mut self) {
        let subject = self.subject;

        if !subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected("an empty vec".to_string())
                .with_actual(format!("a vec with length <{:?}>", subject.len()))
                .fail();
        }
    }

    /// Asserts that the subject vector is not empty. The subject type must be of `&Vec` with a
    /// matching lifetime.
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_vec: Vec<u8> = vec![1];
    /// assert_that(&test_vec).is_not_empty();
    /// ```
    fn is_not_empty(&mut self) {
        let subject = self.subject;

        if subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected("a non empty vec".to_string())
                .with_actual("an empty vec".to_string())
                .fail();
        }
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
    #[should_panic(expected = "\n\texpected: vec to have length <1>\n\t but was: <3>")]
    fn should_panic_if_vec_length_does_not_match_expected() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).has_length(1);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: vec to have length <1>\n\t but was: <3>")]
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
    #[should_panic(expected = "\n\texpected: an empty vec\
                   \n\t but was: a vec with length <1>")]
    fn should_panic_if_vec_was_expected_to_be_empty_and_is_not() {
        assert_that(&vec![1]).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty vec\
                   \n\t but was: a vec with length <1>")]
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
    #[should_panic(expected = "\n\texpected: a non empty vec\
                   \n\t but was: an empty vec")]
    fn should_panic_if_vec_was_expected_to_be_not_empty_and_is_not() {
        let test_vec: Vec<u8> = vec![];
        assert_that(&test_vec).is_not_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: a non empty vec\
                   \n\t but was: an empty vec")]
    fn should_panic_if_ref_vec_was_expected_to_be_not_empty_and_is_not() {
        let test_vec: Vec<u8> = vec![];
        assert_that(&&test_vec).is_not_empty();
    }
}
