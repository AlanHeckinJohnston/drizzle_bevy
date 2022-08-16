use bevy::{prelude::{App, Res, AssetServer, Commands, Camera2dBundle, default, Transform, Vec3, ClearColor, Color}, DefaultPlugins, time::Timer, sprite::SpriteBundle, window::{WindowDescriptor, PresentMode}};
use components::block::Block;
use systems::test_system::HelloTimer;
use crate::systems::move_block_system::move_block_system;

mod systems;
mod components;

fn main() {
    App::new()
    .add_system(move_block_system)
    .insert_resource(HelloTimer(Timer::from_seconds(2.0, true)))
    .add_startup_system(setup)
    .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
    .insert_resource(WindowDescriptor { 
        width: 640.,
        height: 544.,
        title: "Drizzle".to_string(),
        present_mode: PresentMode::Fifo,
        ..default()
    })
    .add_plugins(DefaultPlugins)
    .run();
}


fn setup(mut commands: Commands, asset_server: Res<AssetServer>){

    let mut transform = Transform::default();
    transform.translation.x = -304.0;
    transform.translation.y = 272.;
    transform.scale = Vec3 {x: 1.0, y: 1.0, z: 1.0};
    
    let mut camera = Camera2dBundle::default();
    camera.transform.translation.z = 500.0;
    camera.transform.translation.x = 0.;
    camera.transform.translation.y = 0.;    
    commands.spawn_bundle(camera);
    commands.spawn_bundle(SpriteBundle {
        
        texture: asset_server.load("sprite.png"),
        transform,
        ..default()
    }).insert(Block::new('a'));
}