pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod front_of_the_house {
    mod hosting {
        fn add_to_waitlist() {}

        fn seat_at_table() {}
    }
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    // Absolute path
    create::front_of_the_house::hosting::add_to_waitlist();

    // Relative path
    front_of_the_house::hosting::add_to_waitlist();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
