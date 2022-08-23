use std::{collections::HashMap, fs};


pub struct PropertyParser {
    props: HashMap<String, String>,

    property_types: HashMap<String, PropertyType>
}

pub enum PropertyType {
    Float,
    Integer,
    String
}

pub enum PropertyResult {
    Float(f32),
    Integer(i32),
    String(String)
}

impl PropertyParser {
    pub fn new(schema_location: &str, data_location: &str) -> PropertyParser {
        

        PropertyParser {
            props: get_values(data_location),
            property_types: get_types(schema_location)
        }
    }

    pub fn get_property(&self, property: String) -> Option<PropertyResult> {
        match self.props.get(&property) {
            Some(prop) => {
                return Some(self.get_type(&property, prop));
            },
            None => None
        }
    }

    fn get_type(&self, property: &String, value: &String) -> PropertyResult
    {
        match self.property_types.get(property).unwrap() {
            PropertyType::Float => {
                return PropertyResult::Float(value.parse::<f32>().unwrap());
            },
            PropertyType::Integer => {
                return PropertyResult::Integer(value.parse::<i32>().unwrap());
            },
            PropertyType::String => {
                return PropertyResult::String(value.to_owned());
            }
        }
    }


}

impl PropertyResult {
    pub fn get_float_value(&self) -> f32 {
        match self {
            PropertyResult::Float(val) => return *val,
            _ => panic!("Accessing incorrect data type")
        }
    }

    pub fn get_int_value(&self) -> i32 {
        match self {
            PropertyResult::Integer(val) => return *val,
            _ => panic!("Accessing incorrect data type")
        }
    }

    pub fn get_string_value(&self) -> String {
        match self {
            PropertyResult::String(val) => return val.to_owned(),
            _ => panic!("Accessing incorrect data type")
        }
    }
}


fn get_values(data_location: &str) -> HashMap<String, String>{
    let mut props: HashMap<String, String> = HashMap::new();
    let contents = fs::read_to_string(data_location).unwrap();
    let word_split = contents.split("\r\n");

    for line in word_split {
        let line = String::from(line);
        let split = line.find("=");
        let key = line[0..split.unwrap()].to_string();
        let val = line[split.unwrap()+1..].to_string();

        props.insert(key, val);
    }

    props
}

fn get_types(data_location: &str) -> HashMap<String, PropertyType>{
    let mut props: HashMap<String, PropertyType> = HashMap::new();
    let contents = fs::read_to_string(data_location).unwrap();
    let word_split = contents.split("\r\n");

    for line in word_split {
        let line = String::from(line);
        let split = line.find("=");
        let key = line[0..split.unwrap()].to_string();
        let val = line[split.unwrap()+1..].to_string();

        if val == "Integer" {
            props.insert(key, PropertyType::Integer);
        }
        else
        if val == "Float" {
            props.insert(key, PropertyType::Float);
        }
        else
        if val == "String" {
            props.insert(key, PropertyType::String);
        }
    }

    props
}