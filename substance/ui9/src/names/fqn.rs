use crate::names::Pqn;
use nom::{
    bytes::complete::{tag, take_while1},
    multi::separated_list1,
    AsChar, IResult, Parser,
};
use serde::{Deserialize, Serialize};
use std::fmt;
use std::iter;
use std::str::FromStr;
use thiserror::Error;

/// Helper function to parse a valid identifier component
fn identifier(input: &str) -> IResult<&str, &str> {
    take_while1(|c: char| c.is_alphanum() || c == '_')(input)
}

/// Function to parse an FQN
fn fqn(input: &str) -> IResult<&str, Vec<&str>> {
    separated_list1(tag("."), identifier).parse(input)
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Failed to parse FQN: {0}")]
    FailedToParse(String),
    #[error("Unexpected input remaining: {0}")]
    Remaining(String),
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Deserialize, Serialize)]
pub struct Fqn {
    components: Vec<String>,
}

impl Fqn {
    pub fn genesis() -> Self {
        Self::from_iter([])
    }

    pub fn root(name: &str) -> Self {
        Self::from_iter(iter::once(name))
    }

    pub fn from_iter<'a>(components: impl IntoIterator<Item = &'a str>) -> Self {
        let components: Vec<_> = components.into_iter().map(String::from).collect();
        Fqn { components }
    }

    pub fn iter(&self) -> impl Iterator<Item = &'_ str> {
        self.components.iter().map(String::as_ref)
    }

    pub fn extend(&self, pqn: &Pqn) -> Self {
        let components = self.iter().chain(pqn.iter());
        Self::from_iter(components)
    }
}

impl FromStr for Fqn {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match fqn(s) {
            Ok((remaining, components)) if remaining.is_empty() => Ok(Fqn::from_iter(components)),
            Ok((remaining, _)) => Err(Error::Remaining(remaining.into())),
            Err(err) => Err(Error::FailedToParse(err.to_string())),
        }
    }
}

impl AsRef<[String]> for Fqn {
    fn as_ref(&self) -> &[String] {
        &self.components
    }
}

impl fmt::Display for Fqn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.components.join("."))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fqn_parsing() {
        let input = "app.module.scope.component";
        let expected = Fqn::from_iter(["app", "module", "scope", "component"]);
        let result = Fqn::from_str(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_fqn_parsing_with_error() {
        let input = "app.module..scope.component";
        let result = Fqn::from_str(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_fqn_display() {
        let fqn = Fqn::from_iter(["app", "module", "scope", "component"]);
        assert_eq!(fqn.to_string(), "app.module.scope.component");
    }
}
