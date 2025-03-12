use super::{AssertionFailure, DescriptiveSpec, Spec};

use std::borrow::Borrow;

pub trait StrAssertions<T> {
    #[track_caller]
    fn starts_with<E: AsRef<str>>(&mut self, expected: E);
    #[track_caller]
    fn ends_with<E: AsRef<str>>(&mut self, expected: E);
    #[track_caller]
    fn contains<E: AsRef<str>>(&mut self, expected: E);
    #[track_caller]
    fn does_not_contain<E: AsRef<str>>(&mut self, expected: E);
    #[track_caller]
    fn is_empty(&mut self);
}

impl<T> StrAssertions<T> for Spec<'_, T>
where
    T: AsRef<str>,
{
    /// Asserts that the subject `&str` starts with the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello").starts_with("H");
    /// ```
    fn starts_with<E: AsRef<str>>(&mut self, expected: E) {
        let subject = self.subject.as_ref();
        starts_with(self, subject, expected.as_ref());
    }

    /// Asserts that the subject `&str` ends with the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello").ends_with("o");
    /// ```
    fn ends_with<E: AsRef<str>>(&mut self, expected: E) {
        let subject = self.subject.as_ref();
        ends_with(self, subject, expected.as_ref());
    }

    /// Asserts that the subject `&str` contains the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello").contains("e");
    /// ```
    fn contains<E: AsRef<str>>(&mut self, expected: E) {
        let subject = self.subject.as_ref();
        contains(self, subject, expected.as_ref());
    }

    /// Asserts that the subject `&str` contains the provided `&str`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"Hello").contains("e");
    /// ```
    fn does_not_contain<E: AsRef<str>>(&mut self, expected: E) {
        let subject = self.subject.as_ref();
        does_not_contain(self, subject, expected.as_ref());
    }

    /// Asserts that the subject `&str` is empty.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// assert_that(&"").is_empty();
    /// ```
    fn is_empty(&mut self) {
        let subject = self.subject.as_ref();
        is_empty(self, subject);
    }
}

fn starts_with<'r, 's, S: DescriptiveSpec<'s>, E: Borrow<&'r str>>(
    spec: &'s S,
    subject: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if !subject.starts_with(borrowed_expected) {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("string starting with <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", subject))
            .fail();
    }
}

fn ends_with<'r, 's, S: DescriptiveSpec<'s>, E: Borrow<&'r str>>(
    spec: &'s S,
    subject: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if !subject.ends_with(borrowed_expected) {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("string ending with <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", subject))
            .fail();
    }
}

fn contains<'r, 's, S: DescriptiveSpec<'s>, E: Borrow<&'r str>>(
    spec: &'s S,
    subject: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if !subject.contains(borrowed_expected) {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("string containing <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", subject))
            .fail();
    }
}

fn does_not_contain<'r, 's, S: DescriptiveSpec<'s>, E: Borrow<&'r str>>(
    spec: &'s S,
    subject: &str,
    expected: E,
) {
    let borrowed_expected = expected.borrow();

    if subject.contains(borrowed_expected) {
        AssertionFailure::from_spec(spec)
            .with_expected(format!("string not containing <{:?}>", borrowed_expected))
            .with_actual(format!("<{:?}>", subject))
            .fail();
    }
}

