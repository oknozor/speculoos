use crate::{AssertionFailure, Spec};
use serde_json::{Map, Number, Value};

pub trait JsonAssertions<'s> {
    #[track_caller]
    fn is_null(&self);

    #[track_caller]
    fn is_boolean(&self) -> Spec<'s, bool>;

    #[track_caller]
    fn is_number(&self) -> Spec<'s, Number>;

    #[track_caller]
    fn is_string(&self) -> Spec<'s, String>;

    #[track_caller]
    fn is_array(&self) -> Spec<'s, Vec<Value>>;

    #[track_caller]
    fn is_object(&self) -> Spec<'s, Map<String, Value>>;
}

pub trait JsonObjectAssertions<'s> {
    #[track_caller]
    fn has_length(&mut self, expected: usize);

    #[track_caller]
    fn is_empty(&mut self);

    #[track_caller]
    fn is_not_empty(&mut self);

    #[track_caller]
    fn contains_key(&mut self, expected_key: &str) -> Spec<'s, Value>;

    #[track_caller]
    fn does_not_contain_key(&mut self, expected_key: &str);
}

impl<'s> JsonAssertions<'s> for Spec<'s, Value> {
    /// Asserts that the subject is [`Value::Null`].
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that!(json!(null)).is_null();
    /// ```
    fn is_null(&self) {
        match self.subject {
            Value::Null => {}
            other => {
                AssertionFailure::from_spec(self)
                    .with_expected("json[null]".to_string())
                    .with_actual(build_detail_message(other))
                    .fail();

                unreachable!();
            }
        }
    }

    /// Asserts that the subject is a [`Value::Bool`].
    ///
    /// This will return a new `Spec` containing the boolean value.
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that!(json!(true)).is_boolean();
    /// ```
    fn is_boolean(&self) -> Spec<'s, bool> {
        match self.subject {
            Value::Bool(val) => Spec {
                subject: val,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            },
            other => {
                AssertionFailure::from_spec(self)
                    .with_expected("json[boolean]".to_string())
                    .with_actual(build_detail_message(other))
                    .fail();

                unreachable!();
            }
        }
    }

    /// Asserts that the subject is a [`Value::Number`].
    ///
    /// This will return a new `Spec` containing the number value.
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that!(json!(42)).is_number();
    /// ```
    fn is_number(&self) -> Spec<'s, Number> {
        match self.subject {
            Value::Number(val) => Spec {
                subject: val,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            },
            other => {
                AssertionFailure::from_spec(self)
                    .with_expected("json[number]".to_string())
                    .with_actual(build_detail_message(other))
                    .fail();

                unreachable!();
            }
        }
    }

    /// Asserts that the subject is a [`Value::String`].
    ///
    /// This will return a new `Spec` containing the string value.
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that!(json!("test")).is_string();
    /// ```
    fn is_string(&self) -> Spec<'s, String> {
        match self.subject {
            Value::String(val) => Spec {
                subject: val,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            },
            other => {
                AssertionFailure::from_spec(self)
                    .with_expected("json[string]".to_string())
                    .with_actual(build_detail_message(other))
                    .fail();

                unreachable!();
            }
        }
    }

    /// Asserts that the subject is a [`Value::Array`].
    ///
    /// This will return a new `Spec` containing the array value.
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that!(json!([])).is_array();
    /// ```
    fn is_array(&self) -> Spec<'s, Vec<Value>> {
        match self.subject {
            Value::Array(val) => Spec {
                subject: val,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            },
            other => {
                AssertionFailure::from_spec(self)
                    .with_expected("json[array]".to_string())
                    .with_actual(build_detail_message(other))
                    .fail();

                unreachable!();
            }
        }
    }

    /// Asserts that the subject is a [`Value::Object`].
    ///
    /// This will return a new `Spec` containing the object value.
    ///
    /// # Examples
    /// ```
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that!(json!({})).is_object();
    /// ```
    fn is_object(&self) -> Spec<'s, Map<String, Value>> {
        match self.subject {
            Value::Object(val) => Spec {
                subject: val,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            },
            other => {
                AssertionFailure::from_spec(self)
                    .with_expected("json[object]".to_string())
                    .with_actual(build_detail_message(other))
                    .fail();

                unreachable!();
            }
        }
    }
}

