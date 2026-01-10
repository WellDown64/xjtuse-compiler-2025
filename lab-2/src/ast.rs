

#[derive(Debug)]
pub struct Function {
    return_type: FunctionType,
    name: String,
    block: Block
}

impl Function {
    pub fn new(
        return_type: FunctionType,
        name: String,
        block: Block
    ) -> Self {
        return Function {
            return_type,
            name,
            block
        }
    }
}

#[derive(Debug)]
pub enum FunctionType {
    Int,
}

#[derive(Debug)]
pub struct Block {
    stmts: Vec<Stmt>
}

impl Block {
    pub fn new(stmts: Vec<Stmt>) -> Self {
        Block {
            stmts: stmts
        }
    }
}

#[derive(Debug)]
pub enum Stmt {
    ReturnStmt(Expr),
    IfStmt {
        cond: Expr,
        if_block: Block,
        else_stmt: Option<Block>
    },
    WhileStmt {
        cond: Expr,
        block: Block
    },
    AssignmentStmt {
        lval: String,
        rval: Expr
    },
    DeclareStmt {
        ident_type: IdentType,
        ident: String,
        rval: Option<Expr>
    }
}

#[derive(Debug)]
pub enum IdentType {
    Int,
}

#[derive(Debug)]
pub enum Expr {
    Number(i32),
    Var(String),
    BinaryExpr {
        op: char,
        lhs: Box<Expr>,
        rhs: Box<Expr>
    }
}
