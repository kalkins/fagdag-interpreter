#[derive(Debug, PartialEq)]
pub struct Program {
    pub nodes: Vec<RootNode>
}

#[derive(Debug, PartialEq)]
pub enum RootNode {
    Function {
        name: Ident,
        parameters: Vec<FunctionParam>,
        return_type: Option<Type>,
        block: Vec<BlockNode>,
    },
}

#[derive(Debug, PartialEq)]
pub struct FunctionParam {
    pub name: Ident,
    pub type_name: Type,
}

#[derive(Debug, PartialEq)]
pub enum BlockNode {
    VariableDefinition { name: Ident, type_name: Type, value: ExpressionNode },
    Assignment { lhs: Ident, rhs: ExpressionNode },
    Return(ExpressionNode),
}

#[derive(Debug, PartialEq)]
pub enum ExpressionNode {
    BinaryOperation {
        verb: BinaryVerb,
        lhs: Box<ExpressionNode>,
        rhs: Box<ExpressionNode>,
    },
    Term(TermNode),
}

#[derive(Debug, PartialEq)]
pub enum BinaryVerb {
    Plus,
    Minus,
    Compare,
}

#[derive(Debug, PartialEq)]
pub enum TermNode {
    Variable(Ident),
    Boolean(bool),
    Integer(i32),
    String(String),
}

#[derive(Debug, PartialEq)]
pub enum Type {
    Int,
    Bool,
    String,
}

#[derive(Debug, PartialEq)]
pub struct Ident {
    name: String
}

impl<T: ToString> From<T> for Ident {
    fn from(value: T) -> Self {
        Ident { name: value.to_string() }
    }
}
