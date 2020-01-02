use super::{AssertionFailure, Spec};

use std::borrow::Borrow;
use std::collections::HashSet;
use std::fmt::Debug;
use std::hash::Hash;

pub trait HashSetAssertions<'s> {
    fn has_length(&mut self, expected: usize);
    fn is_empty(&mut self);
}

pub trait ValueHashSetAssertions<'s, V: Hash + Eq> {
    fn contains_value<E: Borrow<V>>(&mut self, expected: E) -> Spec<'s, V>;
    fn does_not_contain_value<E: Borrow<V>>(&mut self, expected: E);
}

impl<'s, V> HashSetAssertions<'s> for Spec<'s, HashSet<V>>
where
    V: Debug,
{
    /// Asserts that the length of the subject hashset is equal to the provided length. The subject
    /// type must be of `HashSet`.
    ///
    /// ```rust,ignore
    /// let mut test_set = HashSet::new();
    /// test_set.insert(1);
    /// test_set.insert(2);
    ///
    /// assert_that(&test_set).has_length(2);
    /// ```
    fn has_length(&mut self, expected: usize) {
        let subject = self.subject;

        if subject.len() != expected {
            AssertionFailure::from_spec(self)
                .with_expected(format!("hashset to have length <{}>", expected))
                .with_actual(format!("<{}>", subject.len()))
                .fail();
        }
    }

    /// Asserts that the subject hashset is empty. The subject type must be of `HashSet`.
    ///
    /// ```rust,ignore
    /// let test_set: HashSet<u8> = HashSet::new();
    /// assert_that(&test_set).is_empty();
    /// ```
    fn is_empty(&mut self) {
        let subject = self.subject;

        if !subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected(format!("an empty hashset"))
                .with_actual(format!("a hashset with length <{:?}>", subject.len()))
                .fail();
        }
    }
}

impl<'s, V> ValueHashSetAssertions<'s, V> for Spec<'s, HashSet<V>>
where
    V: Hash + Eq + Debug,
{
    /// Asserts that the subject hashset contains the expected value. The subject type must be
    /// of `HashSet`.
    ///
    /// This will return a new `Spec` containing the value if present.
    ///
    /// ```rust,ignore
    /// let mut test_set = HashSet::new();
    /// test_set.insert("hello");
    ///
    /// assert_that(&test_set).contains_value(&"hello");
    /// ```
    fn contains_value<E: Borrow<V>>(&mut self, expected: E) -> Spec<'s, V> {
        let subject = self.subject;
        let borrowed_expected = expected.borrow();

        if let Some(value) = subject.get(borrowed_expected) {
            return Spec {
                subject: value,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            };
        }

        let subject_values: Vec<&V> = subject.iter().collect();

        AssertionFailure::from_spec(self)
            .with_expected(format!(
                "hashset to contain value <{:?}>",
                borrowed_expected
            ))
            .with_actual(format!("<{:?}>", subject_values))
            .fail();

        unreachable!();
    }

    /// Asserts that the subject hashset does not contain the provided value. The subject type must be
    /// of `HashSet`.
    ///
    /// ```rust,ignore
    /// let mut test_set = HashSet::new();
    /// test_set.insert("hello");
    ///
    /// assert_that(&test_set).does_not_contain_value(&"hey");
    /// ```
    fn does_not_contain_value<E: Borrow<V>>(&mut self, expected: E) {
        let subject = self.subject;
        let borrowed_expected = expected.borrow();

        if subject.get(borrowed_expected).is_some() {
            AssertionFailure::from_spec(self)
                .with_expected(format!(
                    "hashset to not contain value <{:?}>",
                    borrowed_expected
                ))
                .with_actual(format!("present in hashset"))
                .fail();
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::prelude::*;
    use std::collections::HashSet;

    #[test]
    fn should_not_panic_if_hashset_length_matches_expected() {
        let mut test_set = HashSet::new();
        test_set.insert(1);
        test_set.insert(2);

        assert_that(&test_set).has_length(2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: hashset to have length <1>\n\tbut was: <2>")]
    fn should_panic_if_hashset_length_does_not_match_expected() {
        let mut test_set = HashSet::new();
        test_set.insert(1);
        test_set.insert(2);

        assert_that(&test_set).has_length(1);
    }

    #[test]
    fn should_not_panic_if_hashset_was_expected_to_be_empty_and_is() {
        let test_set: HashSet<u8> = HashSet::new();
        assert_that(&test_set).is_empty();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: an empty hashset\
                               \n\tbut was: a hashset with length <1>")]
    fn should_panic_if_hashset_was_expected_to_be_empty_and_is_not() {
        let mut test_set = HashSet::new();
        test_set.insert(1);

        assert_that(&test_set).is_empty();
    }

    #[test]
    fn should_not_panic_if_hashset_contains_value() {
        let mut test_set = HashSet::new();
        test_set.insert(1);

        assert_that(&test_set).contains_value(&1);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: hashset to contain value <2>\
                               \n\tbut was: <[1]>")]
    fn should_panic_if_hashset_does_not_contain_value() {
        let mut test_set = HashSet::new();
        test_set.insert(1);

        assert_that(&test_set).contains_value(&2);
    }

    #[test]
    fn should_not_panic_if_hashset_does_not_contain_value() {
        let mut test_set = HashSet::new();
        test_set.insert(1);

        assert_that(&test_set).does_not_contain_value(&2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: hashset to not contain value <1>\
                               \n\tbut was: present in hashset")]
    fn should_panic_if_hashset_contains_value() {
        let mut test_set = HashSet::new();
        test_set.insert(1);

        assert_that(&test_set).does_not_contain_value(&1);
    }
}
