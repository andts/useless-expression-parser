use ast::Expression;
use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

mod ast;

#[derive(Parser)]
#[grammar = "expression.pest"]
pub struct ExpressionParser;

pub fn eval_expression(input: &str) -> Result<f64, String> {
    let parsed = ExpressionParser::parse(Rule::expression_input, input)
        .map_err(|e| format!("Parsing error: {:?}", e))?;

    let expr = parsed.into_iter().next().unwrap();
    let ast = convert_to_ast(expr);
    let numeric_expression = eval_ast(ast);
    Ok(numeric_expression)
}

fn convert_to_ast(expr: Pair<Rule>) -> Expression {
    match expr.as_rule() {
        Rule::or_operand => todo!(),
        Rule::and_operand => todo!(),
        Rule::not_operand => todo!(),
        Rule::comp_operand => todo!(),
        Rule::add_operand => todo!(),
        Rule::mul_operand => todo!(),
        Rule::primary_expression => todo!(),
        Rule::literal => todo!(),
        Rule::function_call => todo!(),
        Rule::function_arguments => todo!(),
        Rule::function_argument => todo!(),
        Rule::string_literal => todo!(),
        Rule::raw_string_character => todo!(),
        Rule::escaped_character => todo!(),
        Rule::identifier => todo!(),
        Rule::field_reference => todo!(),
        Rule::number => todo!(),
        Rule::integer => todo!(),
        Rule::float => todo!(),
        Rule::exp => todo!(),
        Rule::comparison_operator => todo!(),
        Rule::gte_op => todo!(),
        Rule::gt_op => todo!(),
        Rule::lte_op => todo!(),
        Rule::lt_op => todo!(),
        Rule::neq_op => todo!(),
        Rule::eq_op => todo!(),
        Rule::and_op => todo!(),
        Rule::or_op => todo!(),
        Rule::not_op => todo!(),
        Rule::boolean_literal => todo!(),
        Rule::plus => todo!(),
        Rule::minus => todo!(),
        Rule::mul => todo!(),
        Rule::div => todo!(),
        Rule::WHITESPACE => todo!(),
        _ => unreachable!()
    }

}

fn eval_ast(ast: Expression) -> f64 {
    todo!()
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