use super::{AssertionFailure, Spec};

use std::fmt::Debug;
use std::cmp::PartialOrd;

#[cfg(feature = "num")]
use num::Float;

pub trait OrderedAssertions<T>
    where T: Debug + PartialOrd
{
    fn is_less_than(&mut self, other: &T);
    fn is_less_than_or_equal_to(&mut self, other: &T);
    fn is_greater_than(&mut self, other: &T);
    fn is_greater_than_or_equal_to(&mut self, other: &T);
}

impl<'s, T> OrderedAssertions<T> for Spec<'s, T>
    where T: Debug + PartialOrd
{
    /// Asserts that the subject is less than the expected value. The subject type must
    /// implement `PartialOrd`.
    ///
    /// ```rust,ignore
    /// assert_that(&1).is_less_than(&2);
    /// ```
    fn is_less_than(&mut self, other: &T) {
        let subject = self.subject;

        if subject >= other {
            AssertionFailure::from_spec(self)
                .with_expected(format!("value less than <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    /// Asserts that the subject is less than or equal to the expected value. The subject type
    /// must implement `PartialOrd`.
    ///
    /// ```rust,ignore
    /// assert_that(&2).is_less_than_or_equal_to(&2);
    /// ```
    fn is_less_than_or_equal_to(&mut self, other: &T) {
        let subject = self.subject;

        if subject > other {
            AssertionFailure::from_spec(self)
                .with_expected(format!("value less than or equal to <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    /// Asserts that the subject is greater than the expected value. The subject type must
    /// implement `PartialOrd`.
    ///
    /// ```rust,ignore
    /// assert_that(&2).is_greater_than(&1);
    /// ```
    fn is_greater_than(&mut self, other: &T) {
        let subject = self.subject;

        if subject <= other {
            AssertionFailure::from_spec(self)
                .with_expected(format!("value greater than <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }

    /// Asserts that the subject is greater than or equal to the expected value. The subject type
    /// must implement `PartialOrd`.
    ///
    /// ```rust,ignore
    /// assert_that(&2).is_greater_than_or_equal_to(&1);
    /// ```
    fn is_greater_than_or_equal_to(&mut self, other: &T) {
        let subject = self.subject;

        if subject < other {
            AssertionFailure::from_spec(self)
                .with_expected(format!("value greater than or equal to <{:?}>", other))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }
}

#[cfg(feature = "num")]
pub trait FloatAssertions<T: Float> {
    fn is_close_to(&mut self, expected: T, tolerance: T);
}

#[cfg(feature = "num")]
impl<'s, T: Float + Debug> FloatAssertions<T> for Spec<'s, T> {
    /// Asserts that the subject is close to the expected value by the specified tolerance.
    /// The subject type must implement `Float` and `Debug`.
    ///
    /// ```rust,ignore
    /// assert_that(&2.0f64).is_close_to(2.0f64, 0.01f64);
    /// ```
    fn is_close_to(&mut self, expected: T, tolerance: T) {
        let subject = *self.subject;

        let difference = (subject - expected).abs();

        if !subject.is_finite() || difference > tolerance.abs() {
            AssertionFailure::from_spec(self)
                .with_expected(format!("float close to <{:?}> (tolerance of <{:?}>)",
                                       expected,
                                       tolerance))
                .with_actual(format!("<{:?}>", subject))
                .fail();
        }
    }
}

#[cfg(test)]
mod tests {

    use super::super::prelude::*;

    use num::Float;

    #[test]
    fn should_not_panic_if_value_is_less_than_expected() {
        assert_that(&1).is_less_than(&2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: value less than <2>\n\t but was: <3>")]
    fn should_panic_if_value_is_greater_than_expected() {
        assert_that(&3).is_less_than(&2);
    }

    #[test]
    fn should_not_panic_if_value_is_less_than_or_equal_to_than_expected() {
        assert_that(&2).is_less_than_or_equal_to(&2);
        assert_that(&2).is_less_than_or_equal_to(&3);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: value less than or equal to <2>\n\t but was: <3>")]
    fn should_panic_if_value_is_greater_than_or_not_equal_to_expected() {
        assert_that(&3).is_less_than_or_equal_to(&2);
    }

    #[test]
    fn should_not_panic_if_value_is_greater_than_expected() {
        assert_that(&3).is_greater_than(&2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: value greater than <3>\n\t but was: <2>")]
    fn should_panic_if_value_is_less_than_expected() {
        assert_that(&2).is_greater_than(&3);
    }

    #[test]
    fn should_not_panic_if_value_is_greater_than_or_equal_to_expected() {
        assert_that(&3).is_greater_than_or_equal_to(&3);
        assert_that(&3).is_greater_than_or_equal_to(&2);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: value greater than or equal to <3>\n\t but was: <2>")]
    fn should_panic_if_value_is_less_than_or_not_equal_to_expected() {
        assert_that(&2).is_greater_than_or_equal_to(&3);
    }

    #[test]
    fn should_not_panic_if_float_exactly_matches() {
        assert_that(&2.0f64).is_close_to(2.0f64, 0.01f64);
        assert_that(&0f64).is_close_to(0f64, 0.01f64);
    }

    #[test]
    fn should_not_panic_if_float_is_close_to() {
        assert_that(&1e-40f32).is_close_to(0.0f32, 0.1f32);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: float close to <1> (tolerance of <0.01>)\
                   \n\t but was: <2>")]
    fn should_panic_if_float_is_not_close_to() {
        assert_that(&2.0f64).is_close_to(1.0f64, 0.01f64);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: float close to <1> (tolerance of <0.01>)\
                   \n\t but was: <NaN>")]
    fn should_panic_if_float_is_nan() {
        assert_that(&Float::nan()).is_close_to(1.0f64, 0.01f64);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: float close to <1> (tolerance of <0.01>)\
                   \n\t but was: <inf>")]
    fn should_panic_if_float_is_infinity() {
        assert_that(&Float::infinity()).is_close_to(1.0f64, 0.01f64);
    }

    #[test]
    #[should_panic(expected = "\n\texpected: float close to <1> (tolerance of <0.01>)\
                   \n\t but was: <-inf>")]
    fn should_panic_if_float_is_negative_infinity() {
        assert_that(&Float::neg_infinity()).is_close_to(1.0f64, 0.01f64);
    }
}
