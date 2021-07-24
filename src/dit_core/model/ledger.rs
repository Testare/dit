use super::super::Action;
use super::{Message, HexString};
use std::iter::FromIterator;

#[derive(Clone)]
pub struct Ledger<'a, A: Action>(&'a [Message<A>]);

pub struct PendingLedger<'a, A: Action>(&'a [Message<A>], &'a HexString);

impl<'a, A: Action> Ledger<'a, A> {

    const EMPTY_MESSAGES: [Message<A>; 0] = [];

    pub fn new() -> Self {
        Ledger(&Ledger::<A>::EMPTY_MESSAGES[..])
    }

    pub fn messages(&self)-> &[Message<A>] {
        self.0
    }

    pub fn with_hash(&self, next_hash: &'a HexString) -> PendingLedger<'a, A> {
        PendingLedger(self.0, next_hash)
    }
}

impl<'a, A:Action> PendingLedger<'a, A> {
    pub fn messages(&self)-> &[Message<A>] {
        self.0
    }

    pub fn next_hash(&self) -> &HexString {
        self.1
    }

}

impl <'a, A: Action> From<&'a [Message<A>]> for Ledger<'a, A> {
    fn from(messages: &'a [Message<A>]) -> Self {
        Ledger(messages)
    }
}

/*
impl <'a, A: Action> From<Vec<Message<A>>> for Ledger<'a, A> {
    fn from(vec: Vec<Message<A>>) -> Self {
        Ledger(&vec[..], None)
    }
}

impl <'a, A: Action> From<&'a Vec<Message<A>>> for Ledger<'a, A> {
    fn from(vec: &'a Vec<Message<A>>) -> Self {
        Ledger(vec, None)
    }
}

impl <'a, A: Action> FromIterator<Message<A>> for Ledger<'a, A> {
    fn from_iter<I: IntoIterator<Item=Message<A>>> (iter: I) -> Ledger<'a, A> {
        Ledger(&iter.into_iter().collect::<Vec<_>>(), None)
    }
}
*/

impl <'a, A: Action> Default for Ledger<'a, A> {

    fn default() -> Ledger<'a, A> {
        Ledger::new()
    }
}