use super::{AssertionFailure, Spec};

use std::borrow::Borrow;
use std::fmt::Debug;

pub trait ResultAssertions<'s, T, E>
where
    T: Debug,
    E: Debug,
{
    #[track_caller]
    fn is_ok(&mut self) -> Spec<'s, T>;
    #[track_caller]
    fn is_err(&mut self) -> Spec<'s, E>;
}

pub trait ContainingResultAssertions<T, E>
where
    T: Debug,
    E: Debug,
{
    #[track_caller]
    fn is_ok_containing<V: Borrow<T>>(&mut self, expected_value: V)
    where
        T: PartialEq;
    #[track_caller]
    fn is_err_containing<V: Borrow<E>>(&mut self, expected_value: V)
    where
        E: PartialEq;
}

impl<T, E> ContainingResultAssertions<T, E> for Spec<'_, Result<T, E>>
where
    T: Debug,
    E: Debug,
{
    /// Asserts that the subject is an `Ok` Result containing the expected value.
    /// The subject type must be a `Result`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&Result::Ok::<usize, usize>(1)).is_ok_containing(&1);
    /// ```
    fn is_ok_containing<V: Borrow<T>>(&mut self, expected_value: V)
    where
        T: PartialEq,
    {
        let borrowed_expected_value = expected_value.borrow();

        match *self.subject {
            Ok(ref val) => {
                if !val.eq(borrowed_expected_value) {
                    AssertionFailure::from_spec(self)
                        .with_expected(build_detail_message("ok", borrowed_expected_value))
                        .with_actual(build_detail_message("ok", val))
                        .fail();
                }
            }
            Err(ref val) => {
                AssertionFailure::from_spec(self)
                    .with_expected(build_detail_message("ok", borrowed_expected_value))
                    .with_actual(build_detail_message("err", val))
                    .fail();
            }
        }
    }

    /// Asserts that the subject is an `Err` Result containing the expected value.
    /// The subject type must be a `Result`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&Result::Err::<usize, usize>(1)).is_err_containing(&1);
    /// ```
    fn is_err_containing<V: Borrow<E>>(&mut self, expected_value: V)
    where
        E: PartialEq,
    {
        let borrowed_expected_value = expected_value.borrow();

        match *self.subject {
            Err(ref val) => {
                if !val.eq(borrowed_expected_value) {
                    AssertionFailure::from_spec(self)
                        .with_expected(build_detail_message("err", borrowed_expected_value))
                        .with_actual(build_detail_message("err", val))
                        .fail();
                }
            }
            Ok(ref val) => {
                AssertionFailure::from_spec(self)
                    .with_expected(build_detail_message("err", borrowed_expected_value))
                    .with_actual(build_detail_message("ok", val))
                    .fail();
            }
        }
    }
}

fn build_detail_message<T: Debug>(variant: &'static str, value: T) -> String {
    format!("Result[{}] containing <{:?}>", variant, value)
}

