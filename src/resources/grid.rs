


pub struct Grid {
    columns: Vec<Vec<u32>>
}

impl Grid {
    pub fn new() -> Grid {
        let mut columns : Vec<Vec<u32>> = Vec::new();
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
            columns
        }
    }

    pub fn get_column_size(&self, column_number: i32) -> i32 {
        self.columns[column_number as usize].len() as i32
    }


    pub fn add_to_column(&mut self, column_number: i8, insertion: u32){
        self.columns[column_number as usize].push(insertion);
    }

    pub fn remove_from_column(&mut self, column_number: i8) { 

    }
}