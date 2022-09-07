use super::{AssertionFailure, Spec};

use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub trait HashSetAssertions<'s> {
    fn has_length(&mut self, expected: usize);
    fn is_empty(&mut self);
}

impl<'s, K> HashSetAssertions<'s> for Spec<'s, HashSet<K>>
where
    K: Hash + Eq + Debug,
{
    /// Asserts that the length of the subject HashSet is equal to the provided length. The subject
    /// type must be of `HashSet`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// # use std::collections::HashSet;
    /// let mut test_map = HashSet::new();
    /// test_map.insert(1);
    /// test_map.insert(2);
    ///
    /// assert_that(&test_map).has_length(2);
    /// ```
    fn has_length(&mut self, expected: usize) {
        let subject = self.subject;

        if subject.len() != expected {
            AssertionFailure::from_spec(self)
                .with_expected(format!("HashSet to have length <{}>", expected))
                .with_actual(format!("<{}>", subject.len()))
                .fail();
        }
    }

    /// Asserts that the subject HashSet is empty. The subject type must be of `HashSet`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// # use std::collections::HashSet;
    /// let test_map: HashSet<u8> = HashSet::new();
    /// assert_that(&test_map).is_empty();
    /// ```
    fn is_empty(&mut self) {
        let subject = self.subject;

        if !subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected("an empty HashSet".to_string())
                .with_actual(format!("a HashSet with length <{:?}>", subject.len()))
                .fail();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::super::prelude::*;

    use std::collections::HashSet;

    #[test]
    fn should_not_panic_if_HashSet_length_matches_expected() {
        let mut test_map = HashSet::new();
        test_map.insert(1);
        test_map.insert(2);

        assert_that(&test_map).has_length(2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: HashSet to have length <1>\n\t but was: <2>")]
    fn should_panic_if_HashSet_length_does_not_match_expected() {
        let mut test_map = HashSet::new();
        test_map.insert(1);
        test_map.insert(2);

        assert_that(&test_map).has_length(1);
    }

    #[test]
    fn should_not_panic_if_HashSet_was_expected_to_be_empty_and_is() {
        let test_map: HashSet<u8> = HashSet::new();
        assert_that(&test_map).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty HashSet\
                   \n\t but was: a HashSet with length <1>")]
    fn should_panic_if_HashSet_was_expected_to_be_empty_and_is_not() {
        let mut test_map = HashSet::new();
        test_map.insert(1);

        assert_that(&test_map).is_empty();
    }

    #[test]
    fn contains_should_allow_multiple_borrow_forms() {
        let mut test_map = HashSet::new();
        test_map.insert("hello");

        assert_that(&test_map).contains("hello");
        assert_that(&test_map).contains(&mut "hello");
        assert_that(&test_map).contains(&"hello");
    }

    #[test]
    fn should_not_panic_if_HashSet_contains() {
        let mut test_map = HashSet::new();
        test_map.insert("hello");

        assert_that(&test_map).contains(&"hello");
    }

    #[test]
    // Unfortunately the order of the keys can change. Doesn't seem to make sense to sort them
    // just for the sake of checking the panic message.
    #[should_panic]
    fn should_not_panic_if_HashSet_does_not_contain() {
        let mut test_map = HashSet::new();
        test_map.insert("hi");
        test_map.insert("hey");

        assert_that(&test_map).contains(&"hello");
    }

    #[test]
    fn does_not_contain_should_allow_multiple_borrow_forms() {
        let mut test_map = HashSet::new();
        test_map.insert("hello");

        assert_that(&test_map).does_not_contain("hey");
        assert_that(&test_map).does_not_contain(&mut "hey");
        assert_that(&test_map).does_not_contain(&"hey");
    }

    #[test]
    fn should_not_panic_if_HashSet_does_not_contain_when_expected() {
        let mut test_map = HashSet::new();
        test_map.insert("hello");

        assert_that(&test_map).does_not_contain(&"hey");
    }

    #[test]
    #[should_panic(expected = "\n\texpected: iterator to not contain <\"hello\">\
                   \n\t but was: <[\"hello\"]>")]
    fn should_panic_if_HashSet_does_contain_when_not_expected() {
        let mut test_map = HashSet::new();
        test_map.insert("hello");

        assert_that(&test_map).does_not_contain(&"hello");
    }
}
