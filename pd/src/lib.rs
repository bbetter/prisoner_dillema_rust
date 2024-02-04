use std::fmt::{Display, Formatter};
use crate::Action::{Betray, Cooperate};

#[derive(PartialEq,Debug, Clone)]
pub enum Action {
    Cooperate,
    Betray
}

pub trait Strategy {
    fn name(&self) -> String;
    fn go(&self, prev_rounds: &[(Action, Action)]) -> Action;
}

#[cfg(test)]
mod tests {
    #[test]
    fn tt_vs_t(){

    }
}
