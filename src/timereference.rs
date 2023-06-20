use bevy_time::Time;

/// The reference the bullets are gonna use.
/// Maybe to be synchronized with a server
/// bevy_time::Time implements this
pub trait TimeReference {
    fn instant(&self) -> f32;
}

impl TimeReference for Time {
    fn instant(&self) -> f32 {
        self.elapsed_seconds()
    }
}