use bevy::{prelude::{Res, Input, MouseButton, ResMut, Query, EventWriter, Vec2}, window::Windows};

use crate::{resources::{current_word::Word, grid::Grid, dict::Dict}, components::{letter::Letter, block::Block}};

use super::{draw_hand::ChangeHandEvent, make_word::MakeWordEvent};



pub const WINDOW_HEIGHT: f32 = 544.;
pub const WINDOW_WIDTH: f32 = 640.;




pub fn mouse_input_system(
    buttons: Res<Input<MouseButton>>, 
    windows: Res<Windows>, 
    mut query: Query<(&mut Block, &Letter)>, 
    mut word: ResMut<Word>, 
    grid: Res<Grid>,
    dict: Res<Dict>,
    mut change_hand_event: EventWriter<ChangeHandEvent>,
    mut make_word_event: EventWriter<MakeWordEvent>
) {

    if buttons.just_pressed(MouseButton::Left) {
        let window = windows.get_primary().unwrap();

        match window.cursor_position() {
            Some(position) => {

                let window_width = window.width();
                let window_height = window.height();


                let (column, index) = get_grid_coordinates(position, window_height, window_width);
                
                let entity = grid.get_at_coordinate(column, index);

                match entity {
                    Some(entity) => {

                        let (mut block, letter) = query.get_mut(*entity).unwrap();

                        if block.used == false {
                            block.used = true;
                            let letter = letter.letter;
                            word.word.push(letter);
                            let send = ChangeHandEvent{};
                            change_hand_event.send(send);
                        }
                        else {
                            if dict.is_word(&word.word) {
                                let send: MakeWordEvent = MakeWordEvent{};
                                make_word_event.send(send);
                                println!("is a word - sending event");

                                let send = ChangeHandEvent{};
                                change_hand_event.send(send);
                                word.word = String::from("");
                            }
                        }
                    },
                    None => ()
                }
            },
            None => ()
        }

    }
    else if buttons.just_pressed(MouseButton::Right) {
        word.word = "".to_owned();
        let send = ChangeHandEvent{};
        change_hand_event.send(send);

        for (mut block, _) in query.iter_mut() {
            block.used = false;
        }
    }
}

pub fn get_grid_coordinates(position: Vec2, window_height: f32, window_width: f32) -> (i8, i32) {
    let multiply_factor_x = window_width / WINDOW_WIDTH;
    let multiply_factor_y = window_height / WINDOW_HEIGHT;

    let column = (position.x  / (64. * multiply_factor_x)) as i8;
    let index: i32 = ((position.y / (64. * multiply_factor_y)) as i32) - 1;

    (column, index)
}