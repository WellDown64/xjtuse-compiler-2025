use crate::token::Token;
use crate::ast::{self, FunctionType};

pub mod error;

pub struct Parser<'a> {
    tokens: &'a [Token],
    pos: usize,
}

impl<'a> Parser<'a> {
    pub fn new(tokens: &'a [Token]) -> Self {
        Parser {
            tokens,
            pos: 0,
        }
    }

    fn curr_token(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn advance(&mut self) {
        self.pos += 1;
    }

    fn consume_token(&mut self, expected: Token) -> Result<(), &'static str> {
        match self.curr_token() {
            Some(token) if token == &expected => {
                self.advance();
                Ok(())
            }
            _ => Err("Unexpected token"),
        }
    }

    pub fn parse(&mut self) -> Result<ast::Function, &'static str> {
        self.parse_function()
    }

    fn parse_function(&mut self) -> Result<ast::Function, &'static str>{
        // Parse return type
        let return_type = match self.curr_token() {
            Some(Token::Keywords(crate::token::Keyword::Int)) => {
                self.advance();
                FunctionType::Int
            }
            _ => return Err("Expected return type (int)"),
        };

        // Parse function name
        let function_name = match self.curr_token() {
            Some(Token::Ident(s)) => {
                let name = s.clone();
                self.advance();
                name
            }
            _ => return Err("Expected function name"),
        };

        // Parse '('
        self.consume_token(Token::LParam)?;

        // Parse ')'
        self.consume_token(Token::RParam)?;

        // Parse block
        let block = self.parse_block()?;

        Ok(ast::Function::new(
            return_type,
            function_name,
            block
        ))
    }

    fn parse_block(&mut self) -> Result<ast::Block, &'static str> {
        self.consume_token(Token::LBrace)?;

        let mut stmts = Vec::new();
        while self.curr_token() != Some(&Token::RBrace) {
            let stmt = self.parse_stmt()?;
            stmts.push(stmt);
        }

        self.consume_token(Token::RBrace)?;
        return Ok(ast::Block::new(stmts));
    }
 
    fn parse_stmt(&mut self) -> Result<ast::Stmt, &'static str> {
        match self.curr_token() {
            Some(Token::Keywords(crate::token::Keyword::Return)) => self.parse_return_stmt(),
            Some(Token::Keywords(crate::token::Keyword::If)) => self.parse_if_stmt(),
            Some(Token::Keywords(crate::token::Keyword::While)) => self.parse_while_stmt(),
            Some(Token::Keywords(crate::token::Keyword::Int)) => self.parse_declare_stmt(),
            Some(Token::Ident(_)) => self.parse_assignment_stmt(),
            _ => Err("Invalid statement"),
        }
    }
// 
    fn parse_return_stmt(&mut self) -> Result<ast::Stmt, &'static str> {
        self.advance(); // consume 'return'
        let expr = self.parse_expr()?;
        self.consume_token(Token::Semicolon)?;
        Ok(ast::Stmt::ReturnStmt(expr))
    }
// 
    fn parse_expr(&mut self) -> Result<ast::Expr, &'static str> {
        self.parse_relation_expr()
    }
