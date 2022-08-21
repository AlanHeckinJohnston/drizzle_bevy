use bevy::{prelude::{App, Res, AssetServer, Commands, Camera2dBundle, default, ClearColor, Color, ResMut, Handle, Image, Vec3, OrthographicProjection, Transform}, DefaultPlugins, window::{WindowDescriptor, PresentMode}, render::{camera::ScalingMode}, time::Timer, ecs::bundle::Bundles};
use resources::{grid::Grid};
use systems::{spawn_system::{SpawnTimer, spawn_block}, init_dictionary::init_dictionary};
use crate::systems::move_block_system::move_block_system;
use crate::resources::sprites::Sprites;

mod systems;
mod components;
mod resources;


fn main() {
    App::new()
    .add_system(move_block_system)
    // .add_system(spawn_block)
    .add_startup_system(setup)
    .add_startup_system(init_dictionary)
    .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
    .insert_resource(Sprites::default())
    .insert_resource(WindowDescriptor { 
        width: 640.,
        height: 544.,
        title: "Drizzle".to_string(),
        present_mode: PresentMode::Fifo,
        ..default()
    })
    .insert_resource(Grid::new())
    .insert_resource(SpawnTimer(Timer::from_seconds(2.0, true)))
    .add_system(spawn_block)
    .add_plugins(DefaultPlugins)
    .run();
}


fn setup<'a>(mut commands: Commands, asset_server: Res<AssetServer>, mut sprites: ResMut<Sprites>){
    commands.spawn_bundle(
    Camera2dBundle {
        transform: Transform {
            translation: Vec3 {
                x: 168.,
                y: 136.,
                ..default()
            },
            ..default()
        },
        
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::None,
            bottom: -136.,
            top: 136.,
            left: -152.,
            right: 168.,
            ..default()
        },
        ..default()
    });
    
    let texture: Handle<Image> = asset_server.load("sprite.png");
    let file_name = "image.png".to_owned();
    sprites.insert(file_name, texture);
}