use crate::tokenizer::{Token, TokenType};
#[derive(PartialEq)]
#[derive(Debug)]
enum  NodeExpr {
    SIN(Box<NodeExprSin>),
    COS(Box<NodeExprCos>),
    LN(Box<NodeExprLn>),
    LOG(Box<NodeExprLog>),
    BIN(Box<NodeBinExpr>),
    INT(Box<NodeExprInt>),
    FLOAT(Box<NodeExprFloat>),

}
#[derive(PartialEq)]
#[derive(Debug)]
struct NodeExprSin {
    expr: NodeExpr,
}
#[derive(PartialEq)]
#[derive(Debug)]
struct NodeExprCos {
    expr: NodeExpr,
}
#[derive(PartialEq)]
#[derive(Debug)]
struct NodeExprLn {
    expr: NodeExpr,
}
#[derive(PartialEq)]
#[derive(Debug)]
struct NodeExprLog {
    expr: NodeExpr,
}
#[derive(PartialEq)]
#[derive(Debug)]
struct NodeExprInt {
    val: i32
}
#[derive(PartialEq)]
#[derive(Debug)]
struct NodeExprFloat {
    val: f32
}
#[derive(PartialEq)]
#[derive(Debug)]
enum Operation {
    ADD,
    SUB,
    MUL,
    DIV,
    EXP
}
#[derive(PartialEq)]
#[derive(Debug)]
struct NodeBinExpr {
    operation: Operation,
    lhs: NodeExpr,
    rhs: NodeExpr,
}

fn determine_precedence(token_type: &TokenType)->i32 {
    return match token_type{
        TokenType::ADD=>0,
        TokenType::SUB=>0,
        TokenType::MUL=>1,
        TokenType::DIV=>1,
        TokenType::EXP=>2,
        _=> -1,
    }
}

struct Parser {
    tokens: Vec<Token>,
    token_index: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) ->Parser {
        Parser{tokens,token_index:0}
    }

    fn parse(&mut self) -> Option<NodeExpr> {
        self.parse_expr()
    }
    fn parse_bin_expr(&mut self, mut lhs: NodeExpr, min_prec: i32) -> Option<NodeExpr> {

        while let Some((op, prec)) = self.next_op(min_prec) {
            self.consume(); // consume the current operator

            let mut rhs = self.parse_primary_expr()?;

            // Look ahead to see if the next operator has higher precedence
            while let Some((_, next_prec)) = self.next_op(prec + 1) {
                if next_prec > prec {
                    rhs = self.parse_bin_expr(rhs, next_prec)?;
                } else {
                    break;
                }
            }

            lhs = NodeExpr::BIN(Box::new(NodeBinExpr {
                operation: op,
                lhs,
                rhs,
            }));
        }
        Some(lhs)

    }
    fn parse_expr(&mut self) -> Option<NodeExpr> {
        if let Some(lhs) = self.parse_primary_expr() {
            self.parse_bin_expr(lhs, 0)
        } else {
            None
        }
    }
    fn parse_primary_expr(&mut self) -> Option<NodeExpr> {
        let token_type = self.peek()?.token_type.clone();;

        match token_type {
            TokenType::INT | TokenType::FLOAT => {
                let token = self.consume()?;
                match token.token_type {
                    TokenType::INT => Some(NodeExpr::INT(Box::from(NodeExprInt { val: token.value.clone().unwrap().parse().unwrap() }))),
                    TokenType::FLOAT => Some(NodeExpr::FLOAT(Box::from(NodeExprFloat { val: token.value.clone().unwrap().parse().unwrap() }))),
                    _ => None, // This case should not happen
                }
            },
            TokenType::OpenParen => {
                self.consume(); // Consume the '(' token
                let expr = self.parse_expr(); // Parse the expression inside the parentheses
                if self.peek().map_or(false, |t| t.token_type == TokenType::CloseParen) {
                    self.consume(); // Consume the ')' token
                } else {
                    panic!("Expected closing parenthesis");
                }
                expr
            },

            TokenType::SIN | TokenType::COS | TokenType::LN | TokenType::LOG => {
                self.consume(); // Consume the function token
                if self.peek().map_or(false, |t| t.token_type == TokenType::OpenParen) {
                    self.consume(); // Consume the '(' token
                    let inside_expr = self.parse_primary_expr().expect("No expression found inside parenthesis.");
                    if self.peek().map_or(false, |t| t.token_type == TokenType::CloseParen) {
                        self.consume(); // Consume the ')' token
                        match token_type {
                            TokenType::SIN => Some(NodeExpr::SIN(Box::from(NodeExprSin { expr: inside_expr }))),
                            TokenType::COS => Some(NodeExpr::COS(Box::from(NodeExprCos { expr: inside_expr }))),
                            TokenType::LN => Some(NodeExpr::LN(Box::from(NodeExprLn { expr: inside_expr }))),
                            TokenType::LOG => Some(NodeExpr::LOG(Box::from(NodeExprLog { expr: inside_expr }))),
                            _ => None, // This case should not happen
                        }
                    } else {
                        panic!("Expected closing parenthesis after function call");
                    }
                } else {
                    panic!("Expected '(' after function name");
                }
            },
            _ => panic!("Cannot parse: {:?}", token_type),
        }
    }


    fn next_op(&self, min_prec: i32) -> Option<(Operation, i32)> {
        if let Some(token) = self.peek() {
            let (op, prec) = match token.token_type {
                TokenType::ADD => (Operation::ADD, 0),
                TokenType::SUB => (Operation::SUB, 0),
                TokenType::MUL => (Operation::MUL, 1),
                TokenType::DIV => (Operation::DIV, 1),
                TokenType::EXP => (Operation::EXP, 2),
                _ => return None, // Not a binary operator
            };
            if prec >= min_prec {
                return Some((op, prec));
            }
        }
        None
    }
    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.token_index)
    }
    fn consume(&mut self) -> Option<&Token> {
        let res = self.tokens.get(self.token_index);
        self.token_index += 1;
        res
    }
}


