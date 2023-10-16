use crate::{ExpressionParser, Rule};
use pest::Parser;

macro_rules! parse_success {
    ($name:ident, $( $expr:literal ),+ ) => {
        #[test]
        fn $name() {
            let expressions = vec![
                $( $expr ),+
            ];

            for expr in expressions {
                let result = ExpressionParser::parse(Rule::expression_input, expr);
                assert!(result.is_ok(), "Expression '{}' should be valid for {} suite", expr, stringify!($name));
            }
        }
    };
}

macro_rules! parse_failure {
    ($name:ident, $( $expr:literal ),+ ) => {
        #[test]
        fn $name() {
            let expressions = vec![
                $( $expr ),+
            ];

            for expr in expressions {
                let result = ExpressionParser::parse(Rule::expression_input, expr);
                assert!(result.is_err(), "Expression '{}' should be invalid for {} suite", expr, stringify!($name));
            }
        }
    };
}

parse_success!(
    arithmetic_expressions,
    "field1 + 4e2",
    "field1.field2 * 3.14",
    "(field1 + field2) * (field3 / field4)",
    "field1 - field2.field3 + 42 * 7.5"
);

parse_success!(
    edge_cases,
    "field",
    "1",
    "1.0",
    "field.field",
    "field1 + field2",
    "(field)",
    "field * (field1 + field2)"
);

parse_success!(
    whitespace_handling,
    " field1 + 42",
    "field1 +\t42",
    "field1\n+\n42",
    "( field1 + field2 ) * ( field3 / field4 )"
);

parse_failure!(
    invalid_arithmetic_expressions,
    "field1 + ",
    "field1 + + field2",
    "field1.field2. * 3.14",
    "(field1 + field2) * field3 /",
    "field1 - field2.field3 + 42 7.5"
);

parse_failure!(
    invalid_whitespace_handling,
    "field1 . field2 + 42",
    "field1. field2"
);

parse_success!(
    latin_letters,
    "field1 + field2",
    "Field1 - field2",
    "fieldWithUpperCaseLetters + anotherField"
);

parse_success!(
    numbers,
    "field1 + 42",
    "field1 - 3.14",
    "field1 * 1.23e-4",
    "field1 * 15e48"
);

parse_success!(
    periods,
    "field1.field2 + field3" // "field1.field2.field3",
);

parse_success!(
    underscores,
    "field_1 + field_2",
    "fie__ld1.field_2",
    "field_name_with_underscore",
    "__field_name_with_under____score"
);

parse_failure!(
    not_allowed_identifiers,
    "1field + 2field",
    "!field + &field",
    "field@ + field$"
);

parse_success!(
    non_latin_characters,
    "поле1 + поле2",
    "フィールド1 + フィールド2",
    "字段1 + 字段2"
);

parse_failure!(
    special_symbols_in_identifiers,
    "field✨ + field🚀",
    "field❤️ + field😊"
);

parse_success!(
    mixed_identifiers,
    "field1_поле + field2",
    "field1_フィールド + field2",
    "field1_字段 + field2"
);

parse_success!(
    function_calls,
    "my_function(field1)",
    "my_function(42)",
    "my_function(3.14)",
    "my_function(\"hello\")",
    "my_function(field1, field2)",
    "my_function(field1, 42, 3.14, \"hello\")",
    "my_function(field1, nested_function(field2))",
    "my_function(field1) + my_function(field2)",
    "my_function(nested_function(field1))"
);

parse_failure!(
    invalid_function_calls,
    "my_function(",
    "my_function)",
    "my_function(,)",
    "my_function(42,)",
    "my_function(,42)",
    "my_function(42, ,)",
    "my_function(42,,42)"
);

parse_success!(
    boolean_literals,
    "true",
    "false",
    "my_function(true)",
    "my_function(false)",
    "my_function(field1, true, 42, \"hello\")",
    "(true)",
    "(false)"
);

parse_success!(
    case_expressions,
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
        END"#
);

parse_success!(
    if_expressions,
    r#"if Salary > 3000 then "Alrite!" else "Dammit!""#,
    r#"if Sales >= 10000 then "High"
          else if Sales <= 2000 then "Low"
          else "Medium""#,
    r#"if Salary = true then "Alrite!" else "Dammit!""#,
    r#"if true = Salary then "Alrite!" else "Dammit!""#,
    r#"if (Salary > 3000) then "Alrite!" else "Dammit!""#,
    r#"if Salary > 3000 then (if Salary > 4000 then ">4000" else "<4000") else "<3000""#,
    r#"if(Salary > 3000, "Alrite!", "Dammit!")"#
);

parse_success!(
    expressions_with_where,
    "sum(sales) [where city = \"Opelika\"]",
    "sum(sales) - sum(planned_sales) [where city = \"Opelika\"]",
    "(sum(sales) - sum(planned_sales)) [where city = \"Opelika\"]",
    "sum(sales) [where allow filters on city] - sum(planned_sales) [where allow filters on city]",
    "sum(sales) [where allow filters on city, state]",
    "sum(sales) [where allow filters on city, state and product = \"Book\"]",
    "sum(sales) [where allow filters on city, state and product = \"Book\" or product = \"Pen\"]",
    "sum(sales) [where ignore all filters]",
    "sum(sales) [where ignore all filters and product = \"Book\"]",
    "sum(sales) [where ignore filters on department]",
    "sum(sales) [where ignore filters on branch, department and year = 2012 and quarter <= 3]"
);

parse_success!(
    expressions_with_partition,
    "sum(sales) [group by city]",
    "sum(sales) [group by state, city]",
    "sum(sales) [group by all groups]",
    "sum(sales) [group by group(1), group(4)]"
);

parse_success!(
    expressions_with_partition_and_where,
        // "sum(sales) [where city = \"Opelika\"] [group by product]",
        // "sum(sales) [group by product] [where city = \"Opelika\"] ",
    "sum(sales) [group by product] [where city = \"Opelika\"] * avg(sales) + max(sales) [where product = \"Book\"] "
);
