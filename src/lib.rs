mod types;


#[cfg(test)]
mod tests {
    use crate::types::{ ParseError, Rule };
    use std::convert::TryFrom;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parse_single_string() {
        let test_string = "1 1 0";
        let expected_result = Rule {
            inputs: vec![true, true],
            result: false,
        };
        let result = Rule::try_from(test_string.to_string());
        match result {
            Ok(rule) => assert_eq!(rule, expected_result),
            Err(_) => panic!(),
        };
    }

    #[test]
    fn fail_too_few() {
        let test_string = "1";
        let expected_result =
            ParseError::TooFewInputs;

        let result = Rule::try_from(test_string.to_string());
        match result {
            Ok(_) => panic!(),
            Err(error) => assert_eq!(error, expected_result),
        }
    }
}
