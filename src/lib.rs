//! TODO

use bevy_ecs::{
    entity::Entity, hierarchy::ChildOf, observer::Observer, spawn::SpawnableList, world::World,
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
