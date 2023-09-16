use googletest::matcher::{Matcher, MatcherResult};
use predicates::{reflection::PredicateReflection, Predicate};
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

/// Provides a [Predicate] implementation for a given googletest [Matcher].
pub fn matcher<M: ToPredicate<T, P>, T: Debug, P: Predicate<T>>(matcher: M) -> impl Predicate<T> {
    matcher.to_predicate()
}

/// Some type that can be converted into a [Predicate].
pub trait ToPredicate<T, P: Predicate<T>> {
    /// Converts the type into a [Predicate].
    fn to_predicate(self) -> P;
}

/// Implementation of [ToPredicate] for all googletest [Matcher]'s.
impl<M, T> ToPredicate<T, MatcherPredicate<M, T>> for M
where
    M: Matcher<ActualT = T>,
    T: Debug,
{
    fn to_predicate(self) -> MatcherPredicate<M, T> {
        MatcherPredicate {
            matcher: self,
            _phantom_data_t: Default::default(),
        }
    }
}

struct MatcherPredicate<M, T>
where
    M: Matcher<ActualT = T>,
    T: Debug,
{
    matcher: M,
    _phantom_data_t: PhantomData<T>,
}

impl<M, T> Predicate<T> for MatcherPredicate<M, T>
where
    M: Matcher<ActualT = T>,
    T: Debug,
{
    fn eval(&self, variable: &T) -> bool {
        match self.matcher.matches(variable) {
            MatcherResult::Match => true,
            MatcherResult::NoMatch => false,
        }
    }
}

impl<M, T> PredicateReflection for MatcherPredicate<M, T>
where
    M: Matcher<ActualT = T>,
    T: Debug,
{
}

impl<M, T> Display for MatcherPredicate<M, T>
where
    M: Matcher<ActualT = T>,
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.matcher.describe(MatcherResult::Match))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use googletest::{expect_that, matchers::*};
    use mockall::automock;

    #[cfg_attr(test, automock)]
    trait MyTrait {
        fn foo(&self, x: u32) -> u32;
    }

    #[googletest::test]
    fn test_matcher_eq_1_to_string() {
        expect_that!(matcher(eq(1)).to_string(), eq("is equal to 1"));
    }

    #[googletest::test]
    fn test_matcher_eq_1_eval_returns_true() {
        expect_that!(matcher(eq(1)).eval(&1), eq(true));
    }

    #[googletest::test]
    fn test_matcher_eq_1_eval_returns_false() {
        expect_that!(matcher(eq(1)).eval(&2), eq(false));
    }

    #[googletest::test]
    fn test_mockall_with_matcher_fn() {
        let mut mock = MockMyTrait::new();
        mock.expect_foo()
            .with(matcher(eq(1)))
            .times(1)
            .returning(|x| x + 1);
        expect_that!(mock.foo(1), eq(2))
    }
}
