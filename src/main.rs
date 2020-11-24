mod ast;
mod parse;
mod scan;

fn main() {
    //let expr = "f(x)!=3x^2-2x+1";
    //let expr = "(x^2+1)(x^2-2)";
    //let expr = "-3(x+2)";
    //let expr = "-3x-6";
    //let expr = "(x(3l)^3*(4^4)^(4^4))3";
    let expr = "sigma(i=0, 100, i^2)";
    println!(
        "{:?}",
        scan::Scanner::new(expr).scan_all().and_then(|ok| Ok(ok
            .iter()
            .map(|tk| tk.kind)
            .collect::<Vec<scan::TokenKind>>()))
    );

    let expr = parse::parse(&mut scan::Scanner::new(expr));
    if let Ok(expr) = expr {
        println!("{:#?}", expr);
        ast::print_expr(&expr);
        ast::print_infix(&expr);
    } else {
        println!("err: {:?}", expr);
    }
}

#[cfg(test)]
mod test {
    use super::{
        ast::Expr,
        parse::parse,
        scan::{Scanner, Token, TokenKind},
    };

    #[test]
    fn correct_exponent() {
        let expr1 = "a^2b";
        let expr2 = "a^2*b";

        let expr1 = parse(&mut Scanner::new(expr1)).unwrap();
        let expr2 = parse(&mut Scanner::new(expr2)).unwrap();

        super::ast::print_expr(&expr1);
        super::ast::print_expr(&expr2);

        assert_eq!(expr1, expr2);
    }

    #[test]
    fn correct_exponent2() {
        let expr1 = "a^b(c^d)";
        let expr2 = "a^b*(c^d)";

        let expr1 = parse(&mut Scanner::new(expr1)).unwrap();
        let expr2 = parse(&mut Scanner::new(expr2)).unwrap();

        super::ast::print_expr(&expr1);
        super::ast::print_expr(&expr2);

        assert_eq!(expr1, expr2);
    }

    #[test]
    fn parse_test() {
        let expr = "(x^2+1)(x^2-2)";
        let expr = parse(&mut Scanner::new(expr)).unwrap();
        println!("{:#?}", expr);
        assert_eq!(
            expr,
            Expr::Binary {
                lhs: Box::new(Expr::Binary {
                    lhs: Box::new(Expr::Binary {
                        lhs: Box::new(Expr::Variable {
                            name: Token::new(TokenKind::Variable, "x")
                        }),
                        op: Token::new(TokenKind::Power, "^"),
                        rhs: Box::new(Expr::Literal {
                            literal: Token::new(TokenKind::Integer(2), "2")
                        }),
                    }),
                    op: Token::new(TokenKind::Plus, "+"),
                    rhs: Box::new(Expr::Literal {
                        literal: Token::new(TokenKind::Integer(1), "1")
                    }),
                }),
                op: Token::new(TokenKind::Multiply, "*"),
                rhs: Box::new(Expr::Binary {
                    lhs: Box::new(Expr::Binary {
                        lhs: Box::new(Expr::Variable {
                            name: Token::new(TokenKind::Variable, "x")
                        }),
                        op: Token::new(TokenKind::Power, "^"),
                        rhs: Box::new(Expr::Literal {
                            literal: Token::new(TokenKind::Integer(2), "2")
                        }),
                    }),
                    op: Token::new(TokenKind::Minus, "-"),
                    rhs: Box::new(Expr::Literal {
                        literal: Token::new(TokenKind::Integer(2), "2")
                    }),
                }),
            }
        );
    }

    #[test]
    fn parse2() {
        let exprs = [
            "f(x)=3x^2-2x+1",
            "(x^2+1)(x^2-2)",
            "-3(x+2)",
            "-3x-6",
            "3(2)(3)",
            "(poopy(3l)^3(4^4)^(4^4))3",
        ];
        for expr in exprs.iter() {
            if let Err(_) = parse(&mut Scanner::new(expr)) {
                panic!("\"{}\" failed", expr);
            }
        }
    }

    #[test]
    fn scan() {
        let ts = Scanner::new("f(x)=3x^2-2x+1").scan_all().unwrap();
        let tks: Vec<TokenKind> = ts.iter().map(|t| t.kind).collect();
        assert_eq!(
            vec![
                TokenKind::Variable,
                TokenKind::LeftParen,
                TokenKind::Variable,
                TokenKind::RightParen,
                TokenKind::Equal,
                TokenKind::Integer(3),
                TokenKind::Variable,
                TokenKind::Power,
                TokenKind::Integer(2),
                TokenKind::Minus,
                TokenKind::Integer(2),
                TokenKind::Variable,
                TokenKind::Plus,
                TokenKind::Integer(1),
                TokenKind::End,
            ],
            tks
        );
    }

    #[test]
    fn scan2() {
        let ts = Scanner::new("f(x)=3.3x^2.2-2.2x+1.1").scan_all().unwrap();
        let tks: Vec<TokenKind> = ts.iter().map(|t| t.kind).collect();
        assert_eq!(
            vec![
                TokenKind::Variable,
                TokenKind::LeftParen,
                TokenKind::Variable,
                TokenKind::RightParen,
                TokenKind::Equal,
                TokenKind::Float(3.3),
                TokenKind::Variable,
                TokenKind::Power,
                TokenKind::Float(2.2),
                TokenKind::Minus,
                TokenKind::Float(2.2),
                TokenKind::Variable,
                TokenKind::Plus,
                TokenKind::Float(1.1),
                TokenKind::End,
            ],
            tks
        );
    }

