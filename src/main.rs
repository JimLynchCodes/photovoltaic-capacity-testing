use std::error::Error;

use crate::{
    send_through_sieve_stack::send_through_sieve_stack, unit_test_mocks::dummy_input_data_point,
};

mod custom_types;
mod send_through_sieve_stack;
mod unit_test_mocks;

/// Runs the filtering logic locally with a custom csv
// fn main() -> Result<(), Box<dyn Error>> {
fn main() {
    // TODO - load custom csv
    let input_points = vec![dummy_input_data_point()];

    // TODO - run cvs data through "send_through_sieve_stack" function
    let filtered_points = send_through_sieve_stack(input_points);

    // TODO - print filtered data points
    println!("{:#?}", filtered_points.get(0).unwrap());

    println!("Hello, sieve stack!");
}
