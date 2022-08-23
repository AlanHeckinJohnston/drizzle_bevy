#![windows_subsystem = "windows"]

use std::collections::HashMap;

use bevy::{prelude::{App, Res, AssetServer, Commands, Camera2dBundle, default, ClearColor, Color, ResMut, Handle, Image, Vec3, OrthographicProjection, Transform}, DefaultPlugins, window::{WindowDescriptor, PresentMode}, render::{camera::ScalingMode}, time::Timer};
use resources::{grid::Grid, current_word::Word};
use systems::{spawn_system::{SpawnTimer, spawn_block}, init_dictionary::init_dictionary, mouse_input_system::mouse_input_system, draw_hand::{draw_hand, ChangeHandEvent}, draw_shade::show_select, make_word::{MakeWordEvent, make_word}};
use crate::systems::move_block_system::move_block_system;
use crate::resources::sprites::Sprites;
use property_parser::{PropertyParser, PropertyType};

mod systems;
mod components;
mod resources;


fn main() {

    let mut property_types : HashMap<String, PropertyType> = HashMap::new();
    property_types.insert("spawn_delay".to_owned(), PropertyType::Float);
    let props: PropertyParser = PropertyParser::new(property_types);

    let time = props.get_property("spawn_delay".to_owned()).unwrap().get_float_value();

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
    .insert_resource(Word::new())
    .insert_resource(Grid::new())
    .insert_resource(SpawnTimer(Timer::from_seconds(time, true)))
    .add_system(spawn_block)
    .add_system(mouse_input_system)
    .add_system(draw_hand)
    .add_system(show_select)
    .add_system(make_word)
    .add_plugins(DefaultPlugins)
    .add_event::<ChangeHandEvent>()
    .add_event::<MakeWordEvent>()
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
    
    let mut i: u8 = 97;
    loop {
        let mut template = "*.png".to_owned();
        let char_as_string = String::from(i as char);
        template.replace_range(0..1, &char_as_string);
        let texture: Handle<Image> = asset_server.load(&template);
        sprites.insert(String::from(i as char), texture);
        
        if i == 121 {
            break;
        }

        i += 1;
    }

    let texture: Handle<Image> = asset_server.load("dark.png");
    sprites.insert(String::from("dark"), texture);
}