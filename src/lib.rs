
//! The purpose of this crate is the modelisation of complex flights that you can find in
//! danmaku games, especially touhou.
//!
//! How to use:
//! - add plugin bevy_flight::DefaultFlightsPlugin to your app.
//! - insert flight components on your bullets such as bevy_flight::flights::LinearFlight.
//!
//! How to create custom flight components:
//! - implement trait bevy_flight::flights::FlightDescriptor
//!   you are require to implement associated method
//!   fn transform(self, t: f32, former_transform: Transform) -> Transform
//!   where t is the elapsed time since the app began given by Res<Time>
//!   former_transform is there if do not want your FlightDescriptor to influence scale,
//!   rotation or z coordinate.
//! - add system bevy_flight::plugin::flight_system::<T> to your app.
//! - insert your custom flight component to your bullets.

#[cfg(test)]
mod tests;

mod plugin;
mod flights;

mod prelude {
    pub use crate::flights::*;
    pub use crate::plugin::*;
}
