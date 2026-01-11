use crate::ast::{Block, Expr, Function, IdentType, Stmt};

#[derive(Debug, Clone)]
pub struct Symbol {
    pub name: String,
    pub symbol_type: SymbolType,
    pub data_type: DataType,
    pub scope_level: usize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SymbolType {
    Variable,
}

#[derive(Debug, Clone, PartialEq)]
pub enum DataType {
    Int,
}

#[derive(Debug)]
pub struct SymbolTable {
    pub symbols: Vec<Symbol>,
    pub scopes: Vec<usize>,
    pub current_scope: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            symbols: Vec::new(),
            scopes: Vec::new(),
            current_scope: 0,
        }
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(self.symbols.len());
        self.current_scope += 1;
    }

    pub fn exit_scope(&mut self) {
        if self.scopes.pop().is_some() {
            self.current_scope -= 1;
        }
    }

    pub fn add_symbol(&mut self, name: String, symbol_type: SymbolType, data_type: DataType) -> Result<(), CompilationError> {
        for symbol in self.symbols.iter().rev() {
            if symbol.name == name && symbol.scope_level == self.current_scope {
                return Err(CompilationError::DuplicateDeclaration {
                    name,
                    location: None,
                });
            }
            if symbol.scope_level < self.current_scope {
                break;
            }
        }
        let symbol = Symbol {
            name: name.clone(),
            symbol_type,
            data_type,
            scope_level: self.current_scope,
        };
        self.symbols.push(symbol);
        Ok(())
    }

    pub fn lookup(&self, name: &str) -> Option<&Symbol> {
        for symbol in self.symbols.iter().rev() {
            if symbol.name == name && symbol.scope_level <= self.current_scope {
                return Some(symbol);
            }
        }
        None
    }
}

#[derive(Debug, Clone)]
pub struct Quadruple {
    pub op: String,
    pub arg1: String,
    pub arg2: String,
    pub result: String,
}

impl Quadruple {
    pub fn new(op: &str, arg1: &str, arg2: &str, result: &str) -> Self {
        Quadruple {
            op: op.to_string(),
            arg1: arg1.to_string(),
            arg2: arg2.to_string(),
            result: result.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Location {
    pub line: usize,
    pub column: usize,
}

impl Location {
    pub fn new(line: usize, column: usize) -> Self {
        Location { line, column }
    }
}

#[derive(Debug)]
pub enum CompilationError {
    UndeclaredVariable {
        name: String,
        location: Option<Location>,
    },
    DuplicateDeclaration {
        name: String,
        location: Option<Location>,
    },
    TypeMismatch {
        expected: String,
        found: String,
        location: Option<Location>,
    },
    GenericSemanticError {
        message: String,
        location: Option<Location>,
    },
}

impl std::fmt::Display for CompilationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            CompilationError::UndeclaredVariable { name, location } => {
                write!(f, "Undeclared variable '{}'", name)?;
                if let Some(loc) = location {
                    write!(f, " at line {}, column {}", loc.line, loc.column)?;
                }
                Ok(())
            }
            CompilationError::DuplicateDeclaration { name, location } => {
                write!(f, "Duplicate declaration of variable '{}'", name)?;
                if let Some(loc) = location {
                    write!(f, " at line {}, column {}", loc.line, loc.column)?;
                }
                Ok(())
            }
            CompilationError::TypeMismatch { expected, found, location } => {
                write!(f, "Type mismatch: expected {}, found {}", expected, found)?;
                if let Some(loc) = location {
                    write!(f, " at line {}, column {}", loc.line, loc.column)?;
                }
                Ok(())
            }
            CompilationError::GenericSemanticError { message, location } => {
                write!(f, "Semantic error: {}", message)?;
                if let Some(loc) = location {
                    write!(f, " at line {}, column {}", loc.line, loc.column)?;
                }
                Ok(())
            }
        }
    }
}

