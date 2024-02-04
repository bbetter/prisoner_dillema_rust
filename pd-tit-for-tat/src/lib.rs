use pd::{Action, Strategy};

pub struct TitForTat;

impl Strategy for TitForTat {
    fn name(&self) -> String {
        return String::from("Tit For Tat")
    }

    fn go(&self, prev_rounds: &[(Action, Action)]) -> Action {
        return if prev_rounds.is_empty() {
            Action::Cooperate
        } else {
            match prev_rounds.last() {
                None => Action::Betray,
                Some((_, enemy)) => enemy.clone()
            }
        }
    }
}


#[no_mangle]
pub extern fn get_strategy() -> *mut dyn Strategy {
    println!("get_strategy called");
    return Box::into_raw(Box::new(TitForTat {})) as *mut dyn Strategy
}

#[cfg(test)]
mod tests {

}
