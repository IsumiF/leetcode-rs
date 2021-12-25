/*
 * @lc app=leetcode id=227 lang=rust
 *
 * [227] Basic Calculator II
 */

struct Solution {}

// @lc code=start
use std::{iter::Peekable, str::Chars};

impl Solution {
    pub fn calculate(s: String) -> i32 {
        let tokenizer = Tokenizer::new(&s);
        let mut op_stack = Vec::<Op>::new();
        let mut number_stack = Vec::<i32>::new();
        for token in tokenizer {
            match token {
                Token::Number(num) => {
                    number_stack.push(num);
                }
                Token::Op(op) => {
                    /*
                        while (
                            there is an operator o2 other than the left parenthesis at the top
                            of the operator stack, and (o2 has greater precedence than o1
                            or they have the same precedence and o1 is left-associative)
                        ):
                            pop o2 from the operator stack into the output queue
                        push o1 onto the operator stack
                    */
                    while op_stack
                        .last()
                        .map(|&o2| o2.precedence() >= op.precedence())
                        .unwrap_or(false)
                    {
                        Self::eval_one_from_op_stack(&mut op_stack, &mut number_stack);
                    }
                    op_stack.push(op);
                }
            }
        }
        while !op_stack.is_empty() {
            Self::eval_one_from_op_stack(&mut op_stack, &mut number_stack);
        }
        if number_stack.len() != 1 {
            panic!("number_stack.len() != 1");
        }
        number_stack[0]
    }

    fn eval_one_from_op_stack(op_stack: &mut Vec<Op>, number_stack: &mut Vec<i32>) {
        op_stack.pop().and_then(|o2| {
            let rhs = number_stack.pop();
            let lhs = number_stack.pop();
            rhs.and_then(|rhs| {
                lhs.map(|lhs| {
                    let new_num = o2.eval(lhs, rhs);
                    number_stack.push(new_num);
                })
            })
        });
    }
}

#[derive(Copy, Clone)]
enum Op {
    Add,
    Subtract,
    Multiply,
    Divide,
}

impl Op {
    fn precedence(&self) -> u8 {
        match *self {
            Op::Add | Op::Subtract => 1,
            Op::Multiply | Op::Divide => 2,
        }
    }

    fn eval(&self, lhs: i32, rhs: i32) -> i32 {
        match *self {
            Op::Add => lhs + rhs,
            Op::Subtract => lhs - rhs,
            Op::Multiply => lhs * rhs,
            Op::Divide => lhs / rhs,
        }
    }
}

#[derive(Copy, Clone)]
enum Token {
    Op(Op),
    Number(i32),
}

struct Tokenizer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Tokenizer<'a> {
    fn new(str: &'a str) -> Self {
        Self {
            chars: str.chars().peekable(),
        }
    }

    fn skip_whitespaces(&mut self) {
        while self
            .chars
            .peek()
            .map(|ch| ch.is_whitespace())
            .unwrap_or(false)
        {
            self.chars.next();
        }
    }

    fn parse_integer(&mut self) -> i32 {
        let mut result = 0;
        while self
            .chars
            .peek()
            .map(|ch| {
                ch.to_digit(10)
                    .map(|d| {
                        result *= 10;
                        result += d;
                        true
                    })
                    .unwrap_or(false)
            })
            .unwrap_or(false)
        {
            self.chars.next();
        }
        result as i32
    }
}

impl<'a> Iterator for Tokenizer<'a> {
    type Item = Token;

    fn next(&mut self) -> Option<Self::Item> {
        self.skip_whitespaces();
        let next = self.chars.peek().map(|&ch| ch);
        next.map(|ch| match ch {
            '+' => {
                self.chars.next();
                Token::Op(Op::Add)
            }
            '-' => {
                self.chars.next();
                Token::Op(Op::Subtract)
            }
            '*' => {
                self.chars.next();
                Token::Op(Op::Multiply)
            }
            '/' => {
                self.chars.next();
                Token::Op(Op::Divide)
            }
            _ => Token::Number(self.parse_integer()),
        })
    }
}
// @lc code=end

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("3+2*2" => 7)]
    #[test_case(" 3/2 " => 1)]
    #[test_case(" 3+5 / 2 " => 5)]
    fn test(expr: &'static str) -> i32 {
        Solution::calculate(expr.to_string())
    }
}
