use bevy::{prelude::{App, Res, AssetServer, Commands, Camera2dBundle, default, ClearColor, Color, ResMut, Handle, Image}, DefaultPlugins, window::{WindowDescriptor, PresentMode}, render::{camera::ScalingMode}, time::Timer};
use systems::spawn_system::{SpawnTimer, spawn_block};
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
    .insert_resource(SpawnTimer(Timer::from_seconds(2.0, true)))
    .add_system(spawn_block)
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