//! An example of a bundle function that creates interactive buttons
//! with the help of [`SpawnObserver`].

use bevy::{ecs::system::IntoObserverSystem, prelude::*};
use bevy_tmp_spawn_observer::SpawnObserver;

fn main() -> AppExit {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, spawn_scene)
        .run()
}

fn spawn_scene(mut commands: Commands) {
    commands.spawn(Camera2d::default());
    commands.spawn(buttons());
}

fn buttons() -> impl Bundle {
    (
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            flex_direction: FlexDirection::Column,
            row_gap: Val::Px(20.0),
            ..default()
        },
        children![
            button("Click me!", on_click_rotate_hue),
            button("Click me!", on_click_rotate_hue),
            button("Click me!", on_click_rotate_hue),
        ],
    )
}

fn button<E: Event, B: Bundle, M, I: IntoObserverSystem<E, B, M>>(
    text: impl Into<String>,
    action: I,
) -> impl Bundle {
    (
        Button,
        Node {
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        BorderRadius::MAX,
        BackgroundColor(Color::srgb(0.0, 0.7, 0.9)),
        Children::spawn((
            Spawn((
                Text::new(text.into()),
                TextFont::from_font_size(40.0),
                TextColor(Color::srgb(0.1, 0.1, 0.2)),
            )),
            SpawnObserver(Observer::new(action)),
        )),
    )
}

fn on_click_rotate_hue(
    trigger: Trigger<Pointer<Click>>,
    mut color_query: Query<&mut BackgroundColor>,
) {
    if let Ok(mut color) = color_query.get_mut(trigger.target()) {
        color.0 = color.0.rotate_hue(30.0);
    }
}
