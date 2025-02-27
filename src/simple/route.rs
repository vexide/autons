use alloc::boxed::Box;
use core::{future::Future, pin::Pin};

type RouteFn<Shared> = for<'s> fn(&'s mut Shared) -> Pin<Box<dyn Future<Output = ()> + 's>>;

/// Route entry for [`SimpleSelect`].
/// 
/// These are provided to [`SimpleSelect`] in the form of an array passed to [`SimpleSelect`].
/// Route entries contain a function pointer to the provided route function, as well as a human-readable
/// name for the route that is displayed in the selector's UI.
/// 
/// It's recommended to use the [`route!()`] macro to aid in creating instances of this struct.
/// 
/// [`SimpleSelect`]: crate::simple::SimpleSelect
#[derive(Debug, Eq, PartialEq)]
pub struct Route<R> {
    pub name: &'static str,
    pub callback: RouteFn<R>,
}

impl<R> Clone for Route<R> {
    fn clone(&self) -> Self {
        Self {
            name: self.name,
            callback: self.callback,
        }
    }
}

impl<R> Route<R> {
    pub fn new(name: &'static str, callback: RouteFn<R>) -> Self {
        Self { name, callback }
    }

    pub fn test(name: &'static str, callback: RouteFn<R>) -> Self {
        Self { name, callback }
    }
}

/// Concisely creates an instance of a [`SimpleSelectRoute`].
/// 
/// # Example
/// 
/// ```
/// let routes = [
///     route!("Route 1", Robot::route_1),
///     route!("Route 2", Robot::route_2),
/// ];
/// ```
#[macro_export]
macro_rules! route {
    ($func:path) => {{
        let type_name = ::core::any::type_name_of_val(&$func);

        ::autons::simple::Route::new(
            match &type_name[..type_name.len() - 3].rfind(':') {
                Some(pos) => &type_name[pos + 1..type_name.len() - 3],
                None => &type_name[..type_name.len() - 3],
            },
            |robot| ::alloc::boxed::Box::pin($func(robot))
        )
    }};
    ($name:expr, $func:path) => {{
        ::autons::simple::Route::new(
            $name,
            |robot| ::alloc::boxed::Box::pin($func(robot))
        )
    }};
}
pub use route;