// This code should draw a white square in the center of the window if it runs properly.
// One in every 5-10 runs no white square will render.
// If the `multithreaded` feature is enabled in bevy's Cargo.toml then the square "always" renders (if it's a timing issue it becomes sufficiently difficult to surface with multithreading)
use bevy::prelude::*;
use bevy_egui::EguiPlugin;
use bevy_ecs_tilemap::prelude::*;

fn main() {
    App::new()
    .add_plugins((DefaultPlugins, TilemapPlugin))
    .init_state::<AppState>()
    .add_systems(Startup, setup)
    .add_systems(
        Update,
        mark_loaded.run_if(in_state(AppState::NotReady)),
    ).add_systems(
        OnEnter(AppState::Ready),
        (spawn_tilemap, spawn_sprite).chain()
    )
    .add_plugins(EguiPlugin)
    .run();
}

#[derive(States, Default, Hash, Clone, Copy, Eq, PartialEq, Debug)]
pub enum AppState {
    #[default]
    NotReady,
    Ready,
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn mark_loaded(mut next_app_state: ResMut<NextState<AppState>>) {
    next_app_state.set(AppState::Ready);
}

pub fn spawn_tilemap(mut commands: Commands) {
    let grid_size = TilemapGridSize { x: 1.0, y: 1.0 };
    let map_type = TilemapType::default();
    let map_size = TilemapSize { x: 144, y: 144 };

    commands.spawn((
        TilemapBundle {
            grid_size,
            map_type,
            size: map_size,
            storage: TileStorage::empty(map_size),
            tile_size: TilemapTileSize { x: 128.0, y: 128.0 },
            ..default()
        },
    ));
}

fn spawn_sprite(
    mut commands: Commands,
    mut tilemap_query: Query<(Entity, &mut TileStorage)>,
) {
    let (tilemap_entity, mut tile_storage) = tilemap_query.single_mut();

    let tile_pos = TilePos { x: 0, y: 0 };

    let entity = commands.spawn(TileBundle {
        position: tile_pos,
        tilemap_id: TilemapId(tilemap_entity),
        texture_index: TileTextureIndex(0),
        ..default()
    }).id();

    tile_storage.set(&tile_pos, entity);
}