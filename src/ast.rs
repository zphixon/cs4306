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
    fn is_binary(&self) -> bool {
        match self {
            Expr::Binary { .. } => true,
            _ => false,
        }
    }
}

pub fn print_infix(expr: &Expr<'_>) {
    match expr {
        Expr::Literal { literal } => print!("{}", literal.lexeme),
        Expr::Variable { name } => print!("{}", name.lexeme),
        Expr::SpecialVariable { name } => print!("{}", name.lexeme),
        Expr::Unary { op, rhs } => {
            print!(" {}", op.lexeme);

            if rhs.is_binary() {
                print!("(");
            }
            print_infix(rhs.as_ref());
            if rhs.is_binary() {
                print!(")");
            }
        }
        Expr::Binary { lhs, op, rhs } => {
            if lhs.is_binary() {
                print!("(");
            }
            print_infix(lhs);
            if lhs.is_binary() {
                print!(")");
            }

            print!(" {} ", op.lexeme);

            if rhs.is_binary() {
                print!("(");
            }
            print_infix(rhs);
            if rhs.is_binary() {
                print!(")");
            }
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
}

pub fn print_prefix(expr: &Expr<'_>) {
    match expr {
        Expr::Literal { literal } => print!("{} ", literal.lexeme),
        Expr::Variable { name } => print!("{} ", name.lexeme),
        Expr::SpecialVariable { name } => print!("{} ", name.lexeme),
        Expr::Unary { op, rhs } => {
            print!("{}", op.lexeme);
            print_prefix(rhs.as_ref());
        }
        Expr::Binary { lhs, op, rhs } => {
            print!("{} ", op.lexeme);

            if lhs.is_binary() {
                print!("(");
            }
            print_prefix(lhs);
            if lhs.is_binary() {
                print!(") ");
            }

            if rhs.is_binary() {
                print!("(");
            }
            print_prefix(rhs);
            if rhs.is_binary() {
                print!(")");
            }
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
}

pub fn print_postfix(expr: &Expr<'_>) {
    match expr {
        Expr::Literal { literal } => print!("{} ", literal.lexeme),
        Expr::Variable { name } => print!("{} ", name.lexeme),
        Expr::SpecialVariable { name } => print!("{} ", name.lexeme),
        Expr::Unary { op, rhs } => {
            print!("{}", op.lexeme);

            if rhs.is_binary() {
                print!("(");
            }
            print_postfix(rhs.as_ref());
            if rhs.is_binary() {
                print!(")");
            }
        }
        Expr::Binary { lhs, op, rhs } => {
            if lhs.is_binary() {
                print!("(");
            }
            print_postfix(lhs);
            if lhs.is_binary() {
                print!(") ");
            }

            if rhs.is_binary() {
                print!("(");
            }
            print_postfix(rhs);
            if rhs.is_binary() {
                print!(") ");
            }

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
