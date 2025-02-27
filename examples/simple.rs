#![no_std]
#![no_main]

extern crate alloc;

use autons::{
    prelude::*,
    simple::{route, SimpleSelect},
};
use vexide::prelude::*;

struct Robot {}

impl Robot {
    async fn route_1(&mut self) {}
    async fn route_2(&mut self) {}
}

impl SelectedCompete for Robot {}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = Robot {};

    robot
        .compete(SimpleSelect::new(
            peripherals.display,
            [
                route!("Route 1", Robot::route_1),
                route!("Route 2", Robot::route_2),
            ],
        ))
        .await;
}
