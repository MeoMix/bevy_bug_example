use bevy::prelude::*;
use bevy_ecs_tilemap::{TilemapPlugin, TilemapBundle};

fn main() {


    App::new()
        .add_plugins((DefaultPlugins, TilemapPlugin))
        .add_state::<ExampleState>()
        .add_systems(Startup, (spawn_camera, spawn_tilemap_example))
        // .add_systems(OnExit(ExampleState::State1), despawn_tilemap_example)
        // .add_systems(OnEnter(ExampleState::State2), spawn_sprite_example)
        .add_systems(PreUpdate, despawn_tilemap_example.run_if(on_exit_state1))
        .add_systems(Update, spawn_sprite_example.run_if(on_enter_state2))
        .add_systems(Update, change_example_state)
        .run();
}

#[derive(States, Default, Hash, Clone, Copy, Eq, PartialEq, Debug)]
pub enum ExampleState {
    #[default]
    State1,
    State2,
}

#[derive(Component)]
pub struct ExampleTilemap;

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn spawn_tilemap_example(mut commands: Commands) {
    info!("spawn_tilemap_example");
    commands.spawn((
        ExampleTilemap,
        TilemapBundle {
            ..default()
        },
    ));
}

pub fn despawn_tilemap_example(query: Query<Entity, With<ExampleTilemap>>, mut commands: Commands) {
    info!("despawn_tilemap_example");
    commands.entity(query.single()).despawn();
}

// TODO: Determine why this panics after despawning `TilemapBundle` components
pub fn spawn_sprite_example(mut commands: Commands) {
    info!("spawn_sprite_example");
    commands.spawn(SpriteBundle {
        ..default()
    });
}

pub fn change_example_state(state: Res<State<ExampleState>>, mut next_state: ResMut<NextState<ExampleState>>) {
    if state.get() == &ExampleState::State1 {
        info!("changing state");
        next_state.set(ExampleState::State2);
    }
}

pub fn on_exit_state1(
    state: Res<State<ExampleState>>,
    next_state: Res<NextState<ExampleState>>,
) -> bool {
    state.get() == &ExampleState::State1 && next_state.0 == Some(ExampleState::State2)
}

pub fn on_enter_state2(
    state: Res<State<ExampleState>>,
    next_state: Res<NextState<ExampleState>>,
) -> bool {
    state.get() == &ExampleState::State1 && next_state.0 == Some(ExampleState::State2)
}