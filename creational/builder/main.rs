#![allow(unused)]

mod builders;
mod cars;
mod components;
mod director;

use builders::{Builder, CarBuilder, CarManualBuilder};
use cars::{Car, Manual};
use director::Director;

fn main() {
    let mut car_builder = CarBuilder::default();

    // Director gets the concrete builder object from the client
    // (application code). That's because application knows better which
    // builder to use to get a specific product.
    Director::construct_sports_car(&mut car_builder);

    // The final product is often retrieved from a builder object, since
    // Director is not aware and not dependent on concrete builders and
    // products.
    let car: Car = car_builder.build();
    println!("Car built: {:?}\n", car.car_type());

    let mut manual_builder = CarManualBuilder::default();

    // Director may know several building recipes.
    Director::construct_city_car(&mut manual_builder);

    // The final car manual.
    let manual: Manual = manual_builder.build();
    println!("Car manual built:\n{}", manual);
}
