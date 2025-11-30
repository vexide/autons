# autons

Autonomous Selection & Routing Library for `vexide`.

`autons` is a library that provides standard interfaces for autonomous route selectors
in programs using the `vexide` robotics runtime. `autons` also provides some basic reference
implementations of such selectors (see: [`SimpleSelect`]) that can be used for selecting
different autonomous routes.

[`SimpleSelect`]: crate::simple::SimpleSelect

An "autonomous route" is an asynchronous function takes a robot struct and runs during the
autonomous period of a robotics match. When programming robots, we often have many different
routes that we want to choose between depending on conditions of the match. Rather than
uploading these routes as individual programs to different slots, `autons` provides a way for
us to choose between these routes in a single program at runtime.

```rs
async fn route_1(robot: &mut MyRobot) {
    // do stuff...
}

async fn route_2(robot: &mut MyRobot) {
    // do other stuff...
}

async fn skills(robot: &mut MyRobot) {
    // run skills...
}
```

# Usage

In `vexide`, you normally have only one `autonomous` function provided to you through the `Compete`
trait:

```rs
impl Compete for MyRobot {
    async fn autonomous(&mut self) {
        // route goes here...
    }
}
```

With `autons`, you can instead use the [`SelectCompete`] trait, which is a modified version of
vexide's `Compete` trait that allows for multiple autonomous routes chosen through a struct
implementing the [`Selector`] trait. Here is a basic example using the [`SimpleSelect`] selector
with two routes on our robot:

[`SelectCompete`]: crate::compete::SelectCompete

```rs
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

impl SelectCompete for Robot {
    async fn driver(&mut self) {
        // ...
    }
}

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
```

This will draw an autonomous selector to the display, allowing you to pick between different routes.

![Screenshot of the `SimpleSelect` menu showing two routes](https://i.imgur.com/qM9qMsd.png)