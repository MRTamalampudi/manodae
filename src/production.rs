use std::sync::Arc;

use crate::{error::ParseError, symbol::Symbol};

//T means AST
#[derive(Clone)]
pub struct Production<AST, Token, TranslatorStack> {
    pub head: String,
    pub body: Vec<Symbol>,
    pub cursor_pos: usize,
    pub index: usize,
    pub error_message: Option<String>,
    pub action: Option<
        Arc<
            dyn Fn(
                &mut AST,
                &mut Vec<Token>,
                &mut Vec<TranslatorStack>,
                &mut Vec<ParseError<Token>>,
            ),
        >,
    >,
}

impl<AST, Token, TranslatorStack> std::fmt::Debug for Production<AST, Token, TranslatorStack> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Production")
            .field("head", &self.head)
            .field("body", &self.body)
            .field("cursor_pos", &self.cursor_pos)
            .field("index", &self.index)
            .field("error_message", &self.error_message)
            .finish_non_exhaustive()
    }
}

impl<AST, Token, TranslatorStack> PartialEq for Production<AST, Token, TranslatorStack> {
    fn eq(&self, other: &Self) -> bool {
        self.head == other.head
            && self.body == other.body
            && self.cursor_pos == other.cursor_pos
            && self.index == other.index
            && self.error_message == other.error_message
    }
}

impl<AST, Token, TranslatorStack> Production<AST, Token, TranslatorStack> {
    pub fn next_symbol(&self) -> Option<&Symbol> {
        if self.cursor_pos == self.body.len() {
            None
        } else {
            self.body.get(self.cursor_pos)
        }
    }
    pub fn advance_cursor(&mut self) {
        self.cursor_pos += 1;
    }
    pub fn is_augment_production(&self) -> bool {
        self.head == String::from("S'")
    }
}