fn is_empty<'s, S: DescriptiveSpec<'s>>(spec: &'s S, subject: &str) {
    if !subject.is_empty() {
        AssertionFailure::from_spec(spec)
            .with_expected("an empty string".to_string())
            .with_actual(format!("<{:?}>", subject))
            .fail();
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_borrows_for_generic_args)]
    #![allow(clippy::unnecessary_to_owned)]
    use super::super::prelude::*;
    use std::borrow::Cow;

    #[test]
    fn should_allow_multiple_borrow_forms_for_str() {
        let value = "Hello";
        assert_that(&value).starts_with("H");
        assert_that(&value).starts_with(&mut "H");
        assert_that(&value).starts_with(&"H");
        assert_that(&value).starts_with(Cow::from("H"));
        assert_that(&value).starts_with("H".to_string());

        assert_that(&value).ends_with("o");
        assert_that(&value).ends_with(&mut "o");
        assert_that(&value).ends_with(&"o");
        assert_that(&value).ends_with(Cow::from("o"));
        assert_that(&value).ends_with("o".to_string());

        assert_that(&value).contains("l");
        assert_that(&value).contains(&mut "l");
        assert_that(&value).contains(&"l");
        assert_that(&value).contains(Cow::from("l"));
        assert_that(&value).contains("l".to_string());
    }

    #[test]
    fn should_not_panic_if_str_starts_with_value() {
        let value = "Hello";
        assert_that(&value).starts_with("H");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string starting with <\"A\">\
                   \n\t but was: <\"Hello\">")]
    fn should_panic_if_str_does_not_start_with_value() {
        let value = "Hello";
        assert_that(&value).starts_with("A");
    }

    #[test]
    fn should_not_panic_if_str_ends_with_value() {
        let value = "Hello";
        assert_that(&value).ends_with("o");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string ending with <\"A\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_str_does_not_end_with_value() {
        let value = "Hello";
        assert_that(&value).ends_with("A");
    }

    #[test]
    fn should_not_panic_if_str_contains_value() {
        let value = "Hello";
        assert_that(&value).contains("l");
    }

    #[test]
    fn should_not_panic_if_str_does_not_contain_value() {
        let value = "Hello";
        assert_that(&value).does_not_contain("x");
    }

    #[test]
    #[should_panic(
        expected = "\n\texpected: string not containing <\"l\">\n\t but was: <\"Hello\">"
    )]
    fn should_panic_if_str_contains_value() {
        let value = "Hello";
        assert_that(&value).does_not_contain("l");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string containing <\"A\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_str_does_not_contain_value() {
        let value = "Hello";
        assert_that(&value).contains("A");
    }

    #[test]
    fn should_not_panic_if_str_is_empty() {
        let value = "";
        assert_that(&value).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty string\n\t but was: <\"Hello\">")]
    fn should_panic_if_str_is_not_empty() {
        let value = "Hello";
        assert_that(&value).is_empty();
    }

    #[test]
    fn should_allow_multiple_borrow_forms_for_string() {
        let value = "Hello".to_owned();
        assert_that(&value).starts_with("H");
        assert_that(&value).starts_with(&mut "H");
        assert_that(&value).starts_with(&"H");
        assert_that(&value).starts_with(Cow::from("H"));
        assert_that(&value).starts_with("H".to_string());

        assert_that(&value).ends_with("o");
        assert_that(&value).ends_with(&mut "o");
        assert_that(&value).ends_with(&"o");
        assert_that(&value).ends_with(Cow::from("o"));
        assert_that(&value).ends_with("o".to_string());

        assert_that(&value).contains("l");
        assert_that(&value).contains(&mut "l");
        assert_that(&value).contains(&"l");
        assert_that(&value).contains(Cow::from("l"));
        assert_that(&value).contains("l".to_string());
    }

    #[test]
    fn should_not_panic_if_string_starts_with_value() {
        let value = "Hello".to_owned();
        assert_that(&value).starts_with("H");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string starting with <\"A\">\
                   \n\t but was: <\"Hello\">")]
    fn should_panic_if_string_does_not_start_with_value() {
        let value = "Hello".to_owned();
        assert_that(&value).starts_with("A");
    }

    #[test]
    fn should_not_panic_if_string_ends_with_value() {
        let value = "Hello".to_owned();
        assert_that(&value).ends_with("o");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string ending with <\"A\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_string_does_not_end_with_value() {
        let value = "Hello".to_owned();
        assert_that(&value).ends_with("A");
    }

    #[test]
    fn should_not_panic_if_string_contains_value() {
        let value = "Hello".to_owned();
        assert_that(&value).contains("l");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: string containing <\"A\">\n\t but was: <\"Hello\">")]
    fn should_panic_if_string_does_not_contain_value() {
        let value = "Hello".to_owned();
        assert_that(&value).contains("A");
    }

    #[test]
    fn should_not_panic_if_string_is_empty() {
        let value = "".to_owned();
        assert_that(&value).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty string\n\t but was: <\"Hello\">")]
    fn should_panic_if_string_is_not_empty() {
        let value = "Hello".to_owned();
        assert_that(&value).is_empty();
    }
}
