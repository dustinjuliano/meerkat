use std::fmt::Display;
//use crate::runtime::Manager;
pub mod printer;
pub use printer::AstPrinter;
use crate::runtime::Symbol;
use crate::runtime::BindingId;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum UnOp {
    Neg, // negate
    Not, // logical not
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,

    Eq,
    Lt,
    Gt,

    And,
    Or,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum ActionStmt {
    Let {
        name: String,
        expr: Expr,
        name_id: Symbol,
        binding_id: BindingId,
    },
    Expr(Expr),
    Do(Expr),
    Assert(Expr),
    Assign {
        var: String,
        name_id: Symbol,
        binding_id: BindingId,
        expr: Expr,
    },
    Insert {
        row: Expr,
        table_name: String,
        table_name_id: Symbol,
        table_binding_id: BindingId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Stmt {
    ActionStmt(ActionStmt),
    Update {
        service: String,
        service_id: Symbol,
        service_binding_id: BindingId,
        decls: Vec<Decl>,
    },
    Connect { path: String, addr: String },
    Import {
        path: String,
        service: String,
        service_id: Symbol,
        service_binding_id: BindingId,
    },
    Service {
        name: String,
        name_id: Symbol,
        binding_id: BindingId,
        decls: Vec<Decl>,
    },
    Test {
        service: String,
        service_id: Symbol,
        service_binding_id: BindingId,
        stmts: Vec<ActionStmt>,
    },
    Watch { expr: Expr },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Value {
    Number {
        val: i32,
    },
    Bool {
        val: bool,
    },
    String {
        val: String,
    },
    Closure {
        params: Vec<String>,
        param_ids: Vec<(Symbol, BindingId)>,
        body: Box<Expr>,
        env: Vec<(String, Value)>,
        service_name: String,
        service_name_id: Symbol,
        service_name_binding_id: BindingId,
    },
    ActionClosure {
        stmts: Vec<ActionStmt>,
        env: Vec<(String, Value)>,
        service_name: String,
        service_name_id: Symbol,
        service_name_binding_id: BindingId,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum Expr {
    /// Basic Lambda Core expressions
    Literal {
        val: Value,
    },
    Variable {
        ident: String,
        name_id: Symbol,
        binding_id: BindingId,
    },
    Tuple {
        val: Vec<Expr>
    },
    KeyVal {    // TODO: replace with a Record type (different from Tuple) that is a list of key value pairs
        key: String,
        key_id: Symbol,
        key_binding_id: BindingId,
        value: Box<Expr>,
    },
    Unop {
        op: UnOp,
        expr: Box<Expr>,
    },
    Binop {
        op: BinOp,
        expr1: Box<Expr>,
        expr2: Box<Expr>,
    },

    If {
        cond: Box<Expr>,
        expr1: Box<Expr>,
        expr2: Box<Expr>,
    },

    Func {
        params: Vec<String>,
        param_ids: Vec<(Symbol, BindingId)>,
        body: Box<Expr>,
    },
    Call {
        func: Box<Expr>,
        args: Vec<Expr>,
    },

    /// Action
    Action(Vec<ActionStmt>),

    MemberAccess {
        service: String,
        service_id: Symbol,
        service_binding_id: BindingId,
        member: String,
        member_id: Symbol,
        member_binding_id: BindingId,
    },
    Select {
        table_name: String,
        table_name_id: Symbol,
        table_binding_id: BindingId,
        column_names: Vec<String>,
        column_ids: Vec<(Symbol, BindingId)>,
        where_clause: Box<Expr>,
    },

    Table { // TODO: remove this, we should just have Records and Tuples
        schema: Vec<Field>,
        records: Vec<Expr>,
        /*How do records differ from rows?
          Records only consist of data contained within tables: {1, "A", 18}
          Rows are what are written inside insert statements, insert {id: 1, name: "A", age: 18};
         */
    },
    Fold {
        table_name: String,
        table_name_id: Symbol,
        table_binding_id: BindingId,
        column_name: String,
        column_name_id: Symbol,
        column_binding_id: BindingId,
        operation: Box<Expr>,
        identity: Box<Expr>,
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Decl {
    VarDecl {
        name: String,
        name_id: Symbol,
        binding_id: BindingId,
        val: Expr,
    },
    DefDecl {
        name: String,
        name_id: Symbol,
        binding_id: BindingId,
        val: Expr,
        is_pub: bool,
    },
    TableDecl {
        name: String,
        name_id: Symbol,
        binding_id: BindingId,
        fields: Vec<Field>,
    },
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub struct Field {
    pub name: String,
    pub name_id: Symbol,
    pub binding_id: BindingId,
    pub type_: DataType,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, serde::Serialize, serde::Deserialize)]
pub enum DataType {
    String,
    Number,
    Bool,
}

impl Display for UnOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            UnOp::Neg => write!(f, "-"),
            UnOp::Not => write!(f, "!"),
        }
    }
}

impl Display for BinOp {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            BinOp::Add => write!(f, "+"),
            BinOp::Sub => write!(f, "-"),
            BinOp::Mul => write!(f, "*"),
            BinOp::Div => write!(f, "/"),
            BinOp::Eq => write!(f, "=="),
            BinOp::Lt => write!(f, "<"),
            BinOp::Gt => write!(f, ">"),
            BinOp::And => write!(f, "&&"),
            BinOp::Or => write!(f, "||"),
        }
    }
}

impl Display for Value {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Value::Number { val } => write!(f, "{}", val),
            Value::Bool { val } => write!(f, "{}", val),
            Value::String { val } => write!(f, "\"{}\"", val),
            Value::Closure { params, body, env, .. } =>
                write!(f, "fn({})[{:?}]{{{}}}", params.join(","), env, body),
            Value::ActionClosure { stmts, env, service_name, .. } =>
                write!(f, "action[{:?}][{}]{{{:?}}}", env, service_name, stmts),  
        }
    }
}

impl Display for Expr {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Expr::Literal { val } => write!(f, "{}", val),
            Expr::Tuple { .. } => write!(f, "vector"),
            Expr::KeyVal { key, value, .. } => write!(f, "keyval: {}, {}", key, value),
            Expr::Variable { ident, .. } => write!(f, "{}", ident),
            Expr::Unop { op, expr } => write!(f, "{}{}", op, expr),
            Expr::Binop { op, expr1, expr2 } => write!(f, "{} {} {}", expr1, op, expr2),
            Expr::If { cond, expr1, expr2 } => {
                write!(f, "if {} then {} else {}", cond, expr1, expr2)
            }
            Expr::Func { params, body, .. } => write!(f, "fn({})[{}]", params.join(","), body),
            Expr::Call { func, args } =>
                write!(
                    f,
                    "{}({})",
                    func,
                    args.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
                ),
            Expr::Action(stmts) =>
                write!(
                    f,
                    "Action({:?})",
                    stmts.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
                ),
            Expr::MemberAccess { service, member, .. } => write!(f, "{}.{}", service, member),
            Expr::Select { where_clause, .. } => write!(f, "{}", where_clause),
            Expr::Table {records , ..} => {
                write!(f, "[",)?;
                for (i, record) in records.iter().enumerate() {
                    if i > 0 {
                        write!(f, ", ")?;
                    }
                    write!(f, "{{")?;
                    match record {
                        Expr::Tuple { val } => {
                            for (j, entry) in val.iter().enumerate() {
                                if j > 0 {
                                write!(f, ", ")?;
                                }
                                write!(f, "{}", entry)?;
                            }
                        },
                        other => {
                            write!(f, "{}", other)?;
                        }
                    }
                write!(f, "}}")?;
            }
            write!(f, "]")
            },
            Expr::Fold { .. } => write!(f, "fold")
        }
    }
}

impl Display for ActionStmt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ActionStmt::Let { name, expr, .. } => write!(f, "let {} = {}", name, expr),
            ActionStmt::Expr(expr) => write!(f, "{}", expr),
            ActionStmt::Do(expr) => write!(f, "do {}", expr),
            ActionStmt::Assert(expr) => write!(f, "assert {}", expr),
            ActionStmt::Assign { var, expr, .. } => write!(f, "{} = {}", var, expr),
            ActionStmt::Insert { row, table_name, .. } => write!(f, "insert into {} {}", table_name, row),
        }
    }
}

impl Display for Decl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Decl::VarDecl { name, val, .. } => { write!(f, "var {} = {}", name, val) },
            Decl::DefDecl { name, val, is_pub, .. } => {
                if *is_pub {
                    write!(f, "pub def {} = {}", name, val)
                } else {
                    write!(f, "def {} = {}", name, val)
                }
            },
            Decl::TableDecl { name, .. } => { write!(f, "table {} created", name) }
        }
    }
}
