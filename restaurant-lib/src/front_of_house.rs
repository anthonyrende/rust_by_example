// mod front_of_house { <-- not needed because the compliler knows to look in this file front_of_house.rs

pub mod hosting {
    pub fn add_to_waitlist() {}

    fn seat_at_table() {}
}

mod serving {
    fn take_order() {}

    fn serve_order() {}

    fn take_payment() {}
}
