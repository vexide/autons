use autons::{
    prelude::*,
    simple::{SimpleSelect, route},
};
use vexide::prelude::*;

struct Robot {}

impl Robot {
    async fn route_1(&mut self) {}
    async fn route_2(&mut self) {}
}

impl SelectCompete for Robot {}

#[vexide::main]
async fn main(peripherals: Peripherals) {
    let robot = Robot {};

    robot
        .compete(SimpleSelect::new(
            peripherals.display,
            [route!(Robot::route_1), route!(Robot::route_2)],
        ))
        .await;
}
