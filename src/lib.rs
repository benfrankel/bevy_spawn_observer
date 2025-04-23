//! This crate provides [`SpawnObserver`], a custom [`SpawnableList`] for spawning observers,
//! which enables you to add observers to your bundles.
//!
//! ```rust
//! use bevy::prelude::*;
//! use bevy_spawn_observer::SpawnObserver;
//!
//! fn button() -> impl Bundle {
//!     (
//!         Button,
//!         Children::spawn(SpawnObserver::new(|_: Trigger<Pointer<Click>>| {
//!             info!("You clicked me!");
//!         })),
//!     )
//! }
//! ```
//!
//! Check out a [full example](https://github.com/benfrankel/bevy_spawn_observer/blob/main/examples/buttons.rs)
//! of how this looks in practice.

use bevy_ecs::{
    bundle::Bundle, entity::Entity, event::Event, hierarchy::ChildOf, observer::Observer,
    spawn::SpawnableList, system::IntoObserverSystem, world::World,
};

/// A [`SpawnableList`] that spawns an [`Observer`] as a child entity.
pub struct SpawnObserver(pub Observer);

impl SpawnableList<ChildOf> for SpawnObserver {
    fn spawn(self, world: &mut World, entity: Entity) {
        world.spawn(self.0.with_entity(entity));
    }

    // A size hint is not important for this simple use case, so return 0.
    fn size_hint(&self) -> usize {
        0
    }
}

impl SpawnObserver {
    /// Create a new [`SpawnObserver`].
    pub fn new<E: Event, B: Bundle, M, I: IntoObserverSystem<E, B, M>>(observer: I) -> Self {
        Self(Observer::new(observer))
    }
}
