pub enum Expression {
    Literal {
        value_type: DataType,
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

pub enum DataType {
    String,
    Number,
    Boolean,
}

pub enum LiteralValue {
    StringValue { value: String },
    NumberValue { value: f64 },
    BooleanValue { value: bool },
}
