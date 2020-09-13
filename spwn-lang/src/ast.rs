//! Abstract Syntax Tree (AST) type definitions

use std::path::PathBuf;

use crate::compiler_types::Value;

#[derive(Clone, PartialEq, Debug)]
pub enum DictDef {
    Def((String, Expression)),
    Extract(Expression),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Statement {
    pub body: StatementBody,
    pub arrow: bool, //context changing
    pub line: (usize, usize),
}

#[derive(Clone, PartialEq, Debug)]
pub enum StatementBody {
    Definition(Definition),
    Call(Call),
    Expr(Expression),

    TypeDef(String),

    Return(Option<Expression>),
    Impl(Implementation),
    If(If),
    For(For),
    Error(Error),
    Extract(Expression),
    //EOI,
}
#[derive(Clone, PartialEq, Debug)]
pub enum ValueLiteral {
    ID(ID),
    Number(f64),
    CmpStmt(CompoundStatement),
    Dictionary(Vec<DictDef>),
    Symbol(String),
    Bool(bool),
    Expression(Expression),
    Str(String),
    Import(PathBuf),
    Array(Vec<Expression>),
    Obj(Vec<(Expression, Expression)>),
    Macro(Macro),
    Resolved(Value),
    TypeIndicator(String),
    Null,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Operator {
    Or,
    And,
    Equal,
    NotEqual,
    Range,
    MoreOrEqual,
    LessOrEqual,
    More,
    Less,
    Slash,
    Star,
    Power,
    Plus,
    Minus,
    Modulo,

    Assign,
    Add,
    Subtract,
    Multiply,
    Divide,
}

#[derive(Clone, PartialEq, Debug)]
pub enum UnaryOperator {
    Not,
    Minus,
    Range,
}

#[derive(Clone, PartialEq, Debug)]
pub enum IDClass {
    Group,
    Color,
    Item,
    Block,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tag {
    pub tags: Vec<(String, Vec<Argument>)>,
}

impl Tag {
    pub fn new() -> Self {
        Tag { tags: Vec::new() }
    }
    pub fn get(&self, t: &str) -> Option<Vec<Argument>> {
        for (key, args) in &self.tags {
            if t == key {
                return Some(args.clone());
            }
        }
        return None;
    }

    pub fn get_desc(&self) -> Option<String> {
        match self.get("desc") {
            Some(args) => {
                if args.is_empty() {
                    None
                } else {
                    match &args[0].value.values[0].value {
                        ValueLiteral::Str(s) => Some(s.clone()),
                        a => Some(format!("{:?}", a)),
                    }
                }
            }

            None => None,
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub enum Path {
    Member(String),
    Index(Expression),
    Call(Vec<Argument>),
}

#[derive(Clone, PartialEq, Debug)]
pub struct Definition {
    pub symbol: String,
    pub value: Expression,
    //pub mutable: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Argument {
    pub symbol: Option<String>,
    pub value: Expression,
}

/*#[derive(Clone, PartialEq, Debug)]
pub struct Event {
    pub symbol: String,
    pub args: Vec<Expression>,
    pub func: Variable,
}*/

#[derive(Clone, PartialEq, Debug)]
pub struct Call {
    pub function: Variable,
}

/*#[derive(Clone, PartialEq, Debug)]
pub struct Native {
    pub function: Variable,
    pub args: Vec<Argument>,
}*/
//                 name         def value     props       type ind.
pub type ArgDef = (String, Option<Expression>, Tag, Option<Expression>);
#[derive(Clone, PartialEq, Debug)]
pub struct Macro {
    pub args: Vec<ArgDef>,
    pub body: CompoundStatement,
    pub properties: Tag,
}

#[derive(Clone, PartialEq, Debug)]
pub struct For {
    pub symbol: String,
    pub array: Expression,
    pub body: Vec<Statement>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Error {
    pub message: Expression,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Variable {
    pub operator: Option<UnaryOperator>,
    pub value: ValueLiteral,
    pub path: Vec<Path>,
}

/*impl Variable {
    pub fn to_expression(&self) -> Expression {
        if let ValueLiteral::Expression(e) = &self.value {
            if self.path.is_empty() {
                return e.to_owned();
            }
        }
        Expression {
            values: vec![self.clone()],
            operators: Vec::new(),
        }
    }
}*/

#[derive(Clone, PartialEq, Debug)]
pub struct Expression {
    pub values: Vec<Variable>,
    pub operators: Vec<Operator>,
}
impl Expression {
    pub fn to_variable(&self) -> Variable {
        Variable {
            operator: None,
            value: ValueLiteral::Expression(self.clone()),
            path: Vec::new(),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct CompoundStatement {
    pub statements: Vec<Statement>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct Implementation {
    pub symbol: Variable,
    pub members: Vec<DictDef>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct If {
    pub condition: Expression,
    pub if_body: Vec<Statement>,
    pub else_body: Option<Vec<Statement>>,
}

#[derive(Clone, PartialEq, Debug)]
pub struct ID {
    pub number: u16,
    pub unspecified: bool,
    pub class_name: IDClass,
}

pub fn str_content(inp: String) -> String {
    inp.clone().replace("\"", "")
    /*.replace("'", "")
    .replace("\r", "")
    .replace("\n", "")*/
}
