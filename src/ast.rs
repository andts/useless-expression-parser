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
        group_by_modifier: Option<WhereModifier>,
    },
}

#[derive(Debug)]
pub struct WhereModifier {
    filter_context: Option<FilterContext>,
    additional_filters: Vec<Expression>,
}

#[derive(Debug)]
pub enum FilterContext {
    AllowedFilters {
        allowed_filters: Vec<Expression>,
    },
    AllFiltersIgnored(),
    IgnoredFilters {
        ignored_filters: Vec<Expression>,
    },
}

#[derive(Debug)]
pub struct GroupByModifier {
    group_context: GroupByContext,
}

#[derive(Debug)]
pub enum GroupByContext {
    AllGroups(),
    IncludedGroups {
        groups: Vec<GroupReference>,
    },
}

#[derive(Debug)]
pub enum GroupReference {
    QueryGroup {
        index: usize,
    },
    FieldGroup {
        field: Expression,
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
