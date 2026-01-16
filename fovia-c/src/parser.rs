use crate::ast::{
    BinOp, Block, Expr, Function, ImplDef, LoopExpr, MatchArm, MatchExpr, MatchPattern, Param,
    Program, Stmt, StructDef, StructField, TraitDef, TraitMethod,
};
use crate::lexer::{Lexer, Token};

pub struct Parser {
    lexer: Lexer,
    lookahead: Token,
}

impl Parser {
    pub fn new(input: &str) -> Self {
        let mut lexer = Lexer::new(input);
        let lookahead = lexer.next_token().unwrap_or(Token::Eof);
        Self { lexer, lookahead }
    }

    pub fn parse_program(&mut self) -> Result<Program, String> {
        let mut functions = Vec::new();
        let mut structs = Vec::new();
        let mut traits = Vec::new();
        let mut impls = Vec::new();
        while self.lookahead != Token::Eof {
            match self.lookahead {
                Token::Struct => structs.push(self.parse_struct_def()?),
                Token::Trait => traits.push(self.parse_trait_def()?),
                Token::Impl => impls.push(self.parse_impl_def()?),
                Token::Fn => functions.push(self.parse_function()?),
                _ => return Err("unexpected token at top-level".to_string()),
            }
        }
        Ok(Program {
            functions,
            structs,
            traits,
            impls,
        })
    }

    fn parse_function(&mut self) -> Result<Function, String> {
        self.expect(Token::Fn)?;
        let name = self.expect_ident()?;
        self.expect(Token::LParen)?;
        let params = self.parse_params()?;
        self.expect(Token::RParen)?;

        let return_type = if self.lookahead == Token::Arrow {
            self.advance()?;
            Some(self.parse_type_until_lbrace()?)
        } else {
            None
        };

        self.expect(Token::LBrace)?;
        let mut body = Vec::new();
        while self.lookahead != Token::RBrace {
            body.push(self.parse_stmt()?);
        }
        self.expect(Token::RBrace)?;

        Ok(Function {
            name,
            params,
            body,
            return_type,
        })
    }

    fn parse_params(&mut self) -> Result<Vec<Param>, String> {
        let mut params = Vec::new();
        if self.lookahead == Token::RParen {
            return Ok(params);
        }

        loop {
            let name = self.expect_ident()?;
            self.expect(Token::Colon)?;
            let ty = self.parse_type_until_delim()?;
            params.push(Param { name, ty });
            if self.lookahead == Token::Comma {
                self.advance()?;
                continue;
            }
            break;
        }
        Ok(params)
    }

    fn parse_type_until_delim(&mut self) -> Result<String, String> {
        let mut parts = Vec::new();
        loop {
            match &self.lookahead {
                Token::Ident(ident) => {
                    parts.push(ident.clone());
                    self.advance()?;
                }
                Token::Comma | Token::RParen => break,
                _ => return Err("unexpected token in type".to_string()),
            }
        }
        Ok(parts.join(" "))
    }

    fn parse_type_until_lbrace(&mut self) -> Result<String, String> {
        let mut parts = Vec::new();
        loop {
            match &self.lookahead {
                Token::Ident(ident) => {
                    parts.push(ident.clone());
                    self.advance()?;
                }
                Token::LBrace => break,
                _ => return Err("unexpected token in return type".to_string()),
            }
        }
        Ok(parts.join(" "))
    }

    fn parse_stmt(&mut self) -> Result<Stmt, String> {
        match self.lookahead.clone() {
            Token::Let => self.parse_let(),
            Token::Brk => self.parse_break(),
            Token::Bang => self.parse_reactor_stdout(),
            _ => {
                if let Token::Ident(_) = self.lookahead {
                    match self.peek_token()? {
                        Token::Assign => {
                            let name = self.expect_ident()?;
                            self.expect(Token::Assign)?;
                            let expr = self.parse_expr()?;
                            self.consume_stmt_terminator()?;
                            return Ok(Stmt::Assign { name, expr });
                        }
                        Token::LParen | Token::ColonColon => {
                            let name = self.parse_call_name()?;
                            self.expect(Token::LParen)?;
                            let args = self.parse_args()?;
                            self.expect(Token::RParen)?;
                            self.consume_stmt_terminator()?;
                            return Ok(Stmt::Call { name, args });
                        }
                        _ => {}
                    }
                }

                let expr = self.parse_expr()?;
                if self.lookahead == Token::Dot {
                    self.advance()?;
                }
                Ok(Stmt::ExprStmt(expr))
            }
        }
    }

