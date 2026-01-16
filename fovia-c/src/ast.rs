#[derive(Debug, Clone)]
pub struct Program {
    pub functions: Vec<Function>,
    pub structs: Vec<StructDef>,
    pub traits: Vec<TraitDef>,
    pub impls: Vec<ImplDef>,
}

#[derive(Debug, Clone)]
pub struct Function {
    pub name: String,
    pub params: Vec<Param>,
    pub body: Vec<Stmt>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub enum Expr {
    Ident(String),
    StrLiteral(Vec<u8>),
    IntLiteral(i64),
    BoolLiteral(bool),
    Call { name: String, args: Vec<Expr> },
    Binary {
        left: Box<Expr>,
        op: BinOp,
        right: Box<Expr>,
    },
    Loop(LoopExpr),
    Match(MatchExpr),
    Block(Block),
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Eq,
    Ne,
    Lt,
    Gt,
    Le,
    Ge,
}

#[derive(Debug, Clone)]
pub enum Stmt {
    Call { name: String, args: Vec<Expr> },
    ReactorStdout { arg: Expr },
    Let { name: String, ty: String, expr: Expr },
    Assign { name: String, expr: Expr },
    Break { label: Option<String>, expr: Expr },
    ExprStmt(Expr),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
    pub tail: Option<Box<Expr>>,
}

#[derive(Debug, Clone)]
pub struct LoopExpr {
    pub label: Option<String>,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub struct MatchExpr {
    pub scrutinee: Box<Expr>,
    pub arms: Vec<MatchArm>,
}

#[derive(Debug, Clone)]
pub struct MatchArm {
    pub pattern: MatchPattern,
    pub body: Block,
}

#[derive(Debug, Clone)]
pub enum MatchPattern {
    LessThan(i64),
    LessEqual(i64),
    Equal(i64),
    NotEqual(i64),
    GreaterThan(i64),
    GreaterEqual(i64),
    Wildcard,
}

#[derive(Debug, Clone)]
pub struct StructDef {
    pub name: String,
    pub fields: Vec<StructField>,
}

#[derive(Debug, Clone)]
pub struct StructField {
    pub name: String,
    pub ty: String,
}

#[derive(Debug, Clone)]
pub struct TraitDef {
    pub name: String,
    pub methods: Vec<TraitMethod>,
}

#[derive(Debug, Clone)]
pub struct TraitMethod {
    pub name: String,
    pub params: Vec<Param>,
    pub return_type: Option<String>,
}

#[derive(Debug, Clone)]
pub struct ImplDef {
    pub name: String,
    pub methods: Vec<Function>,
}
