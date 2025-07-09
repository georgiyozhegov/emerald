use std::str::Chars;

use crate::ch_group::ChGroup;

/// Удобная обёртка над строкой для токенизатора.
pub struct SourceBuffer<'s> {
    iter: Chars<'s>,
    remaining_length: usize,
}

impl<'s> SourceBuffer<'s> {
    pub fn new(source: &'s str) -> Self {
        let iter = source.chars();
        let full_length = source.len();
        Self {
            iter,
            remaining_length: full_length,
        }
    }

    /// Пропускает символы, до того момента пока условие не выполняется.
    pub fn eat_while(&mut self, predicate: impl Fn(ChGroup) -> bool) {
        while let Some(ch) = self.peek() {
            let group = ChGroup::from(ch);
            if !predicate(group) {
                break;
            }
            self.eat();
        }
    }

    /// Возвращает текущий символ, двигая курсор вперед.
    pub fn eat(&mut self) -> char {
        self.iter.next().unwrap()
    }

    /// Возвращает текущий символ, не передвигая курсор.
    pub fn peek(&self) -> Option<char> {
        let mut iter = self.iter.clone();
        iter.next()
    }

    /// Обозначает начало нового токена.
    pub fn mark_token_start(&mut self) {
        let remaining_str = self.iter.as_str();
        self.remaining_length = remaining_str.len();
    }

    /// Возвращает длину текущего токена.
    pub fn token_length(&mut self) -> usize {
        let remaining_str = self.iter.as_str();
        self.remaining_length - remaining_str.len()
    }
}
