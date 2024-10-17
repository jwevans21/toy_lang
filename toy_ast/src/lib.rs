#[derive(Debug)]
#[non_exhaustive]
pub enum ToyTopLevelExpression<'src> {
    FunctionDefinition {
        name: &'src str,
        parameters: Vec<(&'src str, ToyType<'src>)>,
        body: Vec<ToyStatement<'src>>,
    },
    ExternalFunctionDeclaration {
        name: &'src str,
        parameters: Vec<ToyType<'src>>,
        return_type: ToyType<'src>,
    },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ToyType<'src> {
    Int,
    Float,
    Bool,
    String,
    UserDefined(&'src str),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ToyStatement<'src> {
    VariableDeclaration {
        name: &'src str,
        ty: Option<ToyType<'src>>,
        value: ToyExpression<'src>,
    },
    Block {
        statements: Vec<ToyStatement<'src>>,
    },
    If {
        condition: ToyExpression<'src>,
        then_branch: Box<ToyStatement<'src>>,
        else_branch: Box<ToyStatement<'src>>,
    },
    While {
        condition: ToyExpression<'src>,
        body: Box<ToyStatement<'src>>,
    },
    Return {
        value: Option<Box<ToyExpression<'src>>>,
    },    
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ToyExpression<'src> {
    Literal {
        value: ToyLiteral,
    },
    Identifier {
        name: &'src str,
    },
    Binary {
        lhs: Box<ToyExpression<'src>>,
        operator: ToyBinaryOperator,
        rhs: Box<ToyExpression<'src>>,
    },
    Unary {
        operator: ToyUnaryOperator,
        operand: Box<ToyExpression<'src>>,
    },
    Call {
        function: &'src str,
        arguments: Vec<ToyExpression<'src>>,
    },
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ToyLiteral {
    Int(i64),
    Bool(bool),
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ToyBinaryOperator {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo,
    Equality,
    NotEqual,
    GreaterThan,
    GreaterThanEqual,
    LessThan,
    LessThanEqual,
    Assign,
}

#[derive(Debug)]
#[non_exhaustive]
pub enum ToyUnaryOperator {
    Negate,
    Not,
}