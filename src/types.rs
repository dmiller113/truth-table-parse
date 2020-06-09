use std::convert::TryFrom;

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub struct Rule {
  pub inputs: Vec<bool>,
  pub result: bool,
}

#[derive(Eq)]
#[derive(PartialEq)]
#[derive(Debug)]
pub enum ParseError {
  TooFewInputs,
}

impl TryFrom<String> for Rule {
  type Error = ParseError;
  fn try_from(s: String) -> Result<Self, Self::Error> {
    let mut parts: Vec<bool> = s
      .split_whitespace()
      .map(|n| match n {
        "0" => false,
        _ => true,
      }).collect();
    if parts.len() < 2 {
      Err(ParseError::TooFewInputs)
    } else {
      parts
        .pop()
        .map(|r| Rule { inputs: parts, result: r })
        .ok_or(ParseError::TooFewInputs)
    }
  }
}
