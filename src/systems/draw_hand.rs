use bevy::{prelude::{Commands, EventReader, Res, Transform, Vec3, Query, Entity, With}, sprite::SpriteBundle, utils::default};

use crate::{resources::{current_word::Word, sprites::Sprites}, components::{display_block::DisplayBlock}};


pub struct ChangeHandEvent;

pub fn draw_hand(
    mut commands: Commands, 
    event: EventReader<ChangeHandEvent>,
    word: Res<Word>, 
    sprites: Res<Sprites>, 
    query: Query<Entity, With<DisplayBlock>>) {

    if !event.is_empty() {
        
        // clear old
        for entity in query.iter() {
            commands.entity(entity).despawn();
        }


        let mut offset = 0;
        for character in word.word.chars() {
            
            commands.spawn_bundle(SpriteBundle {
                transform: Transform {
                    translation: Vec3 {
                        x: ((offset as i32 * 32) + 32) as f32,
                        y: 16.,
                        z: 0.
                    },
                    ..default()
                },
                texture: sprites.get(String::from(character)).unwrap().to_owned(),
                ..default()
            }).insert(DisplayBlock{number: offset});
    
            offset += 1;
        }
    }
}