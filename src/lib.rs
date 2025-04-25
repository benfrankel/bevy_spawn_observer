//! This crate provides [`SpawnObserver`], a custom [`SpawnableList`] enabling you
//! to add observers to your bundles.
//!
//! ```rust
//! use bevy::{ecs::spawn::SpawnWith, prelude::*};
//! use bevy_spawn_observer::SpawnObserver;
//!
//! // With `bevy_spawn_observer`:
//! fn button_new() -> impl Bundle {
//!     (
//!         Button,
//!         Children::spawn(SpawnObserver::new(|_: Trigger<Pointer<Click>>| {
//!             info!("You clicked me!");
//!         })),
//!     )
//! }
//!
//! // Without `bevy_spawn_observer`:
//! fn button_old() -> impl Bundle {
//!     (
//!         Node::default(),
//!         Children::spawn(SpawnWith::new(|parent: &mut ChildSpawner| {
//!             parent.spawn(Button).observe(|_: Trigger<Pointer<Click>>| {
//!                 info!("You clicked me!");
//!             });
//!         })),
//!     )
//! }
//! ```
//!
//! See a full example [here](https://github.com/benfrankel/bevy_spawn_observer/blob/main/examples/buttons.rs).

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