#[cfg(test)]
mod tests {
    use crate::tokenizer::Tokenizer;
    use super::*;

    // Assuming you have a function to create a Parser from a string of tokens
    // You need to implement this according to your tokenization logic
    fn create_parser(input: &str) -> Parser {
        let mut tokenizer = Tokenizer::new(input);
        let tokens = tokenizer.tokenize();
        Parser::new(tokens)
    }

    #[test]
    fn test_parse_number() {
        let mut parser = create_parser("42");
        assert_eq!(parser.parse(), Some(NodeExpr::INT(Box::from(NodeExprInt { val: 42 }))));
    }

    #[test]
    fn test_parse_float() {
        let mut parser = create_parser("3.14");
        assert_eq!(parser.parse(), Some(NodeExpr::FLOAT(Box::from(NodeExprFloat { val: 3.14 }))));
    }

    #[test]
    fn test_simple_addition() {
        let mut parser = create_parser("1 + 2");
        let expected = Some(NodeExpr::BIN(Box::from(NodeBinExpr {
            operation: Operation::ADD,
            lhs: NodeExpr::INT(Box::from(NodeExprInt { val: 1 })),
            rhs: NodeExpr::INT(Box::from(NodeExprInt { val: 2 })),
        })));
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_precedence() {
        let mut parser = create_parser("1 + 2 * 3");
        let expected = Some(NodeExpr::BIN(Box::from(NodeBinExpr {
            operation: Operation::ADD,
            lhs: NodeExpr::INT(Box::from(NodeExprInt { val: 1 })),
            rhs: NodeExpr::BIN(Box::from(NodeBinExpr {
                operation: Operation::MUL,
                lhs: NodeExpr::INT(Box::from(NodeExprInt { val: 2 })),
                rhs: NodeExpr::INT(Box::from(NodeExprInt { val: 3 })),
            })),
        })));
        assert_eq!(parser.parse(), expected);
    }

    #[test]
    fn test_parentheses() {
        let mut parser = create_parser("(1 + 2) * 3");
        let expected = Some(NodeExpr::BIN(Box::new(NodeBinExpr {
            operation: Operation::MUL,
            lhs: NodeExpr::BIN(Box::new(NodeBinExpr {
                operation: Operation::ADD,
                lhs: NodeExpr::INT(Box::new(NodeExprInt { val: 1 })),
                rhs: NodeExpr::INT(Box::new(NodeExprInt { val: 2 })),
            })),
            rhs: NodeExpr::INT(Box::new(NodeExprInt { val: 3 })),
        })));
        assert_eq!(parser.parse(), expected);
    }


    #[test]
    #[should_panic(expected = "Expected closing parenthesis")]
    fn test_missing_closing_parenthesis() {
        let mut parser = create_parser("(1 + 2");
        parser.parse();
    }

    // More tests can be added for other scenarios...
}