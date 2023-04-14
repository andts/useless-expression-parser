use pest::Parser;

use crate::{ExpressionParser, Rule};

#[test]
fn test_valid_arithmetic_expressions() {
    let expressions = vec![
        "field1 + 42",
        "field1.field2 * 3.14",
        "(field1 + field2) * (field3 / field4)",
        "field1 - field2.field3 + 42 * 7.5",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(result.is_ok(), "Expression '{}' should be valid", expr);
    }
}

#[test]
fn test_invalid_arithmetic_expressions() {
    let expressions = vec![
        "field1 + ",
        "field1 + + field2",
        "field1.field2. * 3.14",
        "(field1 + field2) * field3 /",
        "field1 - field2.field3 + 42 7.5",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_err(),
            "Expression '{}' should be invalid",
            expr
        );
    }
}

#[test]
fn test_edge_cases() {
    let expressions = vec![
        "field",
        "1",
        "1.0",
        "field.field",
        "field1 + field2",
        "(field)",
        "field * (field1 + field2)",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(result.is_ok(), "Expression '{}' should be valid", expr);
    }
}

#[test]
fn test_whitespace_handling() {
    let expressions = vec![
        " field1 + 42",
        "field1 +\t42",
        "field1\n+\n42",
        "( field1 + field2 ) * ( field3 / field4 )",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(result.is_ok(), "Expression '{}' should handle whitespace", expr);
    }
}

#[test]
fn test_invalid_whitespace_handling() {
    let expressions = vec![
        "field1 . field2 + 42",
        "field1. field2",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_err(),
            "Expression '{}' should not allow invalid whitespace",
            expr
        );
    }
}

#[test]
fn test_latin_letters() {
    let expressions = vec![
        "field1 + field2",
        "Field1 - field2",
        "fieldWithUpperCaseLetters + anotherField",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_ok(),
            "Expression '{}' should allow Latin letters, got error: {:?}",
            expr, result.unwrap_err()
        );
    }
}

#[test]
fn test_numbers() {
    let expressions = vec![
        "field1 + 42",
        "field1 - 3.14",
        "field1 * 1.23e-4",
        "field1 * 15e48",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(result.is_ok(), "Expression '{}' should allow numbers", expr);
    }
}

#[test]
fn test_periods() {
    let expressions = vec![
        "field1.field2 + field3",
        // "field1.field2.field3",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_ok(),
            "Expression '{}' should allow periods",
            expr
        );
    }
}

#[test]
fn test_underscores() {
    let expressions = vec![
        "field_1 + field_2",
        "fie__ld1.field_2",
        "field_name_with_underscore",
        "__field_name_with_under____score",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_ok(),
            "Expression '{}' should allow underscores",
            expr
        );
    }
}

#[test]
fn test_not_allowed_identifiers() {
    let expressions = vec![
        "1field + 2field",
        "!field + &field",
        "field@ + field$",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_err(),
            "Expression '{}' should not allow invalid identifiers",
            expr
        );
    }
}

#[test]
fn test_non_latin_characters() {
    let expressions = vec![
        "поле1 + поле2",
        "フィールド1 + フィールド2",
        "字段1 + 字段2",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_ok(),
            "Expression '{}' should allow non-latin characters",
            expr
        );
    }
}

#[test]
fn test_special_symbols() {
    let expressions = vec![
        "field✨ + field🚀",
        "field❤️ + field😊",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_err(),
            "Expression '{}' should not allow special symbols like emojis",
            expr
        );
    }
}

#[test]
fn test_mixed_identifiers() {
    let expressions = vec![
        "field1_поле + field2",
        "field1_フィールド + field2",
        "field1_字段 + field2",
    ];

    for expr in expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_ok(),
            "Expression '{}' should allow mixed-character identifiers",
            expr
        );
    }
}

#[test]
fn test_function_calls() {
    let valid_expressions = vec![
        "my_function(field1)",
        "my_function(42)",
        "my_function(3.14)",
        "my_function(\"hello\")",
        "my_function(field1, field2)",
        "my_function(field1, 42, 3.14, \"hello\")",
        "my_function(field1, nested_function(field2))",
        "my_function(field1) + my_function(field2)",
        "my_function(nested_function(field1))",
    ];

    for expr in valid_expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        println!("{:#?}", result);
        assert!(
            result.is_ok(),
            "Expression '{}' should be a valid function call",
            expr
        );
    }

    let invalid_expressions = vec![
        "my_function(",
        "my_function)",
        "my_function(,)",
        "my_function(42,)",
        "my_function(,42)",
        "my_function(42, ,)",
        "my_function(42,,42)",
    ];

    for expr in invalid_expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_err(),
            "Expression '{}' should be an invalid function call",
            expr
        );
    }
}

#[test]
fn test_boolean_literals() {
    let valid_expressions = vec![
        "true",
        "false",
        "my_function(true)",
        "my_function(false)",
        "my_function(field1, true, 42, \"hello\")",
        "(true)",
        "(false)",
    ];

    for expr in valid_expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_ok(),
            "Expression '{}' should be a valid boolean expression, got error: {:?}",
            expr, result.unwrap_err()
        );
    }
}

#[test]
fn test_operator_precedence_parsing() {
    let input = "2 * (3 + 4) / 7 > 1 and true or false and 3 * (5 - 1) < 12";
    let parse_result = ExpressionParser::parse(Rule::expression_input, input);
    assert!(parse_result.is_ok(), "Failed to parse input expression");

    let parse_tree = parse_result.unwrap();
/*
    // You can implement your custom validation logic here.
    // For example, you can count the number of specific rule occurrences in the parse tree.
    let mut comparison_count = 0;
    let mut and_count = 0;
    let mut or_count = 0;

    for pair in parse_tree.clone().flatten() {
        match pair.as_rule() {
            Rule::comparison_operator => comparison_count += 1,
            Rule::and_op => and_count += 1,
            Rule::or_op => or_count += 1,
            _ => (),
        }
    }

    assert_eq!(comparison_count, 2, "Incorrect number of comparison operators");
    assert_eq!(and_count, 2, "Incorrect number of AND operators");
    assert_eq!(or_count, 1, "Incorrect number of OR operators");
*/
    // Optionally, you can print the parse tree for visual inspection.
    println!("{:#?}", parse_tree);
}