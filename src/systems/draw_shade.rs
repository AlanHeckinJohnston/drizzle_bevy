use bevy::{prelude::{Query, EventReader, Transform}};

use crate::{components::{block::Block}};

use super::draw_hand::ChangeHandEvent;



pub fn show_select(
    mut query: Query<(&mut Transform, &Block)>,
    event: EventReader<ChangeHandEvent>,
)
{
    
    if !event.is_empty() {
        for (mut transform, block)  in query.iter_mut() {
            if block.used {
                transform.scale.x = 1.11;
                transform.scale.y = 1.11;
                
            }
            else
            {
                transform.scale.x = 1.;
                transform.scale.y = 1.;
            }
        }
    }
}