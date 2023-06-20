
//! The purpose of this crate is the modelisation of complex flights that you can find in
//! danmaku games, especially touhou.
//!
//! How to use:
//! - add plugin `bevy_flight::prelude::DefaultFlightsPlugin<_>::default()` to your app.
//! - insert flight components on your bullets such as `bevy_flight::flights::AffineFlight2d`.
//!
//! How to create custom flight components:
//! - implement trait bevy_flight::flights::FlightDescriptor
//!   you are require to implement associated method
//!   `fn transform(self, t: f32, transform: &mut Transform)`
//!   where t is usually the elapsed time since the app began given by Res<Time>
//!   but may also be customized to your needs through the plugin
//!   It's done this way for the zero cost abstraction, as well as modularity
//!   You only address the memory which you wanna change.
//! - You could also implement another trait such as Translation2dDescriptor
//!   thus implementing `fn translation(&self, t: f32) -> Vec2`
//!   and call `.wrap()` on it to make it into a component
//! - add system `bevy_flight::plugin::flight_system::<T>` to your app, or if you used
//!   `Translation2dDescriptor` `bevy_flight::plugin::position2D_system::<T>`
//! - insert your custom flight component to your bullets call wrap if necessary.
//!
//! How to create sums of flights:
//! - You may create things like `bevy_flights::composites::TranslationSum2d<CircleFlight, LinearFlight>`
//!   This implements `Translation2dDescriptor`.
//! - Dont forget to add system position2D_system for TranslationSum2d<A, B>
//!   Because it's not included in the DefaultFlightsPlugin, even if A and B are.
//!   ( that would be impossible anyway, one would have to insert an infinite amount of systems )

#[cfg(test)]
mod tests;

pub mod plugin;
pub mod flights;
pub mod composites;
pub mod traits;
pub mod wrappers;
pub mod timemodifiers;
pub mod timereference;

pub mod prelude {
    pub use crate::flights::prelude::*;
    pub use crate::plugin::*;
    pub use crate::composites;
    pub use crate::traits::*;
    pub use crate::wrappers::*;
    pub use crate::timemodifiers::*;
    pub use crate::timereference::*;
}
