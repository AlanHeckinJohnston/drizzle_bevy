use std::collections::HashMap;

use bevy::{prelude::{Query, Entity, EventReader, ResMut, Without, Commands}};

use crate::{resources::grid::Grid, components::{display_block::DisplayBlock, block::Block}};




pub struct MakeWordEvent;

struct Coordinate {
    x: i8, // column
    y: i32
}


pub fn make_word(
    event: EventReader<MakeWordEvent>,
    mut grid: ResMut<Grid>,
    mut query: Query<(Entity, &mut Block), Without<DisplayBlock>>,
    mut commands: Commands
) {

    let mut shift_coordinates: Vec<Coordinate> = Vec::new();



    if !event.is_empty() {
        for (entity, block) in query.iter() {
            if block.used {
                grid.remove_from_column(entity);
                commands.entity(entity).despawn();
                shift_coordinates.push(
                    Coordinate {
                        x: block.column,
                        y: block.floor
                    }
                );
            }
        }
    }

    let mut adjustments: HashMap<Entity, i8> = HashMap::new(); 

    for coord in shift_coordinates.iter() {
        for ( entity , block) in query.iter() {
            if coord.x == block.column
            && coord.y < block.floor {
                match adjustments.get(&entity) {
                    Some(e) => adjustments.insert(entity, e+1),
                    None => adjustments.insert(entity, 1)
                };
            }
        }
    }


    for (entity, mut block) in query.iter_mut() {
        match adjustments.get(&entity) {
            Some(amount) => block.floor -= *amount as i32,
            None => ()
        }
    }
}