impl<'s> JsonObjectAssertions<'s> for Spec<'s, Map<String, Value>> {
    /// Asserts that the length of the JSON object is equal to the provided length.
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that(&json!({"a": null, "b": null, "c": null})).is_object().has_length(3);
    /// ```
    fn has_length(&mut self, expected: usize) {
        let subject = self.subject;

        if subject.len() != expected {
            AssertionFailure::from_spec(self)
                .with_expected(format!("json[object] to have length <{}>", expected))
                .with_actual(format!("<{}>", subject.len()))
                .fail();
        }
    }

    /// Asserts that the subject JSON object is empty.
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that(&json!({})).is_object().is_empty();
    /// ```
    fn is_empty(&mut self) {
        let subject = self.subject;

        if !subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected("an empty json[object]".to_string())
                .with_actual(format!("a json[object] with length <{:?}>", subject.len()))
                .fail();
        }
    }

    /// Asserts that the subject JSON object is not empty.
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that(&json!({"a": null, "b": null, "c": null})).is_object().is_not_empty();
    /// ```
    fn is_not_empty(&mut self) {
        let subject = self.subject;

        if subject.is_empty() {
            AssertionFailure::from_spec(self)
                .with_expected("a non empty json[object]".to_string())
                .with_actual("an empty json[object]".to_string())
                .fail();
        }
    }

    /// Asserts that the subject JSON object contains the expected key
    ///
    /// This will return a new `Spec` containing the associated value if the key is present.
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that(&json!({"a": null, "b": null, "c": null})).is_object().contains_key("a").is_null();
    /// ```
    fn contains_key(&mut self, expected_key: &str) -> Spec<'s, Value> {
        let subject = self.subject;

        if let Some(value) = subject.get(expected_key) {
            return Spec {
                subject: value,
                subject_name: self.subject_name,
                location: self.location.clone(),
                description: self.description,
            };
        }

        let subject_keys: Vec<&String> = subject.keys().collect();

        AssertionFailure::from_spec(self)
            .with_expected(format!("json[object] to contain key <{:?}>", expected_key))
            .with_actual(format!("<{:?}>", subject_keys))
            .fail();

        unreachable!();
    }

    /// Asserts that the subject JSON object does not contain the provided key.
    ///
    /// ```rust
    /// # use serde_json::json;
    /// # use speculoos::prelude::*;
    /// #
    /// assert_that(&json!({})).is_object().does_not_contain_key("key");
    /// ```
    fn does_not_contain_key(&mut self, expected_key: &str) {
        let subject = self.subject;

        if subject.get(expected_key).is_some() {
            AssertionFailure::from_spec(self)
                .with_expected(format!(
                    "json[object] to not contain key <{}>",
                    expected_key
                ))
                .with_actual("present in json[object]".to_string())
                .fail();
        }
    }
}

fn build_detail_message(value: &Value) -> String {
    match value {
        Value::Null => "json[null]".to_string(),
        Value::Bool(_) => "json[boolean]".to_string(),
        Value::Number(_) => "json[number]".to_string(),
        Value::String(_) => "json[string]".to_string(),
        Value::Array(_) => "json[array]".to_string(),
        Value::Object(_) => "json[object]".to_string(),
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::needless_borrows_for_generic_args)]

    use super::super::prelude::*;
    use serde_json::json;

    #[test]
    fn should_not_panic_if_value_is_expected_to_be_correct_type() {
        assert_that(&json!(null)).is_null();
        assert_that(&json!(true)).is_boolean();
        assert_that(&json!(42)).is_number();
        assert_that(&json!("test")).is_string();
        assert_that(&json!([])).is_array();
        assert_that(&json!({})).is_object();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: json[boolean]\n\t but was: json[null]")]
    fn should_panic_if_option_is_expected_to_contain_null_and_does_not() {
        assert_that(&json!(null)).is_boolean();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: json[number]\n\t but was: json[boolean]")]
    fn should_panic_if_option_is_expected_to_contain_boolean_and_does_not() {
        assert_that(&json!(true)).is_number();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: json[string]\n\t but was: json[number]")]
    fn should_panic_if_option_is_expected_to_contain_number_and_does_not() {
        assert_that(&json!(42)).is_string();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: json[array]\n\t but was: json[string]")]
    fn should_panic_if_option_is_expected_to_contain_string_and_does_not() {
        assert_that(&json!("test")).is_array();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: json[object]\n\t but was: json[array]")]
    fn should_panic_if_option_is_expected_to_contain_array_and_does_not() {
        assert_that(&json!([])).is_object();
    }

    #[test]
    #[should_panic(expected = "\n\texpected: json[null]\n\t but was: json[object]")]
    fn should_panic_if_option_is_expected_to_contain_object_and_does_not() {
        assert_that(&json!({})).is_null();
    }
}
