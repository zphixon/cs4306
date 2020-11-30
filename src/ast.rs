use super::scan::Token;

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    Literal {
        literal: Token<'a>,
    },

    Variable {
        name: Token<'a>,
    },

    SpecialVariable {
        name: Token<'a>,
    },

    Unary {
        op: Token<'a>,
        rhs: Box<Expr<'a>>,
    },

    Binary {
        lhs: Box<Expr<'a>>,
        op: Token<'a>,
        rhs: Box<Expr<'a>>,
    },

    Call {
        name: Token<'a>,
        args: Vec<Expr<'a>>,
    },
}

impl Expr<'_> {
    fn deep(&self) -> bool {
        match self {
            Expr::Unary { rhs, .. } => match rhs.as_ref() {
                Expr::Literal { .. } | Expr::Variable { .. } | Expr::SpecialVariable { .. } => {
                    false
                }
                _ => true,
            },

            Expr::Binary { rhs, lhs, .. } => {
                (match rhs.as_ref() {
                    Expr::Literal { .. } | Expr::Variable { .. } | Expr::SpecialVariable { .. } => {
                        false
                    }
                    _ => true,
                }) || (match lhs.as_ref() {
                    Expr::Literal { .. } | Expr::Variable { .. } | Expr::SpecialVariable { .. } => {
                        false
                    }
                    _ => true,
                })
            }

            _ => false,
        }
    }
}

pub fn print_infix(expr: &Expr<'_>) {
    if expr.deep() {
        print!("(");
    }

    match expr {
        Expr::Literal { literal } => print!("{}", literal.lexeme),
        Expr::Variable { name } => print!("{}", name.lexeme),
        Expr::SpecialVariable { name } => print!("{}", name.lexeme),
        Expr::Unary { op, rhs } => {
            print!(" {}", op.lexeme);
            print_infix(rhs.as_ref());
        }
        Expr::Binary { lhs, op, rhs } => {
            print_infix(lhs);
            print!(" {} ", op.lexeme);
            print_infix(rhs);
        }
        Expr::Call { name, args } => {
            print!("{}(", name.lexeme);
            for expr in args.iter() {
                print_infix(expr);
                print!(",");
            }
            print!(")");
        }
    }

    if expr.deep() {
        print!(")");
    }
}

pub fn print_prefix(expr: &Expr<'_>) {
    if expr.deep() {
        print!("( ");
    }

    match expr {
        Expr::Literal { literal } => print!("{} ", literal.lexeme),
        Expr::Variable { name } => print!("{} ", name.lexeme),
        Expr::SpecialVariable { name } => print!("{} ", name.lexeme),
        Expr::Unary { op, rhs } => {
            print!("{} ", op.lexeme);
            print_prefix(rhs.as_ref());
        }
        Expr::Binary { lhs, op, rhs } => {
            print!("{} ", op.lexeme);
            print_prefix(lhs);
            print_prefix(rhs);
        }
        Expr::Call { name, args } => {
            print!("{}(", name.lexeme);
            for expr in args.iter() {
                print_prefix(expr);
                print!(", ");
            }
            print!(") ");
        }
    }

    if expr.deep() {
        print!(") ");
    }
}

pub fn print_postfix(expr: &Expr<'_>) {
    if expr.deep() {
        print!("( ");
    }

    match expr {
        Expr::Literal { literal } => print!("{} ", literal.lexeme),
        Expr::Variable { name } => print!("{} ", name.lexeme),
        Expr::SpecialVariable { name } => print!("{} ", name.lexeme),
        Expr::Unary { op, rhs } => {
            print!("{} ", op.lexeme);
            print_postfix(rhs.as_ref());
        }
        Expr::Binary { lhs, op, rhs } => {
            print_postfix(lhs);
            print_postfix(rhs);
            print!("{} ", op.lexeme);
        }
        Expr::Call { name, args } => {
            print!("{}(", name.lexeme);
            for expr in args.iter() {
                print_postfix(expr);
                print!(", ");
            }
            print!(") ");
        }
    }

    if expr.deep() {
        print!(") ");
    }
}

#[allow(dead_code)]
pub fn print_expr(expr: &Expr<'_>) {
    print_expr_(expr, 0);
}

fn print_space(level: usize) {
    let mut x = String::with_capacity(level);
    for _ in 0..level * 3 {
        x.push(' ');
    }
    print!("{}", x);
}

fn print_expr_(expr: &Expr<'_>, level: usize) {
    print_space(level);
    match expr {
        Expr::Literal { literal } => println!("{}", literal.lexeme),
        Expr::Variable { name } => println!("{}", name.lexeme),
        Expr::SpecialVariable { name } => println!("{}", name.lexeme),
        Expr::Unary { op, rhs } => {
            println!("unary {}", op.lexeme);
            print_expr_(rhs.as_ref(), level + 1);
        }
        Expr::Binary { lhs, op, rhs } => {
            println!("binary {}", op.lexeme);
            print_expr_(lhs, level + 1);
            print_expr_(rhs, level + 1);
        }
        Expr::Call { name, args } => {
            println!("builtin {}", name.lexeme);
            for expr in args.iter() {
                print_expr_(expr, level + 1);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::scan::*;
    #[test]
    fn deep() {
        assert!(!Expr::Binary {
            rhs: Box::new(Expr::Literal {
                literal: Token::new(TokenKind::Variable, "something")
            }),
            op: Token::new(TokenKind::Multiply, "*"),
            lhs: Box::new(Expr::Literal {
                literal: Token::new(TokenKind::Integer(3), "3")
            }),
        }
        .deep());

        assert!(Expr::Binary {
            rhs: Box::new(Expr::Binary {
                rhs: Box::new(Expr::Literal {
                    literal: Token::new(TokenKind::Variable, "something")
                }),
                op: Token::new(TokenKind::Multiply, "*"),
                lhs: Box::new(Expr::Literal {
                    literal: Token::new(TokenKind::Integer(3), "3")
                })
            }),
            op: Token::new(TokenKind::Multiply, "*"),
            lhs: Box::new(Expr::Literal {
                literal: Token::new(TokenKind::Integer(3), "3")
            }),
        }
        .deep());
    }
}
