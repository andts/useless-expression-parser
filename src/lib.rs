use pest::iterators::Pair;
use pest::Parser;
use pest_derive::Parser;

use crate::ast::*;

mod ast;

#[derive(Parser)]
#[grammar = "expression.pest"]
pub struct ExpressionParser;

pub fn eval_expression(input: &str) -> Result<f64, String> {
    let parsed = ExpressionParser::parse(Rule::expression_input, input)
        .map_err(|e| format!("Parsing error: {:?}", e))?;

    let expr = parsed.into_iter().next().unwrap();
    let ast = dbg!(convert_to_ast(expr));
    let numeric_expression = eval_ast(ast);
    Ok(numeric_expression)
}

fn convert_to_ast(expr: Pair<Rule>) -> Expression {
    match expr.as_rule() {
        Rule::or_operand | Rule::and_operand => {
            let mut child_pairs = expr.into_inner();
            let first_operand = child_pairs.next().unwrap();
            let operator = child_pairs.next();
            // if there is more than one child node, then this is a full expression with an operator
            if let Some(rule) = operator {
                //collect all child nodes that are not or_operand or and_operand
                let mut operands: Vec<Expression> = vec![convert_to_ast(first_operand)];
                while let Some(child) = child_pairs.next() {
                    if child.as_rule() != Rule::or_operand && child.as_rule() != Rule::and_operand {
                        operands.push(convert_to_ast(child));
                    }
                }

                Expression::Function {
                    function_name: rule.as_str().to_string(),
                    params: operands,
                }
            } else {
                //otherwise, just return the first operand
                convert_to_ast(first_operand)
            }
        }
        Rule::comp_operand | Rule::add_operand | Rule::mul_operand => {
            let mut child_pairs = expr.into_inner();
            let first_operand = child_pairs.next().unwrap();
            let operator = child_pairs.next();
            // if there is more than one child node, then this is a full expression with an operator
            if let Some(op_rule) = operator {
                //build an expression tree from all operands in a left-associative way
                //build first function node
                let mut left = Expression::Function {
                    function_name: op_rule.as_str().to_string(),
                    params: vec![convert_to_ast(first_operand), convert_to_ast(child_pairs.next().unwrap())],
                };
                //append all other operands to the tree
                while let Some(op_rule) = child_pairs.next() {
                    left = Expression::Function {
                        function_name: op_rule.as_str().to_string(),
                        params: vec![left, convert_to_ast(child_pairs.next().unwrap())],
                    };
                }

                left
            } else {
                convert_to_ast(first_operand)
            }
        }
        Rule::not_operand => {
            let mut child_pairs = expr.into_inner();
            let first_node = child_pairs.next().unwrap();
            if let Rule::not_op = first_node.as_rule() {
                let second_node = child_pairs.next().unwrap();
                Expression::Function {
                    function_name: first_node.as_str().to_string(),
                    params: vec![convert_to_ast(second_node)],
                }
            } else {
                convert_to_ast(first_node)
            }
        }
        Rule::function_call => {
            let mut child_pairs = expr.into_inner();
            let function = child_pairs.next().unwrap();
            let params = child_pairs.next().unwrap();
            Expression::Function {
                function_name: function.as_str().to_string(),
                params: params.into_inner().map(convert_to_ast).collect(),
            }
        }
        Rule::string_literal => {
            Expression::Literal {
                value: LiteralValue::StringValue { value: expr.as_str().to_string() },
            }
        }
        Rule::integer => {
            Expression::Literal {
                value: LiteralValue::NumberValue { value: expr.as_str().parse::<f64>().unwrap() },
            }
        }
        Rule::float => {
            Expression::Literal {
                value: LiteralValue::NumberValue { value: expr.as_str().parse::<f64>().unwrap() },
            }
        }
        Rule::boolean_literal => {
            Expression::Literal {
                value: LiteralValue::BooleanValue { value: expr.as_str().parse::<bool>().unwrap() },
            }
        }
        Rule::field_reference => {
            let mut child_pairs = expr.into_inner();
            let field = child_pairs.next().unwrap();
            Expression::FieldReference {
                field_id: field.as_str().to_string(),
            }
        }
        _ => unreachable!()
    }
}