#[derive(Debug)]
pub struct CodeGenerator {
    pub symbol_table: SymbolTable,
    pub quadruples: Vec<Quadruple>,
    pub temp_counter: usize,
    pub errors: Vec<CompilationError>,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            symbol_table: SymbolTable::new(),
            quadruples: Vec::new(),
            temp_counter: 0,
            errors: Vec::new(),
        }
    }

    pub fn generate(&mut self, ast: &Function) -> Result<(), Vec<CompilationError>> {
        self.symbol_table = SymbolTable::new();
        self.quadruples.clear();
        self.temp_counter = 0;
        self.errors.clear();

        self.symbol_table.enter_scope();

        self.process_block(&ast.block)?;

        self.symbol_table.exit_scope();

        if !self.errors.is_empty() {
            return Err(std::mem::take(&mut self.errors));
        }
        Ok(())
    }

    fn process_block(&mut self, block: &Block) -> Result<(), Vec<CompilationError>> {
        self.symbol_table.enter_scope();

        for stmt in &block.stmts {
            self.process_stmt(stmt)?;
        }

        self.symbol_table.exit_scope();
        Ok(())
    }

    fn process_stmt(&mut self, stmt: &Stmt) -> Result<(), Vec<CompilationError>> {
        match stmt {
            Stmt::ReturnStmt(expr) => self.process_return_stmt(expr),
            Stmt::IfStmt { cond, if_block, else_stmt } => self.process_if_stmt(cond, if_block, else_stmt),
            Stmt::WhileStmt { cond, block } => self.process_while_stmt(cond, block),
            Stmt::AssignmentStmt { lval, rval } => self.process_assignment_stmt(lval, rval),
            Stmt::DeclareStmt { ident_type, ident, rval } => self.process_declare_stmt(ident_type, ident, rval),
        }
    }

    fn process_return_stmt(&mut self, expr: &Expr) -> Result<(), Vec<CompilationError>> {
        let result = self.process_expr(expr)?;
        self.emit("return", &result, "", "");
        Ok(())
    }

    fn process_if_stmt(&mut self, cond: &Expr, if_block: &Block, else_stmt: &Option<Block>) -> Result<(), Vec<CompilationError>> {
        // 处理条件表达式，生成条件跳转
        let (cond_op, lhs, rhs) = self.extract_condition(cond)?;
        
        // 生成条件跳转指令（占位符目标）
        let cond_jump_op = format!("j{}", cond_op);
        let cond_jump_index = self.quadruples.len();
        self.quadruples.push(Quadruple::new(&cond_jump_op, &lhs, &rhs, "0"));
        
        // 生成无条件跳转到 else 部分（占位符）
        let else_jump_index = self.quadruples.len();
        self.quadruples.push(Quadruple::new("j", "", "", "0"));
        
        // 处理 then 部分
        let then_start_index = self.quadruples.len() + 1;
        self.process_block(if_block)?;
        
        // 如果有 else 部分，生成跳转到 if 语句结束
        let after_else_jump_index = if else_stmt.is_some() {
            let jump_index = self.quadruples.len();
            self.quadruples.push(Quadruple::new("j", "", "", "0"));
            Some(jump_index)
        } else {
            None
        };
        
        // else 部分开始位置
        let else_start_index = self.quadruples.len() + 1;
        
        // 处理 else 部分（如果有）
        if let Some(else_blk) = else_stmt {
            self.process_block(else_blk)?;
        }
        
        // if 语句结束位置
        let after_if_index = self.quadruples.len() + 1;
        
        // 回填跳转目标
        // 条件跳转目标：then 部分开始
        self.quadruples[cond_jump_index].result = then_start_index.to_string();
        
        // 无条件跳转到 else 的目标：else 部分开始（如果没有 else 部分，跳转到 if 语句结束）
        let else_jump_target = if else_stmt.is_some() { else_start_index } else { after_if_index };
        self.quadruples[else_jump_index].result = else_jump_target.to_string();
        
        // 回填跳转到 if 语句结束的跳转（如果有）
        if let Some(jump_index) = after_else_jump_index {
            self.quadruples[jump_index].result = after_if_index.to_string();
        }
        
        Ok(())
    }

    fn process_while_stmt(&mut self, cond: &Expr, block: &Block) -> Result<(), Vec<CompilationError>> {
        // 循环开始位置
        let loop_start_index = self.quadruples.len() + 1;
        
        // 处理条件表达式
        let (cond_op, lhs, rhs) = self.extract_condition(cond)?;
        
        // 生成条件跳转到循环结束的指令（占位符）
        let cond_jump_op = format!("j{}", cond_op);
        let cond_jump_index = self.quadruples.len();
        self.quadruples.push(Quadruple::new(&cond_jump_op, &lhs, &rhs, "0"));
        
        // 处理循环体
        self.process_block(block)?;
        
        // 生成跳回循环开始的指令
        let back_jump_index = self.quadruples.len();
        self.quadruples.push(Quadruple::new("j", "", "", "0"));
        
        // 循环结束位置
        let loop_end_index = self.quadruples.len() + 1;
        
        // 回填跳转目标
        // 条件跳转：不满足条件时跳转到循环结束
        self.quadruples[cond_jump_index].result = loop_end_index.to_string();
        
        // 跳回循环开始
        self.quadruples[back_jump_index].result = loop_start_index.to_string();
        
        Ok(())
    }

    fn extract_condition(&mut self, expr: &Expr) -> Result<(String, String, String), Vec<CompilationError>> {
        match expr {
            Expr::BinaryExpr { op, lhs, rhs } => {
                let left = self.process_expr(lhs)?;
                let right = self.process_expr(rhs)?;
                Ok((op.to_string(), left, right))
            }
            _ => {
                // 对于非二元表达式，假设表达式值为条件（0为假，非0为真）
                // 生成临时变量存储表达式值
                let cond_value = self.process_expr(expr)?;
                Ok(("z".to_string(), cond_value, "0".to_string()))
            }
        }
    }

    fn process_assignment_stmt(&mut self, lval: &str, rval: &Expr) -> Result<(), Vec<CompilationError>> {
        if self.symbol_table.lookup(lval).is_none() {
            self.errors.push(CompilationError::UndeclaredVariable {
                name: lval.to_string(),
                location: None,
            });
            return Ok(());
        }

        let result = self.process_expr(rval)?;
        self.emit("=", &result, "", lval);
        Ok(())
    }

    fn process_declare_stmt(&mut self, ident_type: &IdentType, ident: &str, rval: &Option<Expr>) -> Result<(), Vec<CompilationError>> {
        let data_type = match ident_type {
            IdentType::Int => DataType::Int,
        };
        if let Err(e) = self.symbol_table.add_symbol(ident.to_string(), SymbolType::Variable, data_type) {
            self.errors.push(e);
            return Ok(());
        }

        if let Some(expr) = rval {
            let result = self.process_expr(expr)?;
            self.emit("=", &result, "", ident);
        }
        Ok(())
    }

    fn process_expr(&mut self, expr: &Expr) -> Result<String, Vec<CompilationError>> {
        match expr {
            Expr::Number(n) => Ok(n.to_string()),
            Expr::Var(name) => {
                if self.symbol_table.lookup(name).is_none() {
                    self.errors.push(CompilationError::UndeclaredVariable {
                        name: name.clone(),
                        location: None,
                    });
                    Ok("0".to_string())
                } else {
                    Ok(name.clone())
                }
            }
            Expr::BinaryExpr { op, lhs, rhs } => {
                let left = self.process_expr(lhs)?;
                let right = self.process_expr(rhs)?;
                let temp = self.new_temp();
                self.emit(&op.to_string(), &left, &right, &temp);
                Ok(temp)
            }
        }
    }

    fn emit(&mut self, op: &str, arg1: &str, arg2: &str, result: &str) {
        self.quadruples.push(Quadruple::new(op, arg1, arg2, result));
    }



    fn new_temp(&mut self) -> String {
        self.temp_counter += 1;
        format!("t{}", self.temp_counter)
    }



    pub fn print_quadruples(&self) {
        for (i, quad) in self.quadruples.iter().enumerate() {
            println!("{}: ({}, {}, {}, {})", i + 1, quad.op, quad.arg1, quad.arg2, quad.result);
        }
    }

    pub fn print_symbol_table(&self) {
        println!("Symbol Table:");
        for symbol in &self.symbol_table.symbols {
            println!("  {}: {:?} {:?} scope {}", 
                symbol.name, symbol.symbol_type, symbol.data_type, symbol.scope_level);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Function, FunctionType, Block, Stmt, Expr, IdentType};

    fn create_test_ast() -> Function {
        let block = Block {
            stmts: vec![
                Stmt::DeclareStmt {
                    ident_type: IdentType::Int,
                    ident: "x".to_string(),
                    rval: Some(Expr::Number(5)),
                },
                Stmt::AssignmentStmt {
                    lval: "x".to_string(),
                    rval: Expr::BinaryExpr {
                         op: "+".to_string(),
                        lhs: Box::new(Expr::Var("x".to_string())),
                        rhs: Box::new(Expr::Number(1)),
                    },
                },
                Stmt::ReturnStmt(Expr::Var("x".to_string())),
            ],
        };
        Function {
            return_type: FunctionType::Int,
            name: "main".to_string(),
            block,
        }
    }

    #[test]
    fn test_codegen_basic() {
        let ast = create_test_ast();
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate(&ast);
        assert!(result.is_ok(), "Code generation failed: {:?}", result.err());
        assert!(!codegen.quadruples.is_empty(), "No quadruples generated");
        codegen.print_quadruples();
        codegen.print_symbol_table();
    }

    #[test]
    fn test_undeclared_variable() {
        let block = Block {
            stmts: vec![
                Stmt::AssignmentStmt {
                    lval: "y".to_string(),
                    rval: Expr::Number(3),
                },
            ],
        };
        let ast = Function {
            return_type: FunctionType::Int,
            name: "main".to_string(),
            block,
        };
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate(&ast);
        assert!(result.is_err());
        let errors = result.err().unwrap();
        assert!(!errors.is_empty());
        match &errors[0] {
            CompilationError::UndeclaredVariable { name, .. } => {
                assert_eq!(name, "y");
            }
            _ => panic!("Expected UndeclaredVariable error"),
        }
    }

    #[test]
    fn test_duplicate_declaration() {
        let block = Block {
            stmts: vec![
                Stmt::DeclareStmt {
                    ident_type: IdentType::Int,
                    ident: "x".to_string(),
                    rval: None,
                },
                Stmt::DeclareStmt {
                    ident_type: IdentType::Int,
                    ident: "x".to_string(),
                    rval: None,
                },
            ],
        };
        let ast = Function {
            return_type: FunctionType::Int,
            name: "main".to_string(),
            block,
        };
        let mut codegen = CodeGenerator::new();
        let result = codegen.generate(&ast);
        assert!(result.is_err());
        let errors = result.err().unwrap();
        match &errors[0] {
            CompilationError::DuplicateDeclaration { name, .. } => {
                assert_eq!(name, "x");
            }
            _ => panic!("Expected DuplicateDeclaration error"),
        }
    }
}