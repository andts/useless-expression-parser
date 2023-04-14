#[derive(Debug)]
pub enum Expression {
    Literal {
        value: LiteralValue,
    },
    FieldReference {
        field_id: String,
    },
    Function {
        function_name: String,
        params: Vec<Expression>,
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
