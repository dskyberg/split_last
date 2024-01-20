//! # split_last
//!
//! Split a string with a [Pattern] and return the last element
//!
//! There are a lot of situation where you want to split a string with a delimiter, but only
//! need the last element.
//!
//! The purpose of this crate is to demonstrate how to use traits to extend foreign types,
//! such as core::str.  But this also demonstrates a couple other useful tools:
//!
//! - Use nightly build.
//! - Access [Pattern]
//! - Custom errors
//!
//! ## Requires nightly build (for [Pattern] support)
//!
//! ## Examples
//! ````
//! use split_last::SplitLast;
//!
//! let result = "test".split_last('/').expect("oops");
//! assert_eq!(result, "test");
//!
//! let result = "test/".split_last("/").expect("oops");
//! assert_eq!(result, "test");
//!
//! let result = "/some/long//test/".split_last('/').expect("oops");
//! assert_eq!(result, "test");
//!````
#![feature(pattern)]
use std::str::pattern::{Pattern, ReverseSearcher};

/// Demonstrate how to create a custom error
/// Implement Display and Debug for a tuple containing a single string.
///
pub struct SplitError(
    /// The error message that is displayed
    pub String,
);

impl std::fmt::Display for SplitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0) // user-facing output
    }
}

impl std::fmt::Debug for SplitError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{{ file: {}, line: {}, error: {}}}",
            file!(),
            line!(),
            self.0
        ) // programmer-facing output
    }
}

/// Allows us to implement split_last on external types, such as [core::str].
pub trait SplitLast<'a, P: Pattern<'a> + Copy> {
    type Error;
    /// Takes the same [Pattern] as all other split functions.
    /// Example:
    /// ```rust
    /// use split_last::SplitLast;
    ///
    /// let result = "/some/simple/test".split_last('/').expect("oops");
    /// assert_eq!(result, "test");
    ///
    /// let result = "some/simple/test/with_trailing/".split_last('/').expect("oops");
    /// assert_eq!(result, "with_trailing");
    ///```
    fn split_last(&'a self, pat: P) -> Result<&str, Self::Error>
    where
        <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>;
}

impl<'a, P: Pattern<'a> + Copy> SplitLast<'a, P> for &str {
    type Error = SplitError;

    #[inline]
    fn split_last(&'a self, pat: P) -> Result<&str, Self::Error>
    where
        <P as Pattern<'a>>::Searcher: ReverseSearcher<'a>,
    {
        // This just lets us strip off any trailing patterns.  Else
        // split_last would return an empty string.
        let target = match pat.strip_suffix_of(self) {
            Some(target) => target,
            None => self,
        };

        target
            .split(pat)
            .last()
            .ok_or_else(|| SplitError("Failed to split".to_string()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_it() {
        let result = "test".split_last('/').expect("oops");
        assert_eq!(result, "test");

        let result = "test/".split_last("/").expect("oops");
        assert_eq!(result, "test");

        let result = "/test".split_last('/').expect("oops");
        assert_eq!(result, "test");

        let result = "/test/".split_last('/').expect("oops");
        assert_eq!(result, "test");

        let result = "/some/long//test/".split_last('/').expect("oops");
        assert_eq!(result, "test");
    }
}
