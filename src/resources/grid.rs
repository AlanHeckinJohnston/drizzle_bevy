use bevy::prelude::Entity;


pub struct Grid{
    columns: Vec<Vec<Entity>>,
}

impl Grid {
    pub fn new() -> Grid {
        let mut columns : Vec<Vec<Entity>> = Vec::new();
        columns.push(Vec::new());
        columns.push(Vec::new());
        columns.push(Vec::new());
        columns.push(Vec::new());
        columns.push(Vec::new());
        columns.push(Vec::new());
        columns.push(Vec::new());
        columns.push(Vec::new());
        columns.push(Vec::new());
        columns.push(Vec::new());



        Grid { 
            columns,
        }
    }

    pub fn get_at_coordinate(&self, x: i8, y: i32) -> Option<&Entity> {
        self.columns.get(x as usize)
        .unwrap()
        .get(y as usize)
    }

    pub fn get_column_size(&self, column_number: i32) -> i32 {
        self.columns[column_number as usize].len() as i32
    }


    pub fn add_to_column(&mut self, column_number: i8, insertion: Entity){
        self.columns[column_number as usize].push(insertion);
    }

    pub fn remove_from_column(&mut self, entity: Entity){

        for columns in self.columns.iter_mut() {
            let mut remove: Vec<usize> = Vec::new();

            for (j, cells) in columns.iter_mut().enumerate() {

                if cells.id() == entity.id() {
                    remove.push(j);
                }
            }

            for to_remove in remove {
                columns.remove(to_remove);
            }
        }
    }
}