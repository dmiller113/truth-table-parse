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
pub enum ParseError {
    TooFewInputs,
}

impl FromIterator<Rule> for TruthTable {
    fn from_iter<T>(i: T) -> Self
    where
        T: IntoIterator<Item = Rule>,
    {
        TruthTable(i.into_iter().collect())
    }
}

impl TryFrom<String> for Rule {
    type Error = ParseError;
    fn try_from(s: String) -> Result<Self, Self::Error> {
        let mut parts: Vec<bool> = s
            .split_whitespace()
            .map(|n| match n {
                "0" => false,
                "F" => false,
                "f" => false,
                _ => true,
            })
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
