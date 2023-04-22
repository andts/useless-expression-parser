use std::collections::HashMap;
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
    let numeric_expression = eval_ast(ast, &HashMap::new())?;
    Ok(numeric_expression.into())
}

pub fn convert_to_ast(expr: Pair<Rule>) -> Expression {
    match expr.as_rule() {
        Rule::or_operand | Rule::and_operand => {
            let mut child_pairs = expr.into_inner();
            let first_operand = child_pairs.next().unwrap();
            let operator = child_pairs.next();
            // if there is more than one child node, then this is a full expression with an operator
            if let Some(rule) = operator {
                //collect all child nodes that are not or_operand or and_operand
                let mut params: Vec<Expression> = vec![convert_to_ast(first_operand)];
                let mut where_modifier: Option<WhereModifier> = None;
                while let Some(child) = child_pairs.next() {
                    let child_rule = child.as_rule();
                    match child_rule {
                        Rule::where_clause => {
                            where_modifier = convert_to_where_modifier(child);
                        }
                        c if c != Rule::or_operand && c != Rule::and_operand => {
                            params.push(convert_to_ast(child));
                        }
                        _ => {}
                    }
                }

                Expression::Function {
                    function_name: rule.as_str().to_string(),
                    params,
                    where_modifier,
                    group_by_modifier: None,
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
                    params: vec![
                        convert_to_ast(first_operand),
                        convert_to_ast(child_pairs.next().unwrap()),
                    ],
                    where_modifier: None,
                    group_by_modifier: None,
                };
                //append all other operands to the tree
                while let Some(op_rule) = child_pairs.next() {
                    left = Expression::Function {
                        function_name: op_rule.as_str().to_string(),
                        params: vec![left, convert_to_ast(child_pairs.next().unwrap())],
                        where_modifier: None,
                        group_by_modifier: None,
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
                    where_modifier: None,
                    group_by_modifier: None,
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
                where_modifier: None,
                group_by_modifier: None,
            }
        }
        Rule::if_expr => {
            let mut children = expr.into_inner();
            Expression::IfExpression {
                condition: Box::new(convert_to_ast(children.next().unwrap())),
                result: Box::new(convert_to_ast(children.next().unwrap())),
                else_result: Box::new(convert_to_ast(children.next().unwrap())),
                where_modifier: None,
            }
        }
        Rule::string_literal => Expression::Literal {
            value: LiteralValue::StringValue(expr.as_str().to_string()),
            where_modifier: None,
        },
        Rule::integer => Expression::Literal {
            value: LiteralValue::NumberValue(expr.as_str().parse::<f64>().unwrap()),
            where_modifier: None,
        },
        Rule::float => Expression::Literal {
            value: LiteralValue::NumberValue(expr.as_str().parse::<f64>().unwrap()),
            where_modifier: None,
        },
        Rule::boolean_literal => Expression::Literal {
            value: LiteralValue::BooleanValue(expr.as_str().parse::<bool>().unwrap()),
            where_modifier: None,
        },
        Rule::field_reference => {
            Expression::FieldReference {
                field_id: expr.as_str().to_string(),
                where_modifier: None,
            }
        }
        _ => unreachable!(),
    }
}

fn convert_to_group_by_modifier(child: Pair<Rule>) -> Option<GroupByModifier> {
    // match child.as_rule() {
    //     Rule::window_clause => {
    //         let mut child_pairs = child.into_inner();
    //         let first_node = child_pairs.next().unwrap();
    //         if let Rule::window_clause = first_node.as_rule() {
    //             let second_node = child_pairs.next().unwrap();
    //             GroupByModifier::Window {
    //                 expression: convert_to_ast(second_node),
    //             }
    //         } else {
    //             unreachable!()
    //         }
    //     }
    //     _ => unreachable!(),
    // }
    unimplemented!()
}

fn convert_to_where_modifier(where_clause_node: Pair<Rule>) -> Option<WhereModifier> {
    if let Rule::where_clause = where_clause_node.as_rule() {
        let mut child_pairs = where_clause_node.into_inner();
        let first_node = child_pairs.next().unwrap();
        //first node can be an allowed or ignored filters list, or a filter expression
        let mut filter_context: Option<FilterContext> = None;
        let mut additional_filters: Vec<Expression> = vec![];
        match first_node.as_rule() {
            Rule::allow_field_filters => {
                filter_context = Some(FilterContext::AllowedFilters {
                    allowed_filters: first_node
                        .into_inner()
                        .map(|node| convert_to_ast(node))
                        .collect(),
                });
            }
            Rule::ignore_field_filters => {
                let mut children = first_node.into_inner();
                let first_child = children.next().unwrap();
                if let Rule::ignore_all_filters = first_child.as_rule() {
                    filter_context = Some(FilterContext::AllFiltersIgnored());
                } else {
                    let mut ignored_filters = vec![convert_to_ast(first_child)];
                    children.for_each(|f| ignored_filters.push(convert_to_ast(f)));
                    filter_context = Some(FilterContext::IgnoredFilters { ignored_filters });
                }
            }
            Rule::filter_expr => {
                additional_filters.push(convert_to_ast(first_node));
            }
            _ => unreachable!(),
        }

        while let Some(next_node) = child_pairs.next() {
            additional_filters.push(convert_to_ast(next_node));
        }

        Some(WhereModifier {
            filter_context,
            additional_filters,
        })
    } else {
        None
    }
}

//eval simple arithmetic expressions
fn eval_ast(ast: Expression, ctx: &HashMap<String, LiteralValue>) -> Result<LiteralValue, String> {
    match ast {
        Expression::Literal { value, .. } => Ok(value),
        Expression::FieldReference { field_id, .. } => {
            if let Some(field_value) = ctx.get(&field_id) {
                Ok(field_value.clone())
            } else {
                Err(format!("Field {} not found in context", field_id))
            }
        }
        Expression::Function {
            function_name,
            params,
            ..
        } => {
            let params: Result<Vec<LiteralValue>, String> = params.into_iter().map(|p| eval_ast(p, ctx)).collect();
            let params = params?;
            match function_name.as_str() {
                "+" => {
                    let params: Vec<f64> = params.into_iter().map(|p| p.into()).collect();
                    Ok(LiteralValue::NumberValue(params[0] + params[1]))
                }
                "-" => {
                    let params: Vec<f64> = params.into_iter().map(|p| p.into()).collect();
                    Ok(LiteralValue::NumberValue(params[0] - params[1]))
                }
                "*" => {
                    let params: Vec<f64> = params.into_iter().map(|p| p.into()).collect();
                    Ok(LiteralValue::NumberValue(params[0] * params[1]))
                }
                "/" => {
                    let params: Vec<f64> = params.into_iter().map(|p| p.into()).collect();
                    Ok(LiteralValue::NumberValue(params[0] / params[1]))
                }
                //todo add comparison and boolean funcs
                f => Err(format!("Unknown function {}", f))
            }
        }
        Expression::IfExpression {
            condition,
            result,
            else_result,
            ..
        } => {
            if let Ok(LiteralValue::BooleanValue(true)) = eval_ast(*condition, ctx) {
                eval_ast(*result, ctx)
            } else {
                eval_ast(*else_result, ctx)
            }
        }
        Expression::CaseExpression {
            cases,
            else_result,
            ..
        } => {
            for case in cases {
                if let Ok(LiteralValue::BooleanValue(true)) = eval_ast(case.condition, ctx) {
                    return eval_ast(case.result, ctx);
                }
            }
            eval_ast(*else_result, ctx)
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
            ("1.5 + 2.0 * (3.5 - 4.0 / 2.0) + 5.0 * 6.0", 34.5),
        ];

        for (input, expected_output) in test_cases {
            let result = eval_expression(input).unwrap();
            assert!(
                (result - expected_output).abs() < f64::EPSILON,
                "Expression: {}, Result: {}, Expected: {}",
                input,
                result,
                expected_output
            );
        }
    }
}

#[cfg(test)]
mod expression_parsing_tests;

#[cfg(test)]
mod expression_ast_tests;
