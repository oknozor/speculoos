use super::{AssertionFailure, Spec};

use std::borrow::Borrow;
use std::cmp::PartialEq;
use std::fmt::Debug;

macro_rules! generate_iter_spec_trait {
    ($trait_name:ident) => {
        pub trait $trait_name<'s, T: 's>
        where
            T: Debug + PartialEq,
        {
            #[track_caller]
            fn contains<E: 's + Borrow<T>>(&mut self, expected_value: E);
            #[track_caller]
            fn contains_all_of<E>(&mut self, expected_values_iter: &'s E)
            where
                E: IntoIterator<Item = &'s T> + Clone;
            #[track_caller]
            fn does_not_contain<E: 's + Borrow<T>>(&mut self, expected_value: E);
            #[track_caller]
            fn equals_iterator<E>(&mut self, expected_iter: &'s E)
            where
                E: Iterator<Item = &'s T> + Clone;
        }
    };
}

generate_iter_spec_trait!(ContainingIntoIterAssertions);
generate_iter_spec_trait!(ContainingIteratorAssertions);

pub trait MappingIterAssertions<'s, T: 's>
where
    T: Debug,
{
    #[track_caller]
    fn matching_contains<F>(&mut self, matcher: F)
    where
        F: Fn(&'s T) -> bool;
    #[track_caller]
    fn mapped_contains<F, M>(&mut self, mapping_function: F, expected_value: &'s M)
    where
        M: Debug + PartialEq,
        F: Fn(&'s T) -> M;
}

impl<'s, T: 's, I> ContainingIntoIterAssertions<'s, T> for Spec<'s, I>
where
    T: Debug + PartialEq,
    &'s I: IntoIterator<Item = &'s T>,
{
    /// Asserts that the subject contains the provided value. The subject must implement
    /// `IntoIterator`, and the contained type must implement `PartialEq` and `Debug`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_vec = vec![1,2,3];
    /// assert_that(&test_vec).contains(&2);
    /// ```
    fn contains<E: 's + Borrow<T>>(&mut self, expected_value: E) {
        let subject_iter = self.subject.into_iter();
        check_iterator_contains(self, subject_iter, expected_value, true);
    }

    /// Asserts that the subject contains all of the provided values. The subject must implement
    /// `IntoIterator`, and the contained type must implement `PartialEq` and `Debug`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_vec = vec![1,2,3];
    /// assert_that(&test_vec).contains_all_of(&vec![&2, &3]);
    /// ```
    fn contains_all_of<E>(&mut self, expected_values_iter: &'s E)
    where
        E: IntoIterator<Item = &'s T> + Clone,
    {
        let subject_iter = self.subject.into_iter();
        let expected_iter = expected_values_iter.clone().into_iter();
        check_iterator_contains_all_of(self, subject_iter, expected_iter);
    }

    /// Asserts that the subject does not contain the provided value. The subject must implement
    /// `IntoIterator`, and the contained type must implement `PartialEq` and `Debug`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_vec = vec![1,2,3];
    /// assert_that(&test_vec).does_not_contain(&4);
    /// ```
    fn does_not_contain<E: 's + Borrow<T>>(&mut self, expected_value: E) {
        let subject_iter = self.subject.into_iter();
        check_iterator_contains(self, subject_iter, expected_value, false);
    }

    /// Asserts that the subject is equal to provided iterator. The subject must implement
    /// `IntoIterator`, the contained type must implement `PartialEq` and `Debug` and the expected
    /// value must implement Iterator and Clone.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let expected_vec = vec![1,2,3];
    /// let test_vec = vec![1,2,3];
    /// assert_that(&test_vec).equals_iterator(&expected_vec.iter());
    /// ```
    fn equals_iterator<E>(&mut self, expected_iter: &'s E)
    where
        E: Iterator<Item = &'s T> + Clone,
    {
        compare_iterators(self, self.subject.into_iter(), expected_iter.clone());
    }
}

impl<'s, T: 's, I> ContainingIteratorAssertions<'s, T> for Spec<'s, I>
where
    T: Debug + PartialEq,
    I: Iterator<Item = &'s T> + Clone,
{
    /// Asserts that the iterable subject contains the provided value. The subject must implement
    /// `Iterator`, and the contained type must implement `PartialEq` and `Debug`.
    ///
    /// ```rust
    /// # use speculoos::prelude::*;
    /// let test_vec = vec![1,2,3];
    /// assert_that(&test_vec.iter()).contains(&2);
    /// ```
    fn contains<E: 's + Borrow<T>>(&mut self, expected_value: E) {
        let subject_iter = self.subject.clone();
        check_iterator_contains(self, subject_iter, expected_value, true);
    }

    /// Asserts that the subject contains all of the provided values. The subject must implement
    /// `Iterator`, and the contained type must implement `PartialEq` and `Debug`.
    ///
    /// ```rust
    /// let test_vec = vec![1,2,3];
    /// # use speculoos::prelude::*;
    /// assert_that(&test_vec.iter()).contains_all_of(&vec![&2, &3]);
    /// ```
    fn contains_all_of<E>(&mut self, expected_values_iter: &'s E)
    where
        E: IntoIterator<Item = &'s T> + Clone,
    {
        let subject_iter = self.subject.clone();
        let expected_iter = expected_values_iter.clone().into_iter();
        check_iterator_contains_all_of(self, subject_iter, expected_iter);
    }

    /// Asserts that the iterable subject does not contain the provided value. The subject must
    /// implement `Iterator`, and the contained type must implement `PartialEq` and `Debug`.
    ///
    /// ```rust
    /// let test_vec = vec![1,2,3];
    /// # use speculoos::prelude::*;
    /// assert_that(&test_vec.iter()).does_not_contain(&4);
    /// ```
    fn does_not_contain<E: 's + Borrow<T>>(&mut self, expected_value: E) {
        let subject_iter = self.subject.clone();
        check_iterator_contains(self, subject_iter, expected_value, false);
    }

    /// Asserts that the iterable subject is equal to provided iterator. The subject must implement
    /// `Iterator`, the contained type must implement `PartialEq` and `Debug` and the expected
    /// value must implement Iterator and Clone.
    ///
    /// ```rust
    /// let expected_vec = vec![1,2,3];
    /// let test_vec = vec![1,2,3];
    /// # use speculoos::prelude::*;
    /// assert_that(&test_vec.iter()).equals_iterator(&expected_vec.iter());
    /// ```
    fn equals_iterator<E>(&mut self, expected_iter: &'s E)
    where
        E: Iterator<Item = &'s T> + Clone,
    {
        compare_iterators(self, self.subject.clone(), expected_iter.clone());
    }
}

impl<'s, T: 's, I> MappingIterAssertions<'s, T> for Spec<'s, I>
where
    T: Debug,
    &'s I: IntoIterator<Item = &'s T>,
{
    /// Asserts that the subject contains a matching item by using the provided function.
    /// The subject must implement `IntoIterator`, and the contained type must implement `Debug`.
    ///
    /// ```rust
    /// # use std::collections::LinkedList;
    /// # use speculoos::prelude::*;
    /// #[derive(Debug, PartialEq)]
    /// # pub enum TestEnum { Good, Bad }
    /// let mut test_into_iter = LinkedList::new();
    /// test_into_iter.push_back(TestEnum::Bad);
    /// test_into_iter.push_back(TestEnum::Good);
    /// test_into_iter.push_back(TestEnum::Bad);
    ///
    /// assert_that(&test_into_iter).matching_contains(|val| {
    ///     match val {
    ///         &TestEnum::Good => true,
    ///         _ => false
    ///     }
    /// });
    /// ```
    fn matching_contains<F>(&mut self, matcher: F)
    where
        F: Fn(&'s T) -> bool,
    {
        let mut actual = Vec::new();
        for x in self.subject {
            if matcher(x) {
                return;
            } else {
                actual.push(x);
            }
        }
        AssertionFailure::from_spec(self).fail_with_message(format!(
            "expectation failed for iterator with values <{:?}>",
            actual
        ));
    }

    /// Maps the values of the subject before asserting that the mapped subject contains the
    /// provided value. The subject must implement IntoIterator, and the type of the mapped
    /// value must implement `PartialEq`.
    ///
    /// NOTE: The panic message will refer to the mapped values rather than the values present in
    /// the original subject.
    ///
    /// ```rust
    /// #[derive(PartialEq, Debug)]
    /// struct Simple {
    ///     pub val: usize,
    /// }
    ///
    /// //...
    /// # use speculoos::prelude::*;
    /// assert_that(&vec![Simple { val: 1 }, Simple { val: 2 } ]).mapped_contains(|x| x.val, &2);
    /// ```
    fn mapped_contains<F, M>(&mut self, mapping_function: F, expected_value: &'s M)
    where
        M: Debug + PartialEq,
        F: Fn(&'s T) -> M,
    {
        let subject = self.subject;

        let mapped_vec: Vec<M> = subject.into_iter().map(mapping_function).collect();
        if mapped_vec.contains(expected_value) {
            return;
        }

        panic_unmatched(self, expected_value, mapped_vec, true);
    }
}

fn check_iterator_contains<'s, T, V, I, E: Borrow<V>>(
    spec: &mut Spec<T>,
    actual_iter: I,
    expected_value: E,
    should_contain: bool,
) where
    V: 's + PartialEq + Debug,
    I: Iterator<Item = &'s V>,
{
    let borrowed_expected_value = expected_value.borrow();

    let mut contains_value = false;
    let mut actual = Vec::new();

    for x in actual_iter {
        if borrowed_expected_value.eq(x) {
            contains_value = true;
        }

        actual.push(x);
    }

    if contains_value != should_contain {
        panic_unmatched(spec, borrowed_expected_value, actual, should_contain);
    }
}

fn check_iterator_contains_all_of<T, V, I, E>(
    spec: &mut Spec<T>,
    actual_iter: I,
    expected_values_iter: E,
) where
    V: PartialEq + Debug,
    I: Iterator<Item = V>,
    E: Iterator<Item = V>,
{
    let actual_values: Vec<V> = actual_iter.collect();

    let mut matched_indexes = vec![];
    let mut matched_indexes_holder = vec![];

    let mut matched_values = vec![];
    let mut unmatched_values = vec![];

    'outer: for expected in expected_values_iter {
        matched_indexes.append(&mut matched_indexes_holder);

        for (index, actual) in actual_values
            .iter()
            .enumerate()
            .filter(|&(i, _)| !matched_indexes.contains(&i))
        {
            if expected.eq(actual) {
                matched_indexes_holder.push(index);
                matched_values.push(expected);
                continue 'outer;
            }
        }

        unmatched_values.push(expected);
    }

    if !unmatched_values.is_empty() {
        let mut expected_values: Vec<V> = vec![];

        expected_values.append(&mut matched_values);
        expected_values.append(&mut unmatched_values);

        AssertionFailure::from_spec(spec)
            .with_expected(format!("iterator to contain items <{:?}>", expected_values))
            .with_actual(format!("<{:?}>", actual_values))
            .fail();
    }
}

fn compare_iterators<T, V, I, E>(spec: &mut Spec<T>, actual_iter: I, expected_iter: E)
where
    V: PartialEq + Debug,
    I: Iterator<Item = V>,
    E: Iterator<Item = V>,
{
    let mut actual_iter = actual_iter;
    let mut expected_iter = expected_iter;

    let mut read_subject = vec![];
    let mut read_expected = vec![];

    loop {
        match (actual_iter.next(), expected_iter.next()) {
            (Some(actual), Some(expected)) => {
                if !&actual.eq(&expected) {
                    AssertionFailure::from_spec(spec)
                        .with_expected(format!(
                            "Iterator item of <{:?}> (read <{:?}>)",
                            expected, read_expected
                        ))
                        .with_actual(format!(
                            "Iterator item of <{:?}> (read <{:?}>)",
                            actual, read_subject
                        ))
                        .fail();

                    unreachable!();
                }

                read_subject.push(actual);
                read_expected.push(expected);
            }
            (Some(actual), None) => {
                AssertionFailure::from_spec(spec)
                    .with_expected(format!("Completed iterator (read <{:?}>)", read_expected))
                    .with_actual(format!(
                        "Iterator item of <{:?}> (read <{:?}>)",
                        actual, read_subject
                    ))
                    .fail();

                unreachable!();
            }
            (None, Some(expected)) => {
                AssertionFailure::from_spec(spec)
                    .with_expected(format!(
                        "Iterator item of <{:?}> (read <{:?}>)",
                        expected, read_expected
                    ))
                    .with_actual(format!("Completed iterator (read <{:?}>)", read_subject))
                    .fail();

                unreachable!();
            }
            (None, None) => {
                break;
            }
        }
    }
}

fn panic_unmatched<T, E: Debug, A: Debug>(
    spec: &mut Spec<T>,
    expected: E,
    actual: A,
    should_contain: bool,
) {
    let condition = {
        if should_contain {
            " "
        } else {
            " not "
        }
    };

    AssertionFailure::from_spec(spec)
        .with_expected(format!("iterator to{}contain <{:?}>", condition, expected))
        .with_actual(format!("<{:?}>", actual))
        .fail();
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_borrows_for_generic_args)]

    use super::super::prelude::*;
    use std::collections::LinkedList;

    #[test]

    fn contains_should_allow_for_multiple_borrow_types_for_intoiter() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).contains(2);
        assert_that(&test_vec).contains(&mut 2);
        assert_that(&test_vec).contains(&2);
    }

    #[test]

    fn should_not_panic_if_vec_contains_value() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).contains(&2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: iterator to contain <5>\n\t but was: <[1, 2, 3]>")]
    fn should_panic_if_vec_does_not_contain_value() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).contains(&5);
    }

    #[test]

    fn should_not_panic_if_vec_does_not_contain_value_if_expected() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).does_not_contain(&4);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: iterator to not contain <2>\n\t but was: <[1, 2, 3]>")]
    fn should_panic_if_vec_does_contain_value_and_expected_not_to() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec).does_not_contain(&2);
    }

    #[test]

    fn should_not_panic_if_iterable_contains_value() {
        let mut test_into_iter = LinkedList::new();
        test_into_iter.push_back(1);
        test_into_iter.push_back(2);
        test_into_iter.push_back(3);

        assert_that(&test_into_iter).contains(&2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: iterator to contain <5>\n\t but was: <[1, 2, 3]>")]
    fn should_panic_if_iterable_does_not_contain_value() {
        let mut test_into_iter = LinkedList::new();
        test_into_iter.push_back(1);
        test_into_iter.push_back(2);
        test_into_iter.push_back(3);

        assert_that(&test_into_iter).contains(&5);
    }

    #[test]
    fn should_not_panic_if_iterable_contains_all_expected_values() {
        let mut test_into_iter = LinkedList::new();
        test_into_iter.push_back(1);
        test_into_iter.push_back(2);
        test_into_iter.push_back(3);

        assert_that(&test_into_iter).contains_all_of(&vec![&2, &3]);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: iterator to contain items <[1, 6]>\
                   \n\t but was: <[1, 2, 3]>")]
    fn should_panic_if_iterable_does_not_contain_all_expected_values() {
        let mut test_into_iter = LinkedList::new();
        test_into_iter.push_back(1);
        test_into_iter.push_back(2);
        test_into_iter.push_back(3);

        assert_that(&test_into_iter).contains_all_of(&vec![&1, &6]);
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn should_not_panic_if_iterator_contains_all_expected_values() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec.iter()).contains_all_of(&vec![&2, &3]);
    }

    #[test]
    #[allow(clippy::useless_vec)]
    #[should_panic(expected = "\n\texpected: iterator to contain items <[1, 6]>\
                   \n\t but was: <[1, 2, 3]>")]
    fn should_panic_if_iterator_does_not_contain_all_expected_values() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec.iter()).contains_all_of(&vec![&1, &6]);
    }

    #[test]
    #[allow(clippy::useless_vec)]
    #[should_panic(expected = "\n\texpected: iterator to contain items <[1, 3, 1]>\
                   \n\t but was: <[1, 2, 3]>")]
    fn should_panic_if_iterator_does_not_contain_all_expected_values_exactly() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec.iter()).contains_all_of(&vec![&1, &1, &3]);
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn should_not_panic_if_iteratable_equals_expected_iterator() {
        let expected_vec = vec![1, 2, 3];
        let test_vec = vec![1, 2, 3];

        assert_that(&test_vec).equals_iterator(&expected_vec.iter());
    }

    #[test]
    #[allow(clippy::useless_vec)]
    #[should_panic(expected = "\n\texpected: Iterator item of <4> (read <[1, 2]>)\
                   \n\t but was: Iterator item of <3> (read <[1, 2]>)")]
    fn should_panic_if_iteratable_does_not_equal_expected_iterator() {
        let expected_vec = vec![1, 2, 4];
        let test_vec = vec![1, 2, 3];

        assert_that(&test_vec).equals_iterator(&expected_vec.iter());
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn contains_should_allow_for_multiple_borrow_types_for_iterators() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec.iter()).contains(2);
        assert_that(&test_vec.iter()).contains(&mut 2);
        assert_that(&test_vec.iter()).contains(&2);
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn should_not_panic_if_iterator_contains_value() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec.iter()).contains(&2);
    }

    #[test]
    #[allow(clippy::useless_vec)]
    #[should_panic(expected = "\n\texpected: iterator to contain <5>\n\t but was: <[1, 2, 3]>")]
    fn should_panic_if_iterator_does_not_contain_value() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec.iter()).contains(&5);
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn should_not_panic_if_iterator_does_not_contain_value_if_expected() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec.iter()).does_not_contain(&4);
    }

    #[test]
    #[allow(clippy::useless_vec)]
    #[should_panic(expected = "\n\texpected: iterator to not contain <2>\n\t but was: <[1, 2, 3]>")]
    fn should_panic_if_iterator_does_contain_value_but_expected_not_to() {
        let test_vec = vec![1, 2, 3];
        assert_that(&test_vec.iter()).does_not_contain(&2);
    }

    #[test]
    #[allow(clippy::useless_vec)]
    fn should_not_panic_if_iterator_equals_expected_iterator() {
        let expected_vec = vec![1, 2, 3];
        let test_vec = vec![1, 2, 3];

        assert_that(&test_vec.iter()).equals_iterator(&expected_vec.iter());
    }

    #[test]
    #[allow(clippy::useless_vec)]
    #[should_panic(expected = "\n\texpected: Iterator item of <4> (read <[1, 2]>)\
                   \n\t but was: Iterator item of <3> (read <[1, 2]>)")]
    fn should_panic_if_iterator_does_not_equal_expected_iterator() {
        let expected_vec = vec![1, 2, 4];
        let test_vec = vec![1, 2, 3];

        assert_that(&test_vec.iter()).equals_iterator(&expected_vec.iter());
    }

    #[test]
    fn should_not_panic_if_iterator_matches_on_value() {
        let mut test_into_iter = LinkedList::new();
        test_into_iter.push_back(TestEnum::Bad);
        test_into_iter.push_back(TestEnum::Good);
        test_into_iter.push_back(TestEnum::Bad);

        assert_that(&test_into_iter).matching_contains(|val| matches!(val, &TestEnum::Good));
    }

    #[test]
    #[should_panic(expected = "\n\texpectation failed for iterator with values <[Bad, Bad, Bad]>")]
    fn should_panic_if_iterator_does_not_match_on_value() {
        let mut test_into_iter = LinkedList::new();
        test_into_iter.push_back(TestEnum::Bad);
        test_into_iter.push_back(TestEnum::Bad);
        test_into_iter.push_back(TestEnum::Bad);

        assert_that(&test_into_iter).matching_contains(|val| matches!(val, &TestEnum::Good));
    }

    #[test]
    fn should_not_panic_if_vec_contains_mapped_value() {
        let test_vec = vec![TestStruct { value: 5 }, TestStruct { value: 6 }];
        assert_that(&test_vec).mapped_contains(|val| val.value, &5);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: iterator to contain <1>\n\t but was: <[5, 6]>")]
    fn should_panic_if_vec_does_not_contain_mapped_value() {
        let test_vec = vec![TestStruct { value: 5 }, TestStruct { value: 6 }];
        assert_that(&test_vec).mapped_contains(|val| val.value, &1);
    }

    #[derive(Debug, PartialEq)]
    struct TestStruct {
        pub value: u8,
    }

    #[derive(Debug)]
    enum TestEnum {
        Good,
        Bad,
    }
}
