use super::Spec;

use std::cmp::PartialEq;
use std::fmt::Debug;

pub trait OptionSpec<T>
    where T: Debug
{
    fn is_some(&mut self) -> &mut Self;
    fn is_none(&mut self) -> &mut Self;
}

pub trait ComparingOptionSpec<T>
    where T: Debug + PartialEq
{
    fn contains_value(&mut self, expected_value: &T) -> &mut Self;
}

impl<'s, T> ComparingOptionSpec<T> for Spec<'s, Option<T>>
    where T: Debug + PartialEq
{
    /// Asserts that the subject is a `Some` containing the expected value. The subject type must
    /// be an `Option`.
    ///
    /// ```rust,ignore
    /// assert_that(&Some(1)).contains_value(&1);
    /// ```
    fn contains_value(&mut self, expected_value: &T) -> &mut Self {
        match self.subject {
            &Some(ref val) => {
                if !val.eq(expected_value) {
                    self.with_expected(format!("option to contain <{:?}>", expected_value))
                        .with_actual(format!("<{:?}>", val))
                        .fail();
                }
            }
            &None => {
                self.with_expected(format!("option<{:?}>", expected_value))
                    .with_actual(format!("option[none]"))
                    .fail();
            }
        };

        self
    }
}

impl<'s, T> OptionSpec<T> for Spec<'s, Option<T>>
    where T: Debug
{
    /// Asserts that the subject is `Some`. The subject type must be an `Option`.
    ///
    /// ```rust,ignore
    /// assert_that(&Some(1)).is_some();
    /// ```
    fn is_some(&mut self) -> &mut Self {
        match self.subject {
            &Some(_) => (),
            &None => {
                self.with_expected(format!("option[some]"))
                    .with_actual(format!("option[none]"))
                    .fail();
            }
        };

        self
    }

    /// Asserts that the subject is `None`. The value type must be an `Option`.
    ///
    /// ```rust,ignore
    /// assert_that(&Option::None::<String>).is_none();
    /// ```
    fn is_none(&mut self) -> &mut Self {
        match self.subject {
            &None => (),
            &Some(ref val) => {
                self.with_expected(format!("option[none]"))
                    .with_actual(format!("option<{:?}>", val))
                    .fail();
            }
        };

        self
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    #[test]
    fn should_not_panic_if_option_is_expected_to_contain_value_and_does() {
        let option = Some("Hello");
        assert_that(&option).is_some();
    }

    #[test]
    #[should_panic(expected = "expected option[some] but was option[none]")]
    fn should_panic_if_option_is_expected_to_contain_value_and_does_not() {
        let option: Option<&str> = None;
        assert_that(&option).is_some();
    }

    #[test]
    fn should_not_panic_if_option_contains_expected_value() {
        let option = Some("Hello");
        assert_that(&option).contains_value(&"Hello");
    }

    #[test]
    #[should_panic(expected = "expected option to contain <\"Hi\"> but was <\"Hello\">")]
    fn should_panic_if_option_does_not_contain_expected_value() {
        let option = Some("Hello");
        assert_that(&option).contains_value(&"Hi");
    }

    #[test]
    #[should_panic(expected = "expected option<\"Hello\"> but was option[none]")]
    fn should_panic_if_option_is_none_but_expected_value() {
        let option: Option<&str> = None;
        assert_that(&option).contains_value(&"Hello");
    }

    #[test]
    fn should_not_panic_if_option_is_empty() {
        let option: Option<&str> = None;
        assert_that(&option).is_none();
    }

    #[test]
    #[should_panic(expected = "expected option[none] but was option<\"Hello\"")]
    fn should_panic_if_option_is_not_empty_but_was_expected_as_empty() {
        let option = Some("Hello");
        assert_that(&option).is_none();
    }

}
