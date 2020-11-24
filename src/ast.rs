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
