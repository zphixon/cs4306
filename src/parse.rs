use super::ast::*;
use super::scan::*;

pub fn parse<'a>(scanner: &mut Scanner<'a>) -> Result<Expr<'a>, &'static str> {
    parse_expr(scanner, 0)
}

pub fn parse_expr<'a>(scanner: &mut Scanner<'a>, min_bp: u8) -> Result<Expr<'a>, &'static str> {
    let lhs_token = scanner.next_token()?;

    let mut lhs = match lhs_token.kind {
        TokenKind::Variable => Expr::Variable { name: lhs_token },

        TokenKind::Builtin => {
            // consume left paren
            // collect args separated by commas
            // consume right paren
            panic!("idk how to do this yet xd");
        }

        TokenKind::Integer(_) | TokenKind::Float(_) => Expr::Literal { literal: lhs_token },

        TokenKind::LeftParen => {
            let lhs = parse(scanner)?;
            consume(scanner, TokenKind::RightParen)?;
            lhs
        }

        TokenKind::Minus => {
            let bp = prefix_bp(lhs_token.kind)?;
            let rhs = Box::new(parse_expr(scanner, bp)?);
            Expr::Unary { op: lhs_token, rhs }
        }

        _ => panic!("syntax err: {:?}", lhs_token),
    };

    loop {
        let op_token = scanner.peek_token(0)?;
        if op_token.kind == TokenKind::End {
            break;
        }

        if let Some(lhs_bp) = postfix_bp(op_token.kind) {
            if lhs_bp < min_bp {
                break;
            }

            lhs = Expr::Unary {
                op: scanner.next_token()?,
                rhs: Box::new(lhs),
            };

            continue;
        }

        if op_token.kind == TokenKind::Variable {
            if MULTIPLY_DIVIDE_MOD < min_bp {
                break;
            }

            lhs = Expr::Binary {
                lhs: Box::new(lhs),
                op: Token::new(TokenKind::Multiply, "*"),
                rhs: Box::new(parse_expr(scanner, MULTIPLY_DIVIDE_MOD + 1)?),
            };

            continue;
        }

        if let Some(lbp) = infix_bp(op_token.kind) {
            if lbp < min_bp {
                break;
            }

            if scanner.peek_token(0)?.kind != TokenKind::LeftParen {
                let op = scanner.next_token()?;

                lhs = Expr::Binary {
                    lhs: Box::new(lhs),
                    op,
                    rhs: Box::new(parse_expr(scanner, lbp + 1)?),
                };
            } else {
                consume(scanner, TokenKind::LeftParen)?;
                let rhs = parse_expr(scanner, 0)?;
                consume(scanner, TokenKind::RightParen)?;

                lhs = Expr::Binary {
                    lhs: Box::new(lhs),
                    op: Token::new(TokenKind::Multiply, "*"),
                    rhs: Box::new(rhs),
                };
            }

            continue;
        }

        break;
    }

    Ok(lhs)
}

const EQUAL_LESS_GREATER: u8 = 1;
const PLUS_MINUS: u8 = 2;
const MULTIPLY_DIVIDE_MOD: u8 = 3;
const POWER: u8 = 4;
const MINUS_PREFIX: u8 = 5;
const FACTORIAL: u8 = 6;

fn prefix_bp(kind: TokenKind) -> Result<u8, &'static str> {
    match kind {
        TokenKind::Minus => Ok(MINUS_PREFIX),
        _ => Err("syntax err: prefix op"),
    }
}

fn postfix_bp(kind: TokenKind) -> Option<u8> {
    match kind {
        TokenKind::Factorial => Some(FACTORIAL),
        _ => None,
    }
}

fn infix_bp(kind: TokenKind) -> Option<u8> {
    match kind {
        TokenKind::Equal => Some(EQUAL_LESS_GREATER),
        TokenKind::Plus | TokenKind::Minus => Some(PLUS_MINUS),
        TokenKind::Multiply | TokenKind::Divide | TokenKind::Modulo | TokenKind::LeftParen => {
            Some(MULTIPLY_DIVIDE_MOD)
        }
        TokenKind::Power => Some(POWER),
        _ => None,
    }
}

fn consume(scanner: &mut Scanner<'_>, kind: TokenKind) -> Result<(), &'static str> {
    let t = scanner.next_token()?;
    if t.kind == kind {
        Ok(())
    } else {
        println!("wanted {:?} got {:?}", kind, t.kind);
        Err("syntax error")
    }
}
