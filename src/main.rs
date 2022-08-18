use bevy::{prelude::{App, Res, AssetServer, Commands, Camera2dBundle, default, Transform, Vec3, ClearColor, Color, ResMut, Handle, Image}, DefaultPlugins, sprite::{SpriteBundle}, window::{WindowDescriptor, PresentMode}, render::{camera::ScalingMode}};
use components::block::Block;
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
    .insert_resource(ClearColor(Color::rgb(1.0, 1.0, 1.0)))
    .insert_resource(Sprites::default())
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


fn setup<'a>(mut commands: Commands, asset_server: Res<AssetServer>, mut sprites: ResMut<Sprites>){
    let mut camera_bundle = Camera2dBundle::default();
    camera_bundle.transform.translation.x = 160.;
    camera_bundle.transform.translation.y = 136.;
    camera_bundle.projection.scaling_mode = ScalingMode::None;
    camera_bundle.projection.bottom = -136.;
    camera_bundle.projection.top = 136.;
    camera_bundle.projection.left = -160.;
    camera_bundle.projection.right = 160.;
    
    commands.spawn_bundle(camera_bundle);
    

    // setup_test_block(commands, asset_server);

    let texture: Handle<Image> = asset_server.load("sprite.png");
    let file_name = "image.png".to_owned();
    sprites.insert(file_name, texture);


}

#[allow(dead_code)]
fn setup_test_block(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut transform = Transform::default();
    transform.translation.x = 100.;
    transform.translation.y = 580.;
    transform.scale = Vec3 {x: 1.0, y: 1.0, z: 1.0};
    commands.spawn_bundle(SpriteBundle {
        
        texture: asset_server.load("sprite.png"),
        transform,
        ..default()
    }).insert(Block::new('a'));
}