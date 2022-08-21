use bevy::{prelude::{Query, Transform, Res}, time::Time};
use crate::components::block::Block;
use crate::components::block::STAGE_BOTTOM;
use crate::components::block::ACCELERATION_SECOND;
use crate::components::block::TERMINAL_VELOCITY;



pub fn move_block_system(mut query: Query<(&mut Transform, &mut Block)>, time: Res<Time>)
{

    let delta = time.delta_seconds();
    for (mut transform, mut block) in &mut query {

        let bottom = STAGE_BOTTOM + (block.floor * 32) as f32;
        if transform.translation.y <= bottom {
            block.velocity = 0.;
            transform.translation.y = bottom as f32;
        }
        else {
            block.velocity += delta * ACCELERATION_SECOND;
        }

        if block.velocity >= TERMINAL_VELOCITY {
            block.velocity = TERMINAL_VELOCITY
        }

        transform.translation.y -= delta * block.velocity;
        
    }
}