    fn parse_let(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Let)?;
        let name = self.expect_ident()?;
        self.expect(Token::Colon)?;
        let ty = self.parse_type_until_assign()?;
        self.expect(Token::Assign)?;
        let expr = self.parse_expr()?;
        self.consume_stmt_terminator()?;
        Ok(Stmt::Let { name, ty, expr })
    }

    fn parse_break(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Brk)?;
        let label = if self.lookahead == Token::Tick {
            self.advance()?;
            Some(self.expect_ident()?)
        } else {
            None
        };
        let expr = self.parse_expr()?;
        self.consume_stmt_terminator()?;
        Ok(Stmt::Break { label, expr })
    }

    fn parse_reactor_stdout(&mut self) -> Result<Stmt, String> {
        self.expect(Token::Bang)?;
        let reactor = self.expect_ident()?;
        if reactor != "reactor" {
            return Err("expected reactor".to_string());
        }
        self.expect(Token::Dot)?;
        let method = self.expect_ident()?;
        if method != "stdout" {
            return Err("expected stdout".to_string());
        }
        self.expect(Token::LParen)?;
        let mut args = self.parse_args()?;
        self.expect(Token::RParen)?;
        self.consume_stmt_terminator()?;

        if args.len() != 1 {
            return Err("stdout expects exactly one argument".to_string());
        }
        Ok(Stmt::ReactorStdout { arg: args.remove(0) })
    }

    fn parse_args(&mut self) -> Result<Vec<Expr>, String> {
        let mut args = Vec::new();
        if self.lookahead == Token::RParen {
            return Ok(args);
        }
        loop {
            args.push(self.parse_expr()?);
            if self.lookahead == Token::Comma {
                self.advance()?;
                continue;
            }
            break;
        }
        Ok(args)
    }

    fn parse_expr(&mut self) -> Result<Expr, String> {
        self.parse_equality()
    }

    fn parse_equality(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_comparison()?;
        loop {
            let op = match self.lookahead {
                Token::EqEq => BinOp::Eq,
                Token::NotEq => BinOp::Ne,
                _ => break,
            };
            self.advance()?;
            let right = self.parse_comparison()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_comparison(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_term()?;
        loop {
            let op = match self.lookahead {
                Token::Lt => BinOp::Lt,
                Token::Le => BinOp::Le,
                Token::Gt => BinOp::Gt,
                Token::Ge => BinOp::Ge,
                _ => break,
            };
            self.advance()?;
            let right = self.parse_term()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_term(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_factor()?;
        loop {
            let op = match self.lookahead {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance()?;
            let right = self.parse_factor()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_factor(&mut self) -> Result<Expr, String> {
        let mut expr = self.parse_primary()?;
        loop {
            let op = match self.lookahead {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance()?;
            let right = self.parse_primary()?;
            expr = Expr::Binary {
                left: Box::new(expr),
                op,
                right: Box::new(right),
            };
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, String> {
        match &self.lookahead {
            Token::Ident(name) => {
                let name = name.clone();
                if matches!(self.peek_token()?, Token::LParen | Token::ColonColon) {
                    let name = self.parse_call_name()?;
                    self.expect(Token::LParen)?;
                    let args = self.parse_args()?;
                    self.expect(Token::RParen)?;
                    Ok(Expr::Call { name, args })
                } else {
                    self.advance()?;
                    Ok(Expr::Ident(name))
                }
            }
            Token::Str(bytes) => {
                let bytes = bytes.clone();
                self.advance()?;
                Ok(Expr::StrLiteral(bytes))
            }
            Token::Number(value) => {
                let value = *value;
                self.advance()?;
                Ok(Expr::IntLiteral(value))
            }
            Token::True => {
                self.advance()?;
                Ok(Expr::BoolLiteral(true))
            }
            Token::False => {
                self.advance()?;
                Ok(Expr::BoolLiteral(false))
            }
            Token::Loop => self.parse_loop_expr(),
            Token::Match => self.parse_match_expr(),
            Token::LBrace => {
                let block = self.parse_block()?;
                Ok(Expr::Block(block))
            }
            Token::LParen => {
                self.advance()?;
                let expr = self.parse_expr()?;
                self.expect(Token::RParen)?;
                Ok(expr)
            }
            _ => Err("unexpected token in expression".to_string()),
        }
    }

    fn parse_block(&mut self) -> Result<Block, String> {
        self.expect(Token::LBrace)?;
        let mut stmts = Vec::new();
        let mut tail = None;
        while self.lookahead != Token::RBrace {
            match self.lookahead {
                Token::Let | Token::Brk | Token::Bang | Token::Ident(_) => {
                    let stmt = self.parse_stmt()?;
                    stmts.push(stmt);
                }
                _ => {
                    let expr = self.parse_expr()?;
                    if self.lookahead == Token::Dot {
                        self.advance()?;
                        stmts.push(Stmt::ExprStmt(expr));
                    } else {
                        tail = Some(Box::new(expr));
                        break;
                    }
                }
            }
        }
        self.expect(Token::RBrace)?;
        Ok(Block { stmts, tail })
    }

    fn parse_loop_expr(&mut self) -> Result<Expr, String> {
        self.expect(Token::Loop)?;
        let label = if self.lookahead == Token::Tick {
            self.advance()?;
            Some(self.expect_ident()?)
        } else {
            None
        };
        let body = self.parse_block()?;
        Ok(Expr::Loop(LoopExpr { label, body }))
    }

    fn parse_match_expr(&mut self) -> Result<Expr, String> {
        self.expect(Token::Match)?;
        let scrutinee = self.parse_expr()?;
        self.expect(Token::LBrace)?;
        let mut arms = Vec::new();
        while self.lookahead != Token::RBrace {
            let pattern = match self.lookahead.clone() {
                Token::Underscore => {
                    self.advance()?;
                    MatchPattern::Wildcard
                }
                Token::Lt => {
                    self.advance()?;
                    let value = match self.lookahead.clone() {
                        Token::Number(n) => n,
                        _ => return Err("expected number after '<'".to_string()),
                    };
                    self.advance()?;
                    MatchPattern::LessThan(value)
                }
                Token::Le => {
                    self.advance()?;
                    let value = match self.lookahead.clone() {
                        Token::Number(n) => n,
                        _ => return Err("expected number after '<='".to_string()),
                    };
                    self.advance()?;
                    MatchPattern::LessEqual(value)
                }
                Token::EqEq => {
                    self.advance()?;
                    let value = match self.lookahead.clone() {
                        Token::Number(n) => n,
                        _ => return Err("expected number after '=='".to_string()),
                    };
                    self.advance()?;
                    MatchPattern::Equal(value)
                }
                Token::NotEq => {
                    self.advance()?;
                    let value = match self.lookahead.clone() {
                        Token::Number(n) => n,
                        _ => return Err("expected number after '!='".to_string()),
                    };
                    self.advance()?;
                    MatchPattern::NotEqual(value)
                }
                Token::Gt => {
                    self.advance()?;
                    let value = match self.lookahead.clone() {
                        Token::Number(n) => n,
                        _ => return Err("expected number after '>'".to_string()),
                    };
                    self.advance()?;
                    MatchPattern::GreaterThan(value)
                }
                Token::Ge => {
                    self.advance()?;
                    let value = match self.lookahead.clone() {
                        Token::Number(n) => n,
                        _ => return Err("expected number after '>='".to_string()),
                    };
                    self.advance()?;
                    MatchPattern::GreaterEqual(value)
                }
                _ => return Err("invalid match pattern".to_string()),
            };
            self.expect(Token::FatArrow)?;
            let body = if self.lookahead == Token::LBrace {
                self.parse_block()?
            } else {
                let expr = self.parse_expr()?;
                Block {
                    stmts: Vec::new(),
                    tail: Some(Box::new(expr)),
                }
            };
            arms.push(MatchArm { pattern, body });
        }
        self.expect(Token::RBrace)?;
        Ok(Expr::Match(MatchExpr {
            scrutinee: Box::new(scrutinee),
            arms,
        }))
    }

    fn consume_stmt_terminator(&mut self) -> Result<(), String> {
        if self.lookahead == Token::Dot {
            self.advance()?;
        }
        Ok(())
    }

    fn parse_type_until_assign(&mut self) -> Result<String, String> {
        let mut parts = Vec::new();
        loop {
            match &self.lookahead {
                Token::Ident(ident) => {
                    parts.push(ident.clone());
                    self.advance()?;
                }
                Token::Assign | Token::EqEq | Token::FatArrow | Token::Dot | Token::LBrace => break,
                _ => break,
            }
        }
        Ok(parts.join(" "))
    }

    fn parse_struct_def(&mut self) -> Result<StructDef, String> {
        self.expect(Token::Struct)?;
        let name = self.expect_ident()?;
        self.expect(Token::LBrace)?;
        let mut fields = Vec::new();
        while self.lookahead != Token::RBrace {
            let field_name = self.expect_ident()?;
            self.expect(Token::Colon)?;
            let ty = self.parse_type_until_struct_field_end()?;
            fields.push(StructField { name: field_name, ty });
            if self.lookahead == Token::Comma {
                self.advance()?;
            }
        }
        self.expect(Token::RBrace)?;
        Ok(StructDef { name, fields })
    }

    fn parse_trait_def(&mut self) -> Result<TraitDef, String> {
        self.expect(Token::Trait)?;
        let name = self.expect_ident()?;
        self.expect(Token::LBrace)?;
        let mut methods = Vec::new();
        while self.lookahead != Token::RBrace {
            self.expect(Token::Fn)?;
            let method_name = self.expect_ident()?;
            self.expect(Token::LParen)?;
            let params = self.parse_params()?;
            self.expect(Token::RParen)?;
            let return_type = if self.lookahead == Token::Arrow {
                self.advance()?;
                Some(self.parse_type_until_trait_end()?)
            } else {
                None
            };
            if self.lookahead == Token::Dot {
                self.advance()?;
            }
            methods.push(TraitMethod {
                name: method_name,
                params,
                return_type,
            });
        }
        self.expect(Token::RBrace)?;
        Ok(TraitDef { name, methods })
    }

    fn parse_impl_def(&mut self) -> Result<ImplDef, String> {
        self.expect(Token::Impl)?;
        let name = self.expect_ident()?;
        self.expect(Token::LBrace)?;
        let mut methods = Vec::new();
        while self.lookahead != Token::RBrace {
            methods.push(self.parse_function()?);
        }
        self.expect(Token::RBrace)?;
        Ok(ImplDef { name, methods })
    }

    fn parse_type_until_struct_field_end(&mut self) -> Result<String, String> {
        let mut parts = Vec::new();
        loop {
            match &self.lookahead {
                Token::Ident(ident) => {
                    parts.push(ident.clone());
                    self.advance()?;
                }
                Token::Comma | Token::RBrace => break,
                _ => return Err("unexpected token in struct field type".to_string()),
            }
        }
        Ok(parts.join(" "))
    }

    fn parse_type_until_trait_end(&mut self) -> Result<String, String> {
        let mut parts = Vec::new();
        loop {
            match &self.lookahead {
                Token::Ident(ident) => {
                    parts.push(ident.clone());
                    self.advance()?;
                }
                Token::Dot | Token::RBrace => break,
                _ => return Err("unexpected token in trait return type".to_string()),
            }
        }
        Ok(parts.join(" "))
    }

    fn peek_token(&self) -> Result<Token, String> {
        let mut lexer = self.lexer.clone();
        lexer.next_token()
    }

    fn expect(&mut self, token: Token) -> Result<(), String> {
        if self.lookahead == token {
            self.advance()?;
            Ok(())
        } else {
            Err(format!("expected {token:?}, got {:?}", self.lookahead))
        }
    }

    fn expect_ident(&mut self) -> Result<String, String> {
        match &self.lookahead {
            Token::Ident(name) => {
                let name = name.clone();
                self.advance()?;
                Ok(name)
            }
            _ => Err("expected identifier".to_string()),
        }
    }

    fn parse_call_name(&mut self) -> Result<String, String> {
        let base = self.expect_ident()?;
        if self.lookahead == Token::ColonColon {
            self.advance()?;
            let method = self.expect_ident()?;
            Ok(format!("{base}__{method}"))
        } else {
            Ok(base)
        }
    }

    fn advance(&mut self) -> Result<(), String> {
        self.lookahead = self.lexer.next_token()?;
        Ok(())
    }
}
