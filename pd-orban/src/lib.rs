use pd::{Action, Strategy};
use pd::Action::Betray;

pub struct Orban;

impl Strategy for Orban {
    fn name(&self) -> String {
        return String::from("Orban")
    }

    fn go(&self, prev_rounds: &[(Action, Action)]) -> Action {
        return Betray
    }
}


#[no_mangle]
pub extern fn get_strategy() -> *mut dyn Strategy {
    println!("get_strategy called");
    return Box::into_raw(Box::new(Orban {})) as *mut dyn Strategy
}

#[cfg(test)]
mod tests {

}
