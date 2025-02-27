//! Competition lifecycle traits that support route selection.
//!
//! This module provides the [`SelectCompete`] and [`SelectCompeteExt`] traits,
//! which are equivalents to vexide's [`Compete`] and [`CompeteExt`] traits with
//! additional support for autonomous selectors.
//! 
//! [`Compete`]: vexide::competition::Compete
//! [`CompeteExt`]: vexide::competition::CompeteExt

use alloc::boxed::Box;
use core::{future::Future, ops::ControlFlow, pin::Pin};

use vexide::prelude::CompetitionRuntime;

use crate::Selector;

/// A set of functions and routes to run when the competition is in a particular mode.
#[allow(async_fn_in_trait)]
pub trait SelectCompete: Sized {
    /// Runs when the robot is put into driver control mode.
    ///
    /// When in opcontrol mode, all device access is available including access to
    /// controller joystick values for reading user-input from drive team members.
    ///
    /// Robots may be placed into opcontrol mode at any point in the competition after
    /// connecting, but are typically placed into this mode following the autonomous
    /// period.
    async fn driver(&mut self) {}

    /// Runs when the robot is disabled.
    ///
    /// When in disabled mode, voltage commands to motors are disabled. Motors are forcibly
    /// locked to the "coast" brake mode and cannot be moved.
    ///
    /// Robots may be placed into disabled mode at any point in the competition after
    /// connecting, but are typically disabled before the autonomous period, between
    /// autonomous and opcontrol periods, and following the opcontrol period of a match.
    async fn disabled(&mut self) {}

    /// Runs when the robot becomes connected into a competition controller.
    ///
    /// See [`vexide::competition::CompetitionBuilder::on_connect`] for more information.
    async fn connected(&mut self) {}

    /// Runs when the robot disconnects from a competition controller.
    ///
    /// <section class="warning">
    ///
    /// This function does NOT run if connection to the match is lost due to
    /// a radio issue. It will only execute if the field control wire becomes
    /// physically disconnected from the controller (i.e.) from unplugging after
    /// a match ends.
    ///
    /// </section>
    ///
    /// See [`vexide::competition::CompetitionBuilder::on_disconnect`] for more information.
    async fn disconnected(&mut self) {}
    
    /// Runs immediately *before* the selected autonomous route.
    async fn before_route(&mut self) {}

    /// Runs immediately *after* the selected autonomous route.
    async fn after_route(&mut self) {}
}

/// Internal shared state for [`SelectCompete`]'s competition runtime instance.
/// 
/// This structure stores both the robot and the user's autonomous selector.
#[doc(hidden)]
pub struct SelectCompeteShared<R, S: Selector<R>> {
    robot: R,
    selector: S,
}

/// Extension methods for [`SelectCompete`].
///
/// Automatically implemented for any type implementing [`SelectCompete`].
#[allow(clippy::type_complexity)]
pub trait SelectCompeteExt<S: Selector<Self>>: SelectCompete {
    fn compete(
        self,
        selector: S,
    ) -> CompetitionRuntime<
        SelectCompeteShared<Self, S>,
        !,
        impl for<'s> FnMut(
            &'s mut SelectCompeteShared<Self, S>,
        ) -> Pin<Box<dyn Future<Output = ControlFlow<!>> + 's>>,
        impl for<'s> FnMut(
            &'s mut SelectCompeteShared<Self, S>,
        ) -> Pin<Box<dyn Future<Output = ControlFlow<!>> + 's>>,
        impl for<'s> FnMut(
            &'s mut SelectCompeteShared<Self, S>,
        ) -> Pin<Box<dyn Future<Output = ControlFlow<!>> + 's>>,
        impl for<'s> FnMut(
            &'s mut SelectCompeteShared<Self, S>,
        ) -> Pin<Box<dyn Future<Output = ControlFlow<!>> + 's>>,
        impl for<'s> FnMut(
            &'s mut SelectCompeteShared<Self, S>,
        ) -> Pin<Box<dyn Future<Output = ControlFlow<!>> + 's>>,
    > {
        CompetitionRuntime::builder(SelectCompeteShared {
            robot: self,
            selector,
        })
        .on_connect(|s| {
            Box::pin(async {
                s.robot.connected().await;
                ControlFlow::<!>::Continue(())
            })
        })
        .on_disconnect(|s| {
            Box::pin(async {
                s.robot.disconnected().await;
                ControlFlow::Continue(())
            })
        })
        .while_disabled(|s| {
            Box::pin(async {
                s.robot.disabled().await;
                ControlFlow::Continue(())
            })
        })
        .while_autonomous(|s| {
            Box::pin(async {
                s.robot.before_route().await;
                s.selector.run(&mut s.robot).await;
                s.robot.after_route().await;
                ControlFlow::Continue(())
            })
        })
        .while_driving(|s| {
            Box::pin(async {
                s.robot.driver().await;
                ControlFlow::Continue(())
            })
        })
        .finish()
    }
}

impl<R, S: Selector<Self>> SelectCompeteExt<S> for R where R: SelectCompete + 'static {}
