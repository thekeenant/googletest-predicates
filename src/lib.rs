use googletest::matcher::{Matcher, MatcherResult};
use predicates::{reflection::PredicateReflection, Predicate};
use std::{
    fmt::{Debug, Display},
    marker::PhantomData,
};

/// Provides a [Predicate] implementation for a given googletest [Matcher].
pub fn predicate<M: Matcher<ActualT = T>, T: Debug>(matcher: M) -> impl Predicate<T> {
    MatcherPredicate {
        matcher,
        _phantom_data_t: Default::default(),
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
