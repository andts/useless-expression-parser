use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "expression.pest"]
pub struct ExpressionParser;

fn parse_numeric_expression(pair: Pair<Rule>) -> f64 {
    let mut inner_rules = pair.into_inner();

    let first_operand_pair = inner_rules.next().unwrap();
    let first_operand = parse_numeric_term(first_operand_pair);

    let mut result = first_operand;

    if let Some(operator) = inner_rules.next() {
        let second_operand_pair = inner_rules.next().unwrap();
        let second_operand = parse_numeric_term(second_operand_pair);

        result = match operator.as_rule() {
            Rule::plus => first_operand + second_operand,
            Rule::minus => first_operand - second_operand,
            _ => unreachable!(),
        }
    }

    result
}

fn parse_numeric_term(pair: Pair<Rule>) -> f64 {
    let mut inner_rules = pair.into_inner();

    let first_operand_pair = inner_rules.next().unwrap();
    let first_operand = parse_numeric_factor(first_operand_pair);

    let mut result = first_operand;

    if let Some(operator) = inner_rules.next() {

        let second_operand_pair = inner_rules.next().unwrap();
        let second_operand = parse_numeric_factor(second_operand_pair);

        result = match operator.as_rule() {
            Rule::mul => first_operand * second_operand,
            Rule::div => first_operand / second_operand,
            _ => unreachable!(),
        }
    }

    result
}

fn parse_numeric_factor(pair: Pair<Rule>) -> f64 {
    let numeric_factor = pair.into_inner().next().unwrap();

    match numeric_factor.as_rule() {
        Rule::number => parse_number(numeric_factor),
        Rule::numeric_expression => parse_numeric_expression(numeric_factor),
        _ => unreachable!(),
    }
}

fn parse_number(pair: Pair<Rule>) -> f64 {
    let number = pair.into_inner().next().unwrap();

    match number.as_rule() {
        Rule::integer => number.as_str().parse().unwrap(),
        Rule::float => number.as_str().parse().unwrap(),
        _ => unreachable!(),
    }
}

pub fn parse_arithmetic_expression(input: &str) -> Result<f64, String> {
    let parsed = ExpressionParser::parse(Rule::expression, input)
        .map_err(|e| format!("Parsing error: {:?}", e))?;

    let expr = parsed.into_iter().next().unwrap();
    let numeric_expression = parse_numeric_expression(expr);
    Ok(numeric_expression)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arithmetic_parsing() {
        let test_cases = vec![
            ("1 + 2", 3.0),
            ("1 - 2", -1.0),
            ("1 * 2", 2.0),
            ("1 / 2", 0.5),
            ("1 + 2 * 3", 7.0),
            ("(1 + 2) * 3", 9.0),
            ("-1 * 2", -2.0),
            ("3.5 + 1.5", 5.0),
            ("3.5 * 2", 7.0),
            ("(2 + 3) * 4.5", 22.5),
        ];

        for (input, expected_output) in test_cases {
            let result = parse_arithmetic_expression(input).unwrap();
            assert!((result - expected_output).abs() < f64::EPSILON, "Expression: {}, Result: {}, Expected: {}", input, result, expected_output);
        }
    }
}

#[cfg(test)]
mod expression_tests;