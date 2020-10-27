use std::collections::HashMap;

pub type Value = i16;
pub type ForthResult = Result<(), Error>;

pub struct Forth<'a> {
    program: Vec<&'a str>,
    stack: Vec<Value>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

pub enum Operation {
    DUP,
    DROP,
    SWAP,
    OVER,
    ADD,
    SUBTRACT,
    MULTIPLY,
    DIVIDE,
}

impl Forth<'_> {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            program: Vec::new(),
        }
    }

    pub fn stack(&self) -> Vec<Value> {
        return self.stack;
    }

    pub fn eval(&mut self, input: &'_ str) -> ForthResult {
        self.program = input.split(' ');

        unimplemented!("result of evaluating '{}'", input)
    }
}
