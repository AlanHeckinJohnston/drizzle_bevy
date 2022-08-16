use bevy::{time::{Time, Timer}, prelude::{Res, ResMut}};


pub struct HelloTimer(pub Timer);

pub fn hello_world(time: Res<Time>, mut timer: ResMut<HelloTimer>)
{
    if timer.0.tick(time.delta()).just_finished() {
        println!("hello world");
    }
    
}