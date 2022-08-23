use bevy::{prelude::{Commands, Res, Transform, ResMut}, time::{Time, Timer}, utils::default, sprite::SpriteBundle};
use crate::{components::{block::Block, letter::Letter}, resources::{sprites::Sprites, grid::Grid, dict::Dict}};
use rand::{thread_rng, Rng};


pub struct SpawnTimer(pub Timer);


pub fn spawn_block(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>, sprites: Res<Sprites>, mut grid: ResMut<Grid>, mut dict: ResMut<Dict>) {

    let mut rng = thread_rng();
    let column :i32 = rng.gen_range(1..=10);

    let x: f32 = column as f32 * 32.;

    if timer.0.tick(time.delta()).just_finished() {

        let letter = dict.get_letter();
        let transform = Transform{
            translation: bevy::prelude::Vec3 { 
                x,
                y: 298.,
                ..default()
            },
            ..default()
        };


        let entity = commands.spawn_bundle(SpriteBundle {
            texture: sprites.get(String::from(letter)).unwrap().to_owned(),
            transform,
            ..default()
        }).insert(Block::new(grid.get_column_size(column-1), column as i8))
        .insert(Letter {letter})
        .id();

        

        grid.add_to_column((column-1) as i8, entity);
    }
}