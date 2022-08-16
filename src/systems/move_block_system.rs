use bevy::{prelude::{Query, Transform, Res}, time::Time};
use crate::components::block::Block;
use crate::components::block::STAGE_BOTTOM;
use crate::components::block::ACCELERATION_SECOND;
use crate::components::block::TERMINAL_VELOCITY;



pub fn move_block_system(mut query: Query<(&mut Transform, &mut Block)>, time: Res<Time>)
{

    let delta = time.delta_seconds();
    for (mut transform, mut block) in &mut query {
        if transform.translation.y <= STAGE_BOTTOM {
            block.velocity = 0.;
            transform.translation.y = STAGE_BOTTOM;
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