//eval simple arithmetic expressions
fn eval_ast(ast: Expression) -> f64 {
    match ast {
        Expression::Literal { value } => {
            match value {
                LiteralValue::NumberValue { value } => value,
                _ => unimplemented!()
            }
        }
        Expression::FieldReference { .. } => {
            unimplemented!()
        }
        Expression::Function { function_name, params } => {
            let params = params.into_iter().map(eval_ast).collect::<Vec<f64>>();
            match function_name.as_str() {
                "+" => params[0] + params[1],
                "-" => params[0] - params[1],
                "*" => params[0] * params[1],
                "/" => params[0] / params[1],
                _ => unimplemented!()
            }
        }
    }
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
            ("4 / 2 / 2", 1.0),
            ("6 + 1 + 2 + 1 + 15.3", 25.3),
            ("15 / 3 * 2 / 10", 1.0),
            ("2 + 3 + 4", 9.0),
            ("10 - 5 - 2", 3.0),
            ("3 * 4 * 5", 60.0),
            ("24 / 4 / 3", 2.0),
            ("2 + 3 * 4", 14.0),
            ("2 * 3 + 4", 10.0),
            ("10 - 2 * 3", 4.0),
            ("10 / 2 - 3", 2.0),
            ("(2 + 3) * 4", 20.0),
            ("(10 - 5) / 5", 1.0),
            ("2 * (3 + 4)", 14.0),
            ("10 - (5 - 2)", 7.0),
            ("(24 / 4) * 3", 18.0),
            ("6 / (3 * 2)", 1.0),
            ("(2 + 3) * (4 + 5)", 45.0),
            ("(10 - 5) / (5 - 2)", 1.6666666666666667),
            ("2 * (3 + 4) * (5 + 6)", 154.0),
            ("(24 / 4) * (3 + 2)", 30.0),
            ("6 / (3 * 2) / (1 + 1)", 0.5),
            ("(2 + 3) * (4 + 5) * (6 + 7)", 585.0),
            ("2.5 + 3.5 + 4.0", 10.0),
            ("10.5 - 5.5 - 2.0", 3.0),
            ("3.0 * 4.5 * 2.0", 27.0),
            ("24.0 / 4.0 / 3.0", 2.0),
            ("2.5 + 3.0 * 4.0", 14.5),
            ("2.0 * 3.5 + 4.0", 11.0),
            ("10.0 - 2.0 * 3.5", 3.0),
            ("10.0 / 2.5 - 3.0", 1.0),
            ("(2.5 + 3.5) * 4.0", 24.0),
            ("(10.5 - 5.0) / 5.5", 1.0),
            ("2.0 * (3.0 + 4.5)", 15.0),
            ("10.5 - (5.0 - 2.5)", 8.0),
            ("(24.0 / 4.0) * 3.5", 21.0),
            ("6.0 / (3.0 * 2.5)", 0.8),
            ("2.5 * 3.0 + 4.5 / 1.5 - 1.0", 9.5),
            ("10.0 / 2.0 * 3.5 - 4.0 + 1.5", 15.0),
            ("(2.0 + 3.5) * 4.0 - 10.0 / 2.0", 17.0),
            ("3.0 * (2.5 + 4.0 / 2.0) - 5.5", 8.0),
            ("2.0 * 3.0 + 4.0 * 5.0 - 6.0 / 2.0", 23.0),
            ("1.0 + 2.0 * 3.0 / 4.0 - 5.0 + 6.0 * 7.0", 39.5),
            ("3.0 * 4.0 - 5.0 * 6.0 / 2.0 + 7.0 * 2.0", 11.0),
            ("4.0 * (3.0 + 2.0) / 2.0 - 6.0 + 8.0 * 1.5", 16.0),
            ("2.0 / 4.0 + 6.0 * 8.0 / 10.0 * 3.0 - 5.0", 9.899999999999999),
            ("1.5 + 2.0 * (3.5 - 4.0 / 2.0) + 5.0 * 6.0", 34.5),
        ];

        for (input, expected_output) in test_cases {
            let result = eval_expression(input).unwrap();
            assert!((result - expected_output).abs() < f64::EPSILON, "Expression: {}, Result: {}, Expected: {}", input, result, expected_output);
        }
    }
}

#[cfg(test)]
mod expression_tests;