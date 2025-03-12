use super::{AssertionFailure, Spec};

pub trait BooleanAssertions {
    #[track_caller]
    fn is_true(&mut self);
    #[track_caller]
    fn is_false(&mut self);
}

impl<'s> BooleanAssertions for Spec<'s, bool> {
    /// Asserts that the subject is true. The subject type must be `bool`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&true).is_true();
    /// ```
    fn is_true(&mut self) {
        if !*self.subject {
            AssertionFailure::from_spec(self)
                .with_expected("bool to be <true>".to_string())
                .with_actual("<false>".to_string())
                .fail();
        }
    }

    /// Asserts that the subject is false. The subject type must be `bool`.
    ///
    /// ```rust, should_panic
    /// # use speculoos::prelude::*;
    /// assert_that(&true).is_false();
    /// ```
    fn is_false(&mut self) {
        if *self.subject {
            AssertionFailure::from_spec(self)
                .with_expected("bool to be <false>".to_string())
                .with_actual("<true>".to_string())
                .fail();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    #[test]
    pub fn should_not_panic_if_value_is_expected_to_be_true_and_is() {
        assert_that(&true).is_true();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: bool to be <true>\n\t but was: <false>")]
    pub fn should_panic_if_value_is_expected_to_be_true_and_is_not() {
        assert_that(&false).is_true();
    }

    #[test]
    pub fn should_not_panic_if_value_is_expected_to_be_false_and_is() {
        assert_that(&false).is_false();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: bool to be <false>\n\t but was: <true>")]
    pub fn should_panic_if_value_is_expected_to_be_false_and_is_not() {
        assert_that(&true).is_false();
    }
}
