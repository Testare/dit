
//! Ideally, the ultimate representation of the dit model. 

use super::super::{Action, State};
use super::{Message, Ledger};
use std::convert::{TryFrom, TryInto};
use std::io::{self, BufReader, BufRead as _, Read, Write};
use std::iter::Iterator;

type MessageVec<A> = Vec<Message<A>>;

#[derive(Clone)]
pub struct Book<A: Action> {
    saved_lines: usize,
    ledger: Ledger<A>,
    state: A::State
}

impl <A:Action> Book<A> {
    pub fn ledger(&self) -> &Ledger<A> {
        &self.ledger
    }

    pub fn state(&self) -> &A::State {
        &self.state
    }

    pub fn write_changes<W: Write> (&mut self, writer: &mut W) {
        self.write_pending_changes(writer);
        self.saved_lines = self.ledger.ledger_vec().len();
    }

    pub fn write_pending_changes<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.ledger.iter().skip(self.saved_lines).try_for_each(|line| {
            writeln!(writer, "{}", line)
        })
    }

    // Temporarily ignored until we have a way to write headers
    fn from_read_header<I>(i: I) -> A::State 
        where I: Iterator<Item=Result<String, io::Error>> {
         i.take_while(|line|
            (&line).is_ok() && line.as_ref().unwrap() == "\"\""
        ).fold(A::State::default(), |state, header_line| {
            state.read_header_line(header_line.unwrap().as_str())
        })
    }

    pub fn from_read<R: Read>(r: R) -> Result<Book<A>, super::Error<A>> {
        let reader= BufReader::new(r);
        let iter = reader.lines();
        // let mode: String = iter.next();
        // do some mode checking
        let mut state = A::State::default(); //Self::from_read_header(iter.by_ref());
        /*let ledger: Ledger<A> = iter.map(|line_result|line_result
            .map_err(super::Error::IoError2)
            .and_then(|line|serde_json::from_str::<Message<A>>(line.as_str()).map_err(super::Error::SerdeError))).collect::<Result<_, _>> ()?;
        let saved_lines = ledger.len();
        state = ledger.iter().try_fold(state, |state, msg| msg.action().apply(state))?;*/

        iter.map(|line_result|line_result
            .map_err(super::Error::IoError2)
            .and_then(|line|serde_json::from_str::<Message<A>>(line.as_str()).map_err(super::Error::SerdeError)))
            .try_fold(Book{ state, ledger: Ledger::default(), saved_lines: 0}, |book, msg_res| msg_res.and_then(|msg|book.apply_message(msg)))

            /*
        Ok(Book{
            saved_lines,
            ledger,
            state,
        })*/
    }

    fn apply_message(mut self, msg: Message<A>) -> Result<Self, super::Error<A>> {
        let action = msg.action().clone();
        self.ledger.ledger_vec_mut().push(msg);
        let Book {ledger, state, saved_lines} = self;
        let state = action.apply(self.ledger(), self.state)?;
        return Ok( Book {
            saved_lines: self.saved_lines,
            ledger: self.ledger,
            state
        })
    }
}

/*impl <A: Action, R:Read> TryFrom<R> for Book<A> {
    type Error = io::Error;
    fn try_from(read: R) -> Result<Book<A>, Self::Error> {
        Ok(Book::default())
    }
}*/


impl <A: Action> Default for Book<A> {
    fn default() -> Book<A> {
        Book {
            saved_lines: 0,
            ledger: Ledger::default(),
            state: A::State::default()
        }
    }
}