impl<'s, T, E> ResultAssertions<'s, T, E> for Spec<'s, Result<T, E>>
where
    T: Debug,
    E: Debug,
{
    /// Asserts that the subject is `Ok`. The value type must be a `Result`.
    ///
    /// This will return a new `Spec` containing the unwrapped value if it is `Ok`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&Result::Ok::<usize, usize>(1)).is_ok();
    /// ```
    fn is_ok(&mut self) -> Spec<'s, T> {
        match *self.subject {
            Ok(ref val) => Spec {
                subject: val,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            },
            Err(ref err) => {
                AssertionFailure::from_spec(self)
                    .with_expected("result[ok]".to_string())
                    .with_actual(format!("result[error]<{:?}>", err))
                    .fail();

                unreachable!();
            }
        }
    }

    /// Asserts that the subject is `Err`. The value type must be a `Result`.
    ///
    /// This will return a new `Spec` containing the unwrapped value if it is `Err`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&Result::Err::<usize, usize>(1)).is_err();
    /// ```
    fn is_err(&mut self) -> Spec<'s, E> {
        match *self.subject {
            Err(ref val) => Spec {
                subject: val,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            },
            Ok(ref val) => {
                AssertionFailure::from_spec(self)
                    .with_expected("result[error]".to_string())
                    .with_actual(format!("result[ok]<{:?}>", val))
                    .fail();

                unreachable!();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_borrows_for_generic_args)]
    use super::super::prelude::*;

    #[test]
    fn should_not_panic_if_result_is_expected_to_be_ok_and_is() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_ok();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: result[ok]\n\t but was: result[error]<\"Oh no\">")]
    fn should_panic_if_result_is_expected_to_be_ok_and_is_not() {
        let result: Result<&str, &str> = Err("Oh no");
        assert_that(&result).is_ok();
    }

    #[test]
    fn should_return_unwrapped_value_if_subject_is_ok() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_ok().is_equal_to(&"Hello");
    }

    #[test]
    fn should_not_panic_if_result_is_expected_to_be_error_and_is() {
        let result: Result<&str, &str> = Err("Oh no");
        assert_that(&result).is_err();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: result[error]\n\t but was: result[ok]<\"Hello\">")]
    fn should_panic_if_result_is_expected_to_be_error_and_is_not() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_err();
    }

    #[test]
    fn should_return_unwrapped_value_if_subject_is_err() {
        let result: Result<&str, &str> = Err("Hello");
        assert_that(&result).is_err().is_equal_to(&"Hello");
    }

    #[test]
    fn is_ok_containing_should_allow_multiple_borrow_forms() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_ok_containing("Hello");
        assert_that(&result).is_ok_containing(&mut "Hello");
        assert_that(&result).is_ok_containing(&"Hello");
    }

    #[test]
    fn should_not_panic_if_result_is_ok_with_expected_value() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_ok_containing(&"Hello");
    }

    #[test]
    fn should_not_panic_if_result_is_ok_with_uncomparable_ok() {
        #[derive(Debug)]
        struct Incomparable;
        let result: Result<&str, Incomparable> = Ok("Hello");
        assert_that(&result).is_ok_containing(&"Hello");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: Result[ok] containing <\"Hi\">\
                   \n\t but was: Result[ok] containing <\"Hello\">")]
    fn should_panic_if_result_is_ok_without_expected_value() {
        let result: Result<&str, &str> = Ok("Hello");
        assert_that(&result).is_ok_containing(&"Hi");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: Result[ok] containing <\"Hi\">\
                   \n\t but was: Result[err] containing <\"Hi\">")]
    fn should_panic_if_result_is_err_if_ok_with_value_expected() {
        let result: Result<&str, &str> = Err("Hi");
        assert_that(&result).is_ok_containing(&"Hi");
    }

    #[test]
    fn is_error_containing_should_allow_multiple_borrow_forms() {
        let result: Result<&str, &str> = Err("Oh no");
        assert_that(&result).is_err_containing("Oh no");
        assert_that(&result).is_err_containing(&mut "Oh no");
        assert_that(&result).is_err_containing(&"Oh no");
    }

    #[test]
    fn should_not_panic_if_result_is_err_with_expected_value() {
        let result: Result<&str, &str> = Err("Oh no");
        assert_that(&result).is_err_containing(&"Oh no");
    }

    #[test]
    fn should_not_panic_if_result_is_err_with_uncomparable_ok() {
        #[derive(Debug)]
        struct Incomparable;
        let result: Result<Incomparable, &str> = Err("Oh no");
        assert_that(&result).is_err_containing(&"Oh no");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: Result[err] containing <\"Oh no\">\
                   \n\t but was: Result[err] containing <\"Whoops\">")]
    fn should_panic_if_result_is_err_without_expected_value() {
        let result: Result<&str, &str> = Err("Whoops");
        assert_that(&result).is_err_containing(&"Oh no");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: Result[err] containing <\"Oh no\">\
                   \n\t but was: Result[ok] containing <\"Oh no\">")]
    fn should_panic_if_result_is_ok_if_err_with_value_expected() {
        let result: Result<&str, &str> = Ok("Oh no");
        assert_that(&result).is_err_containing(&"Oh no");
    }
}
