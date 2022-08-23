use bevy::{prelude::{Res, Input, MouseButton, ResMut, Query, EventWriter}, window::Windows};

use crate::{resources::{current_word::Word, grid::Grid, dict::Dict}, components::{letter::Letter, block::Block}};

use super::{draw_hand::ChangeHandEvent, make_word::MakeWordEvent};





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

                let column = (position.x / 64.) as i8;
                let index: i32 = ((position.y / 64.) as i32) - 1;
                
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