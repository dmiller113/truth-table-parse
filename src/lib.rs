mod types;

#[cfg(test)]
mod tests {
    use crate::types::{ParseError, Rule, TestableInput, TruthTable};
    use std::convert::TryFrom;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn parse_number_string() {
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
    fn parse_char_string() {
        let test_string = "t t f";
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
        let expected_result = ParseError::TooFewInputs;

        let result = Rule::try_from(test_string.to_string());
        match result {
            Ok(_) => panic!(),
            Err(error) => assert_eq!(error, expected_result),
        }
    }

    #[test]
    fn make_truth_table_from_rows() {
        let rule1 = Rule {
            inputs: vec![true, false],
            result: true,
        };
        let rule2 = Rule {
            inputs: vec![false, true],
            result: false,
        };

        let rules = vec![rule1, rule2];
        let expected_result = TruthTable(rules.clone());
        let result: TruthTable = rules.into_iter().collect();
        assert_eq!(result, expected_result);
    }

    #[test]
    fn testable_input_from_string() {
        fn test_input<F>(
            input: &str,
            expected_result: &TestableInput,
            assert: F,
        ) where F: FnOnce(TestableInput, &TestableInput) {
            let result: TestableInput = input
            .split_whitespace()
            .map(str::to_string)
            .collect();

            assert(result, expected_result);
        }

        fn true_assert(a: TestableInput, b: &TestableInput) {
            assert_eq!(a, *b);
        }

        fn false_assert(a: TestableInput, b: &TestableInput) {
            assert_ne!(a, *b);
        }

        let true_number_input = "1 1 1";
        let true_string_input = "t t t";
        let false_number_input = "0 0 0";
        let false_string_input = "f f f";
        let false_capital_input = "F F F";

        let mixed_input = "1 0 1 t f F";

        let true_raw_result = vec![true, true, true];
        let false_raw_result = vec![false, false, false];

        let true_expected_result =
            TestableInput(true_raw_result.clone());
        let false_expected_result =
            TestableInput(false_raw_result.clone());

        test_input(
            true_number_input,
            &true_expected_result,
            true_assert,
        );
        test_input(
            true_string_input,
            &true_expected_result,
            true_assert,
        );
        test_input(
            false_number_input,
            &false_expected_result,
            true_assert,
        );
        test_input(
            false_string_input,
            &false_expected_result,
            true_assert,
        );
        test_input(
            false_capital_input,
            &false_expected_result,
            true_assert,
        );
        test_input(
            mixed_input,
            &true_expected_result,
            false_assert,
        );
    }
}
