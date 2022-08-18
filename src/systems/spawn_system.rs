use bevy::{prelude::{Commands, Res}, time::Time};
use crate::components::block::Block;


pub struct SpawnTimer;


pub fn spawn_block(mut commands: Commands, time: Res<Time>, timer: Res<SpawnTimer>) {

}