use bevy::{prelude::{Commands, Res, Transform, ResMut}, time::{Time, Timer}, utils::default, sprite::SpriteBundle};
use crate::{components::block::Block, resources::sprites::Sprites};
use rand::{thread_rng, Rng};


pub struct SpawnTimer(pub Timer);


pub fn spawn_block(mut commands: Commands, time: Res<Time>, mut timer: ResMut<SpawnTimer>, sprites: Res<Sprites>) {

    let mut rng = thread_rng();
    let column :i32 = rng.gen_range(1..=10);

    let x: f32 = column as f32 * 32.;

    if timer.0.tick(time.delta()).just_finished() {
        let mut transform = Transform::default();
        transform.translation.x = x;
        transform.translation.y = 580.;
        commands.spawn_bundle(SpriteBundle {
            
            texture: sprites.get("image.png".to_string()).unwrap().to_owned(),
            transform,
            ..default()
        }).insert(Block::new('a'));
    }
}