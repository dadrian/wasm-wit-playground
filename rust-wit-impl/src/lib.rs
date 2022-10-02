wit_bindgen_guest_rust::export!("../exports.wit");

use crate::exports::Exports;
struct RustAPI;

impl exports::Exports for RustAPI {
    fn add_one(n: u32) -> u32 {
        return n + 1;
    }
}

#[cfg(test)]
mod tests {
    use crate::RustAPI;
    use crate::exports::Exports;

    #[test]
    fn it_works() {
        let result = RustAPI::add_one(6);
        assert_eq!(result, 7)
    }
}
