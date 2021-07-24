use super::super::Action;
use super::{Error, HexString, Ledger, Message};

/// WIP not sure quite how to do this.
/// A struct that can be passed with the execution of an action to have certain things happen
pub struct ActionInterface {
    invalid: Box<dyn Fn() -> ()>, // Should accept Error here, instead of returning result from `run`
    iter: Box<dyn Fn(HexString) -> ()>,
    success: Box<dyn Fn(&HexString) -> ()>,
    iter_period: u32,
}

impl ActionInterface {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn on_fail<F>(&mut self, fail_function: F) -> &mut Self
    where
        F: Fn() -> () + 'static,
    {
        self.invalid = Box::new(fail_function);
        self
    }

    pub fn on_success<F>(&mut self, success_function: F) -> &mut Self
    where
        F: Fn(&HexString) -> () + 'static,
    {
        self.success = Box::new(success_function);
        self
    }

    // On attempt would be a better name
    pub fn on_iter<F>(&mut self, iter_hook: F) -> &mut Self
    where
        F: Fn(HexString) -> () + 'static,
    {
        self.iter = Box::new(iter_hook);
        self
    }

    pub fn with_period(&mut self, period: u32) -> &mut Self {
        self.iter_period = period;
        self
    }

    pub fn run<A: Action>(
        &self,
        action: A,
        ledger: Ledger<A>,
        state: A::State,
    ) -> Result<(), Error<A>> {
        // TODO init()
        // TODO return results from hooks

        if action.applicable(&ledger, &state) {
            let next_message = ledger
                .messages()
                .last()
                .unwrap_or(&Message::<A>::default())
                .gen_next_message_with_hook(action, &state, &self.iter, self.iter_period);

            let result = next_message
                .action()
                .apply(&ledger.with_hash(next_message.key()), state)
                .map(|_| (*self.success)(next_message.key()));
            if result.is_err() {
                (*self.invalid)();
            }
            result
        } else {
            Ok((*self.invalid)())
        }

        // Should probably take a state as an input
        // Run interface start which gets an Action
        // Check if the action is valid for the state
        // If it is invalid, call invalid
        // If not invalid, generate a message with the iter hook at iter_frequency
        // ? rename iter_frequency to iter_hook_period?
        // If successful, apply function to state
        // Then call success with State
        // Ok(())
    }
}

impl Default for ActionInterface {
    fn default() -> Self {
        ActionInterface {
            iter_period: u32::MAX,
            invalid: Box::new(|| {}),
            iter: Box::new(|_| {}),
            success: Box::new(|_| {}),
        }
    }
}
