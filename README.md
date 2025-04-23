# bevy_spawn_observer

[![Crates.io](https://img.shields.io/crates/v/bevy_spawn_observer.svg)](https://crates.io/crates/bevy_spawn_observer)
[![Docs](https://docs.rs/bevy_spawn_observer/badge.svg)](https://docs.rs/bevy_spawn_observer/latest/bevy_spawn_observer/)
[![License](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](https://github.com/benfrankel/bevy_spawn_observer)

This crate provides [`SpawnObserver`](https://docs.rs/bevy_spawn_observer/latest/bevy_spawn_observer/struct.SpawnObserver.html),
a custom [`SpawnableList`](https://docs.rs/bevy/latest/bevy/ecs/spawn/trait.SpawnableList.html)
enabling you to add observers to your bundles.

```rust
use bevy::prelude::*;
use bevy_spawn_observer::SpawnObserver;

fn button() -> impl Bundle {
    (
        Button,
        Children::spawn(SpawnObserver::new(|_: Trigger<Pointer<Click>>| {
            info!("You clicked me!");
        })),
    )
}
```

See a full example [here](https://github.com/benfrankel/bevy_spawn_observer/blob/main/examples/buttons.rs).

# Bevy version compatibility

| `bevy` version | `bevy_spawn_observer` version |
| -------------- | ----------------------------- |
| 0.16           | 0.1 (unreleased)              |

# License

This crate is available under either of [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-Apache-2.0) at your choice.