// 
    fn parse_relation_expr(&mut self) -> Result<ast::Expr, &'static str> {
        let mut expr = self.parse_additive_expr()?;
        
        loop {
            match self.curr_token() {
                Some(Token::Gt) => {
                    self.advance(); // consume '>'
                    let rhs = self.parse_additive_expr()?;
                    expr = ast::Expr::BinaryExpr {
                        op: '>',
                        lhs: Box::new(expr),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_additive_expr(&mut self) -> Result<ast::Expr, &'static str> {
        let mut expr = self.parse_multiplicative_expr()?;
        
        loop {
            match self.curr_token() {
                Some(Token::Plus) => {
                    self.advance(); // consume '+'
                    let rhs = self.parse_multiplicative_expr()?;
                    expr = ast::Expr::BinaryExpr {
                        op: '+',
                        lhs: Box::new(expr),
                        rhs: Box::new(rhs),
                    };
                }
                Some(Token::Minus) => {
                    self.advance(); // consume '-'
                    let rhs = self.parse_multiplicative_expr()?;
                    expr = ast::Expr::BinaryExpr {
                        op: '-',
                        lhs: Box::new(expr),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }
// 
    fn parse_multiplicative_expr(&mut self) -> Result<ast::Expr, &'static str> {
        let mut expr = self.parse_primary_expr()?;
        
        loop {
            match self.curr_token() {
                Some(Token::Times) => {
                    self.advance(); // consume '*'
                    let rhs = self.parse_primary_expr()?;
                    expr = ast::Expr::BinaryExpr {
                        op: '*',
                        lhs: Box::new(expr),
                        rhs: Box::new(rhs),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }
// 
    fn parse_primary_expr(&mut self) -> Result<ast::Expr, &'static str> {
        match self.curr_token() {
            Some(Token::Int(s)) => {
                let num = s.parse::<i32>().map_err(|_| "Invalid integer")?;
                self.advance();
                Ok(ast::Expr::Number(num))
            }
            Some(Token::Ident(s)) => {
                let var_name = s.clone();
                self.advance();
                Ok(ast::Expr::Var(var_name))
            }
            Some(Token::LParam) => {
                self.advance(); // consume '('
                let expr = self.parse_expr()?;
                self.consume_token(Token::RParam)?;
                Ok(expr)
            }
            _ => Err("Expected expression"),
        }
    }
// 
    fn parse_declare_stmt(&mut self) -> Result<ast::Stmt, &'static str> {
        self.advance(); // consume 'int'
        let ident = match self.curr_token() {
            Some(Token::Ident(s)) => s.clone(),
            _ => return Err("Expected identifier after int"),
        };
        self.advance(); // consume identifier
        
        let rval = if self.curr_token().is_some_and(|t| matches!(t, Token::Assign)) {
            self.advance(); // consume '='
            let expr = self.parse_expr()?;
            Some(expr)
        } else {
            None
        };
        
        self.consume_token(Token::Semicolon)?;
        Ok(ast::Stmt::DeclareStmt {
            ident_type: ast::IdentType::Int,
            ident,
            rval,
        })
    }
// 
    fn parse_assignment_stmt(&mut self) -> Result<ast::Stmt, &'static str> {
        let lval = match self.curr_token() {
            Some(Token::Ident(s)) => s.clone(),
            _ => return Err("Expected identifier in assignment"),
        };
        self.advance(); // consume identifier
        
        self.consume_token(Token::Assign)?;
        
        let rval = self.parse_expr()?;
        
        self.consume_token(Token::Semicolon)?;
        Ok(ast::Stmt::AssignmentStmt {
            lval,
            rval,
        })
    }
// 
    fn parse_if_stmt(&mut self) -> Result<ast::Stmt, &'static str> {
        self.advance(); // consume 'if'
        
        self.consume_token(Token::LParam)?;
        
        let cond = self.parse_expr()?;
        
        self.consume_token(Token::RParam)?;
        
        let if_block = self.parse_block()?;
        
        let else_stmt = if self.curr_token().is_some_and(|t| matches!(t, Token::Keywords(crate::token::Keyword::Else))) {
            self.advance(); // consume 'else'
            Some(self.parse_block()?)
        } else {
            None
        };
        
        Ok(ast::Stmt::IfStmt {
            cond,
            if_block,
            else_stmt,
        })
    }
// 
    fn parse_while_stmt(&mut self) -> Result<ast::Stmt, &'static str> {
        self.advance(); // consume 'while'
        
        self.consume_token(Token::LParam)?;
        
        let cond = self.parse_expr()?;
        
        self.consume_token(Token::RParam)?;
        
        let block = self.parse_block()?;
        
        Ok(ast::Stmt::WhileStmt {
            cond,
            block,
        })
    }

}
