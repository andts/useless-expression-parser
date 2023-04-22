#[derive(Debug, PartialEq)]
pub enum Expression {
    Literal {
        value: LiteralValue,
        where_modifier: Option<WhereModifier>,
    },
    FieldReference {
        field_id: String,
        where_modifier: Option<WhereModifier>,
    },
    Function {
        function_name: String,
        params: Vec<Expression>,
        where_modifier: Option<WhereModifier>,
        group_by_modifier: Option<WhereModifier>,
    },
    IfExpression {
        condition: Box<Expression>,
        result: Box<Expression>,
        else_result: Box<Expression>,
        where_modifier: Option<WhereModifier>,
    },
    CaseExpression {
        cases: Vec<CaseBranch>,
        else_result: Box<Expression>,
        where_modifier: Option<WhereModifier>,
    },
}

#[derive(Debug, PartialEq)]
pub struct CaseBranch {
    pub condition: Expression,
    pub result: Expression,
}

#[derive(Debug, PartialEq)]
pub struct WhereModifier {
    pub filter_context: Option<FilterContext>,
    pub additional_filters: Vec<Expression>,
}

#[derive(Debug, PartialEq)]
pub enum FilterContext {
    AllowedFilters {
        allowed_filters: Vec<Expression>,
    },
    AllFiltersIgnored(),
    IgnoredFilters {
        ignored_filters: Vec<Expression>,
    },
}

#[derive(Debug, PartialEq)]
pub struct GroupByModifier {
    group_context: GroupByContext,
}

#[derive(Debug, PartialEq)]
pub enum GroupByContext {
    AllGroups(),
    IncludedGroups {
        groups: Vec<GroupReference>,
    },
}

#[derive(Debug, PartialEq)]
pub enum GroupReference {
    QueryGroup {
        index: usize,
    },
    FieldGroup {
        field: Expression,
    },
}

#[derive(Debug, PartialEq, Clone)]
pub enum LiteralValue {
    StringValue(String),
    NumberValue(f64),
    BooleanValue(bool),
}

impl Into<f64> for LiteralValue {
    fn into(self) -> f64 {
        match self {
            LiteralValue::NumberValue(value) => value,
            _ => panic!("Cannot convert non-number literal to number"),
        }
    }
}

impl Into<String> for LiteralValue {
    fn into(self) -> String {
        match self {
            LiteralValue::StringValue(value) => value,
            _ => panic!("Cannot convert non-string literal to string"),
        }
    }
}

impl Into<bool> for LiteralValue {
    fn into(self) -> bool {
        match self {
            LiteralValue::BooleanValue(value) => value,
            _ => panic!("Cannot convert non-boolean literal to boolean"),
        }
    }
}

//short funcs for tests
pub fn lit_str(value: &str) -> Expression {
    Expression::Literal {
        value: LiteralValue::StringValue(value.to_string()),
        where_modifier: None,
    }
}

pub fn lit_num(value: f64) -> Expression {
    Expression::Literal {
        value: LiteralValue::NumberValue(value),
        where_modifier: None,
    }
}

pub fn lit_bool(value: bool) -> Expression {
    Expression::Literal {
        value: LiteralValue::BooleanValue(value),
        where_modifier: None,
    }
}

pub fn field_ref(field_id: &str) -> Expression {
    Expression::FieldReference {
        field_id: field_id.to_string(),
        where_modifier: None,
    }
}

pub fn func(function_name: &str, params: Vec<Expression>) -> Expression {
    Expression::Function {
        function_name: function_name.to_string(),
        params,
        where_modifier: None,
        group_by_modifier: None,
    }
}

pub fn if_expr(condition: Expression, result: Expression, else_result: Expression) -> Expression {
    Expression::IfExpression {
        condition: Box::new(condition),
        result: Box::new(result),
        else_result: Box::new(else_result),
        where_modifier: None,
    }
}

pub fn case_expr(cases: Vec<CaseBranch>, else_result: Expression) -> Expression {
    Expression::CaseExpression {
        cases,
        else_result: Box::new(else_result),
        where_modifier: None,
    }
}

pub fn case_branch(condition: Expression, result: Expression) -> CaseBranch {
    CaseBranch { condition, result }
}

pub fn where_modifier(filter_context: Option<FilterContext>, additional_filters: Vec<Expression>) -> WhereModifier {
    WhereModifier { filter_context, additional_filters }
}

pub fn allowed_filters(allowed_filters: Vec<Expression>) -> FilterContext {
    FilterContext::AllowedFilters { allowed_filters }
}

pub fn ignored_filters(ignored_filters: Vec<Expression>) -> FilterContext {
    FilterContext::IgnoredFilters { ignored_filters }
}

pub fn ignore_all_filters() -> FilterContext {
    FilterContext::AllFiltersIgnored()
}

pub fn group_by_modifier(group_context: GroupByContext) -> GroupByModifier {
    GroupByModifier { group_context }
}

pub fn query_group(index: usize) -> GroupReference {
    GroupReference::QueryGroup { index }
}

pub fn field_group(field: Expression) -> GroupReference {
    GroupReference::FieldGroup { field }
}

pub fn all_groups() -> GroupByContext {
    GroupByContext::AllGroups()
}

pub fn included_groups(groups: Vec<GroupReference>) -> GroupByContext {
    GroupByContext::IncludedGroups { groups }
}


