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
        assert!(result.is_err(), "Expression '{}' should be invalid", expr);
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
        assert!(
            result.is_ok(),
            "Expression '{}' should handle whitespace",
            expr
        );
    }
}

#[test]
fn test_invalid_whitespace_handling() {
    let expressions = vec!["field1 . field2 + 42", "field1. field2"];

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
            expr,
            result.unwrap_err()
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
        assert!(result.is_ok(), "Expression '{}' should allow periods", expr);
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
    let expressions = vec!["1field + 2field", "!field + &field", "field@ + field$"];

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
    let expressions = vec!["field✨ + field🚀", "field❤️ + field😊"];

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
            expr,
            result.unwrap_err()
        );
    }
}

#[test]
fn test_case_expressions() {
    let valid_expressions = vec![
        r#"CASE
         WHEN city = "Opelika" THEN "Op"
         WHEN city = "Brownsboro" THEN "Br"
         WHEN city = "Phoenix" THEN "Ph"
         WHEN city = "Alameda" THEN "Al"
         WHEN city = "Los Angeles" THEN "LA"
         WHEN city = "Buena Park" THEN "BP"
         WHEN city = "Sacramento" THEN "Sa"
         WHEN city = "Alta Loma" THEN "AL"
         WHEN city = "Ceres" THEN "Ce"
         WHEN city = "Aurora" THEN "Au"
         WHEN city = "Botsford" THEN "Bot"
         WHEN city = "Brooksville" THEN "Broo"
         WHEN city = "Balm" THEN "Ba"
         WHEN city = "Hialeah" THEN "Hia"
         WHEN city = "Austell" THEN "Au"
         WHEN city = "Alpharetta" THEN "Alpha"
         WHEN city = "Arlington Heights" THEN "AH"
         WHEN city = "Antioch" THEN "Ant"
         WHEN city = "Granger" THEN "Gra"
         WHEN city = "Bussey" THEN "Bus"
         WHEN city = "Lane" THEN "Lan"
         WHEN city = "Leonardville" THEN "Leon"
         WHEN city = "Aberdeen" THEN "Aber"
         ELSE city
         END"#,
        r#"CASE
         WHEN streamtype = 1 THEN upper("CDR")
         WHEN streamtype = 2 THEN upper("STATS")
         WHEN streamtype = 3 THEN upper("PDU")
         WHEN streamtype = 4 THEN upper("CDR")
         WHEN streamtype = 5 THEN upper("CDR")
         WHEN streamtype = 6 THEN upper("CDR")
         WHEN streamtype = 7 THEN upper("CDR")
         WHEN streamtype = 8 THEN upper("PDU")
         WHEN streamtype = 9 THEN upper("STATS")
         WHEN streamtype = 10 THEN upper("LOG")
         WHEN streamtype = 11 THEN upper("STATS")
         WHEN streamtype = 12 THEN upper("PDU")
         WHEN streamtype = 13 THEN upper("CDR")
         WHEN streamtype = 14 THEN upper("CDR")
         WHEN streamtype = 15 THEN upper("STATS")
         ELSE upper("UNKNOWN")
         END"#,
        r#"CASE
         WHEN sum(sales) > 100 THEN
            CASE
            WHEN avg(satisfaction) > 5 THEN "Awesome"
            ELSE "Nice"
            END
         WHEN sum(sales) > 50 THEN "OK"
         ELSE "Bad"
         END"#,
    ];

    for expr in valid_expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_ok(),
            "Expression '{}' should be a valid CASE expression, got error: {:?}",
            expr,
            result.unwrap_err()
        );
    }
}

#[test]
fn test_if_expressions() {
    let valid_expressions = vec![
        r#"if Salary > 3000 then "Alrite!" else "Dammit!""#,
        r#"if Sales >= 10000 then "High"
          else if Sales <= 2000 then "Low"
          else "Medium""#,
        r#"if Salary = true then "Alrite!" else "Dammit!""#,
        r#"if true = Salary then "Alrite!" else "Dammit!""#,
        r#"if (Salary > 3000) then "Alrite!" else "Dammit!""#,
        r#"if Salary > 3000 then (if Salary > 4000 then ">4000" else "<4000") else "<3000""#,
        r#"if(Salary > 3000, "Alrite!", "Dammit!")"#,
    ];

    for expr in valid_expressions {
        let result = ExpressionParser::parse(Rule::expression_input, expr);
        assert!(
            result.is_ok(),
            "Expression '{}' should be a valid IF expression, got error: {:?}",
            expr,
            result.unwrap_err()
        );
    }
}

#[test]
fn test_expressions_with_where() {
    let valid_expressions = vec![
        "sum(sales) [where city = \"Opelika\"]",
        "sum(sales) - sum(planned_sales) [where city = \"Opelika\"]",
        "(sum(sales) - sum(planned_sales)) [where city = \"Opelika\"]",
        "sum(sales) [where allow filters on city] - sum(planned_sales) [where allow filters on city])",
        "sum(sales) [where allow filters on city, state]",
        "sum(sales) [where allow filters on city, state and product = \"Book\"]",
        "sum(sales) [where allow filters on city, state and product = \"Book\" or product = \"Pen\"]",
        "sum(sales) [where ignore all filters]",
        "sum(sales) [where ignore all filters and product = \"Book\"]",
        "sum(sales) [where ignore filters on department]",
        "sum(sales) [where ignore filters on branch, department and year = 2012 and quarter <= 3]",
    ];

    for expr in valid_expressions {
        let result = dbg!(ExpressionParser::parse(Rule::expression_input, expr));
        assert!(
            result.is_ok(),
            "Expression '{}' should be a valid expression, got error: {:?}",
            expr,
            result.unwrap_err()
        );
    }
}

#[test]
fn test_expressions_with_partition() {
    let valid_expressions = vec![
        "sum(sales) [where city = \"Opelika\"]",
        "sum(sales) - sum(planned_sales) [where city = \"Opelika\"]",
        "(sum(sales) - sum(planned_sales)) [where city = \"Opelika\"]",
        "(sum(sales) - sum(planned_sales)) [where allow ]",
    ];

    for expr in valid_expressions {
        let result = dbg!(ExpressionParser::parse(Rule::expression_input, expr));
        assert!(
            result.is_ok(),
            "Expression '{}' should be a valid expression, got error: {:?}",
            expr,
            result.unwrap_err()
        );
    }
}

#[test]
fn test_expressions_with_partition_and_where() {
    let valid_expressions = vec![
        "sum(sales) [where city = \"Opelika\"]",
        "sum(sales) - sum(planned_sales) [where city = \"Opelika\"]",
        "(sum(sales) - sum(planned_sales)) [where city = \"Opelika\"]",
    ];

    for expr in valid_expressions {
        let result = dbg!(ExpressionParser::parse(Rule::expression_input, expr));
        assert!(
            result.is_ok(),
            "Expression '{}' should be a valid expression, got error: {:?}",
            expr,
            result.unwrap_err()
        );
    }
}
