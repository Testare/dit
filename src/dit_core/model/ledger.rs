use super::super::Action;
use super::Message;
use std::slice::Iter;
use std::iter::FromIterator;

#[derive(Clone)]
pub struct Ledger<A: Action>(Vec<Message<A>>);

impl<A: Action> Ledger<A> {
    pub fn new() -> Self {
        Ledger::default()
    }

    pub fn ledger_vec(&self) -> &Vec<Message<A>> {
        &self.0
    }
    
    pub fn ledger_vec_mut(&mut self) -> &mut Vec<Message<A>> {
        &mut self.0
    }

    pub fn iter(&self) -> Iter<Message<A>> {
        self.0.iter()
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl <A: Action> From<Vec<Message<A>>> for Ledger<A> {
    fn from(vec: Vec<Message<A>>) -> Self {
        Ledger(vec)
    }
}

impl <A: Action> From<&Vec<Message<A>>> for Ledger<A> {
    fn from(vec: &Vec<Message<A>>) -> Self {
        Ledger(vec.clone())
    }
}

impl <A: Action> Default for Ledger<A> {
    fn default() -> Ledger<A> {
        Ledger(vec![])
    }
}

impl <A: Action> FromIterator<Message<A>> for Ledger<A> {
    fn from_iter<I: IntoIterator<Item=Message<A>>> (iter: I) -> Ledger<A> {
        Ledger(iter.into_iter().collect())
    }
}