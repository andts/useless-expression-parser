#[derive(Debug)]
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
    },
}

#[derive(Debug)]
pub struct WhereModifier {
    filter_context: FilterContext,
    filters: Vec<Expression>,
}

#[derive(Debug)]
pub enum FilterContext {
    AllowedFilters {
        allowed_filters: Vec<Expression>,
    },
    IgnoredFilters {
        all: bool,
        ignored_filters: Vec<Expression>,
    },
}

#[derive(Debug)]
pub enum DataType {
    String,
    Number,
    Boolean,
}

#[derive(Debug)]
pub enum LiteralValue {
    StringValue { value: String },
    NumberValue { value: f64 },
    BooleanValue { value: bool },
}
