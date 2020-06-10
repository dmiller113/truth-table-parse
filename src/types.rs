use std::clone::Clone;
use std::convert::TryFrom;
use std::iter::FromIterator;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct Rule {
    pub inputs: Vec<bool>,
    pub result: bool,
}

#[derive(Eq, PartialEq, Debug)]
pub struct TruthTable(pub Vec<Rule>);

#[derive(Eq, PartialEq, Debug)]
pub struct TestableInput(pub Vec<bool>);

#[derive(Eq, PartialEq, Debug)]
pub enum ParseError {
    TooFewInputs,
}

pub fn parse_input(s: &str) -> bool {
  match s {
    "0" => false,
    "f" => false,
    "F" => false,
    _ => true,
  }
}

impl FromIterator<Rule> for TruthTable {
    fn from_iter<T>(i: T) -> Self
    where
        T: IntoIterator<Item = Rule>,
    {
        TruthTable(i.into_iter().collect())
    }
}

impl<T> FromIterator<T> for  TestableInput where T: AsRef<str> {
  fn from_iter<U>(i: U) -> Self where U: IntoIterator<Item = T> {
      TestableInput(
        i.into_iter().map(|i| parse_input(i.as_ref())).collect()
      )
    }
}

impl TryFrom<String> for Rule {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut parts: Vec<bool> = s
            .split_whitespace()
            .map(parse_input)
            .collect();
        if parts.len() < 2 {
            Err(ParseError::TooFewInputs)
        } else {
            parts
                .pop()
                .map(|r| Rule {
                    inputs: parts,
                    result: r,
                })
                .ok_or(ParseError::TooFewInputs)
        }
    }
}