    #[test]
    fn names() {
        Scanner::new("aa+bb").scan_all().unwrap();
        Scanner::new(" aa   + bb").scan_all().unwrap();
        Scanner::new(" aa   +   bb").scan_all().unwrap();
        Scanner::new(" aa    + bb").scan_all().unwrap();
        Scanner::new("   aa +   bb ").scan_all().unwrap();
        Scanner::new("   aa   +   bb ").scan_all().unwrap();
        Scanner::new("   aa   +    bb ").scan_all().unwrap();
        Scanner::new(" aa    +  bb").scan_all().unwrap();
        Scanner::new(" aa    +   bb").scan_all().unwrap();
        Scanner::new("  aa    +  bb").scan_all().unwrap();
        Scanner::new("  aa    +   bb").scan_all().unwrap();
        Scanner::new("  aa    +    bb").scan_all().unwrap();
        Scanner::new("   aa+bb").scan_all().unwrap();
        Scanner::new("   aa    +  bb").scan_all().unwrap();
        Scanner::new("   aa    +   bb").scan_all().unwrap();
        Scanner::new("   aa+bb ").scan_all().unwrap();
        Scanner::new("   aa+ bb ").scan_all().unwrap();
        Scanner::new("aa +   bb").scan_all().unwrap();
        Scanner::new("aa +    bb").scan_all().unwrap();
        Scanner::new("aa  +  bb").scan_all().unwrap();
        Scanner::new(" aa  +  bb").scan_all().unwrap();
        Scanner::new(" aa  +   bb").scan_all().unwrap();
        Scanner::new("   aa    + bb ").scan_all().unwrap();
        Scanner::new("   aa    +  bb ").scan_all().unwrap();
        Scanner::new("   aa    +   bb ").scan_all().unwrap();
        Scanner::new("   aa+  bb ").scan_all().unwrap();
        Scanner::new("   aa+   bb ").scan_all().unwrap();
        Scanner::new("aa+ bb").scan_all().unwrap();
        Scanner::new("aa+   bb").scan_all().unwrap();
        Scanner::new("aa+    bb").scan_all().unwrap();
        Scanner::new("aa + bb").scan_all().unwrap();
        Scanner::new("aa +  bb").scan_all().unwrap();
        Scanner::new(" aa  +    bb").scan_all().unwrap();
        Scanner::new("   aa+    bb ").scan_all().unwrap();
        Scanner::new("   aa + bb ").scan_all().unwrap();
        Scanner::new("   aa +  bb ").scan_all().unwrap();
        Scanner::new("   aa    +    bb ").scan_all().unwrap();
    }

    #[test]
    fn spaces() {
        Scanner::new("1+1").scan_all().unwrap();
        Scanner::new(" 1   + 1").scan_all().unwrap();
        Scanner::new(" 1   +   1").scan_all().unwrap();
        Scanner::new(" 1    + 1").scan_all().unwrap();
        Scanner::new("   1 +   1 ").scan_all().unwrap();
        Scanner::new("   1   +   1 ").scan_all().unwrap();
        Scanner::new("   1   +    1 ").scan_all().unwrap();
        Scanner::new(" 1    +  1").scan_all().unwrap();
        Scanner::new(" 1    +   1").scan_all().unwrap();
        Scanner::new("  1    +  1").scan_all().unwrap();
        Scanner::new("  1    +   1").scan_all().unwrap();
        Scanner::new("  1    +    1").scan_all().unwrap();
        Scanner::new("   1+1").scan_all().unwrap();
        Scanner::new("   1    +  1").scan_all().unwrap();
        Scanner::new("   1    +   1").scan_all().unwrap();
        Scanner::new("   1+1 ").scan_all().unwrap();
        Scanner::new("   1+ 1 ").scan_all().unwrap();
        Scanner::new("1 +   1").scan_all().unwrap();
        Scanner::new("1 +    1").scan_all().unwrap();
        Scanner::new("1  +  1").scan_all().unwrap();
        Scanner::new(" 1  +  1").scan_all().unwrap();
        Scanner::new(" 1  +   1").scan_all().unwrap();
        Scanner::new("   1    + 1 ").scan_all().unwrap();
        Scanner::new("   1    +  1 ").scan_all().unwrap();
        Scanner::new("   1    +   1 ").scan_all().unwrap();
        Scanner::new("   1+  1 ").scan_all().unwrap();
        Scanner::new("   1+   1 ").scan_all().unwrap();
        Scanner::new("1+ 1").scan_all().unwrap();
        Scanner::new("1+   1").scan_all().unwrap();
        Scanner::new("1+    1").scan_all().unwrap();
        Scanner::new("1 + 1").scan_all().unwrap();
        Scanner::new("1 +  1").scan_all().unwrap();
        Scanner::new(" 1  +    1").scan_all().unwrap();
        Scanner::new("   1+    1 ").scan_all().unwrap();
        Scanner::new("   1 + 1 ").scan_all().unwrap();
        Scanner::new("   1 +  1 ").scan_all().unwrap();
        Scanner::new("   1    +    1 ").scan_all().unwrap();
    }
}
