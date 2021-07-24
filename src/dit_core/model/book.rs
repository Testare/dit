//! Ideally, the ultimate representation of the dit model.

use super::super::{Action, State};
use super::{Ledger, Message};
use std::io::{self, BufRead as _, BufReader, Read, Write};
use std::iter::Iterator;

type MessageVec<A> = Vec<Message<A>>;

#[derive(Clone)]
pub struct Book<A: Action> {
    saved_lines: usize,
    messages: MessageVec<A>,
    state: A::State,
}

impl<A: Action> Book<A> {
    pub fn ledger(&self) -> Ledger<'_, A> {
        Ledger::from(&self.messages[..])
    }

    pub fn state(&self) -> &A::State {
        &self.state
    }

    pub fn write_changes<W: Write>(&mut self, writer: &mut W) {
        self.write_pending_changes(writer);
        self.saved_lines = self.messages.len();
    }

    pub fn write_pending_changes<W: Write>(&self, writer: &mut W) -> io::Result<()> {
        self.messages
            .iter()
            .skip(self.saved_lines)
            .try_for_each(|line| writeln!(writer, "{}", line))
    }

    // Temporarily ignored until we have a way to write headers
    fn from_read_header<I>(i: I) -> A::State
    where
        I: Iterator<Item = Result<String, io::Error>>,
    {
        i.take_while(|line| (&line).is_ok() && line.as_ref().unwrap() == "\"\"")
            .fold(A::State::default(), |state, header_line| {
                state.read_header_line(header_line.unwrap().as_str())
            })
    }

    pub fn from_read<R: Read>(r: R) -> Result<Book<A>, super::Error<A>> {
        let reader = BufReader::new(r);
        let iter = reader.lines();
        // let mode: String = iter.next();
        // do some mode checking
        let mut state = A::State::default(); //Self::from_read_header(iter.by_ref());

        let messages: MessageVec<A> = iter
            .map(|line_result| {
                line_result
                    .map_err(super::Error::IoError2)
                    .and_then(|line| {
                        serde_json::from_str::<Message<A>>(line.as_str())
                            .map_err(super::Error::SerdeError)
                    })
            })
            .collect::<Result<_, _>>()?;
        let saved_lines = messages.len();
        let state = (0..saved_lines).try_fold(A::State::default(), |state, n| {
            let (messages_to_point, rest) = messages.split_at(n);
            let next_message = rest.first().unwrap();
            Self::apply_message_internal(messages_to_point, next_message, state)
        })?;

        Ok(Book {
            messages,
            saved_lines,
            state,
        })
    }

    fn apply_message_internal(
        message_slice: &[Message<A>],
        msg: &Message<A>,
        state: A::State,
    ) -> Result<A::State, super::Error<A>> {
        let action = msg.action();
        let ledger = Ledger::from(message_slice);
        if action.applicable(&ledger, &state) {
            action.apply(&ledger.with_hash(msg.key()), state)
        } else {
            Err(super::Error::BadAction)
        }
    }

    pub fn apply_message(&mut self, msg: Message<A>) -> Result<&mut Self, super::Error<A>> {
        // Is this really the right pattern of ownership? It seems like this will create a lot of copies of default state...
        // Perhaps clone() would be better, since we need the Book to survive the message being applied. This function
        // is only invoked on new action, not on loading messages, so optimization is not as heavily needed (especially)
        // considering that we have a specifically time-consuming proof-of-work generator function
        // Or perhaps just don't pass it as mutable?
        self.state = Self::apply_message_internal(
            &self.messages[..],
            &msg,
            std::mem::take(&mut self.state),
        )?;
        self.messages.push(msg);
        Ok(self)
    }
}

/*
impl <A: Action, R:Read> TryFrom<R> for Book<A> {
    type Error = io::Error;
    fn try_from(read: R) -> Result<Book<A>, Self::Error> {
        Ok(Book::default())
    }
}
*/

impl<A: Action> Default for Book<A> {
    fn default() -> Book<A> {
        Book {
            saved_lines: 0,
            messages: Vec::new(),
            state: A::State::default(),
        }
    }
}

#[cfg(test)]
mod test {

    use super::super::super::super::mode_a::ActionA;
    use super::Book;
    use std::io::Cursor; // Change later to some test action

    #[test]
    #[should_panic]
    fn book_from_read() {
        let cursor =
            Cursor::new("Bag of beans, barely even human\nsavages, savages, wrotten to the core");

        let book = Book::<ActionA>::from_read(cursor).expect("Should be readable");
    }
}
