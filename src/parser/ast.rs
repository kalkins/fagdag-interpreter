use std::fmt::{Display, Formatter};

#[derive(Debug, PartialEq, Clone)]
pub struct Program {
    pub nodes: Vec<FunctionNode>
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionNode {
    pub name: Ident,
    pub parameters: Vec<FunctionParam>,
    pub return_type: Option<Type>,
    pub block: Vec<BlockNode>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct FunctionParam {
    pub name: Ident,
    pub type_name: Type,
}

#[derive(Debug, PartialEq, Clone)]
pub enum BlockNode {
    VariableDefinition { name: Ident, type_name: Type, value: ExpressionNode },
    Assignment { lhs: Ident, rhs: ExpressionNode },
    Expression(ExpressionNode),
    Block(Vec<BlockNode>),
    IfStatement { condition: ExpressionNode, block: Vec<BlockNode> },
    Return(ExpressionNode),
}

#[derive(Debug, PartialEq, Clone)]
pub enum ExpressionNode {
    BinaryOperation {
        verb: BinaryVerb,
        lhs: Box<ExpressionNode>,
        rhs: Box<ExpressionNode>,
    },
    Term(TermNode),
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinaryVerb {
    Plus,
    Minus,
    Compare,
}

#[derive(Debug, PartialEq, Clone)]
pub enum TermNode {
    Variable(Ident),
    Boolean(bool),
    Integer(i32),
    String(String),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Type {
    Int,
    Bool,
    String,
}

impl Display for Type {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Int => write!(f, "int"),
            Self::Bool => write!(f, "bool"),
            Self::String => write!(f, "string"),
        }
    }
}

pub type Ident = String;
