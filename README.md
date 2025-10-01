# bevy_spawn_observer

[![Crates.io](https://img.shields.io/crates/v/bevy_spawn_observer.svg)](https://crates.io/crates/bevy_spawn_observer)
[![Docs](https://docs.rs/bevy_spawn_observer/badge.svg)](https://docs.rs/bevy_spawn_observer/latest/bevy_spawn_observer/)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/benfrankel/bevy_spawn_observer)

> [!WARNING]
> This crate will stop being maintained after a final update to Bevy 0.18 in the future.
> 
> You can already use the experimental [`bevy::ui_widgets::observe`](https://docs.rs/bevy/latest/bevy/ui_widgets/fn.observe.html)
> in 0.17, and a proper upstream solution is planned for 0.18.

This crate provides [`SpawnObserver`](https://docs.rs/bevy_spawn_observer/latest/bevy_spawn_observer/struct.SpawnObserver.html),
a custom [`SpawnableList`](https://docs.rs/bevy/latest/bevy/ecs/spawn/trait.SpawnableList.html)
enabling you to add observers to your bundles.

```rust
use bevy::{ecs::spawn::SpawnWith, prelude::*};
use bevy_spawn_observer::SpawnObserver;

// With `bevy_spawn_observer`:
fn button_new() -> impl Bundle {
    (
        Button,
        Children::spawn(SpawnObserver::new(|_: Trigger<Pointer<Click>>| {
            info!("You clicked me!");
        })),
    )
}

// Without `bevy_spawn_observer`:
fn button_old() -> impl Bundle {
    (
        Node::default(),
        Children::spawn(SpawnWith(|parent: &mut ChildSpawner| {
            parent.spawn(Button).observe(|_: Trigger<Pointer<Click>>| {
                info!("You clicked me!");
            });
        })),
    )
}
```

See a full example [here](https://github.com/benfrankel/bevy_spawn_observer/blob/main/examples/buttons.rs).

# Bevy version compatibility

| `bevy` version | `bevy_spawn_observer` version |
| -------------- | ----------------------------- |
| 0.17           | 0.2                           |
| 0.16           | 0.1                           |

# License

This crate is available under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-Apache-2.0) at your choice.
