pub mod msg;
use crate::msg::HiMsg;

pub fn sayHi() -> HiMsg {
    HiMsg { name: "Cuong".to_string(), message: "Hi Rust".to_string()}
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        assert_eq!(sayHi().name, "Cuong".to_string());
    }
}
