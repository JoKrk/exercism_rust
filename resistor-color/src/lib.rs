use std::fmt::{Display, Formatter, Result};


#[derive(Debug, PartialEq)]
pub enum ResistorColor {
    Black,
    Blue,
    Brown,
    Green,
    Grey,
    Orange,
    Red,
    Violet,
    White,
    Yellow,
}

impl Display for ResistorColor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match &self {
            ResistorColor::Black => write!(f, "Black"),
            ResistorColor::Blue =>  write!(f, "Blue"),
            ResistorColor::Brown =>  write!(f, "Brown"),
            ResistorColor::Green =>  write!(f, "Green"),
            ResistorColor::Grey =>  write!(f, "Grey"),
            ResistorColor::Orange =>  write!(f, "Orange"),
            ResistorColor::Red =>  write!(f, "Red"),
            ResistorColor::Violet =>  write!(f, "Violet"),
            ResistorColor::White =>  write!(f, "White"),
            ResistorColor::Yellow =>  write!(f, "Yellow"),
        }      
    }
}

pub fn color_to_value(_color: ResistorColor) -> usize {

    match _color {
        ResistorColor::Black => 0,
        ResistorColor::Brown => 1,
        ResistorColor::Red => 2,
        ResistorColor::Orange =>3,
        ResistorColor::Yellow => 4,
        ResistorColor::Green => 5,
        ResistorColor::Blue => 6,
        ResistorColor::Violet => 7,
        ResistorColor::Grey => 8,
        ResistorColor::White => 9,
    }
}

pub fn value_to_color(value: usize) -> ResistorColor {

    match value {
        0 => ResistorColor::Black,
        1 => ResistorColor::Brown,
        2 => ResistorColor::Red ,
        3 => ResistorColor::Orange,
        4 => ResistorColor::Yellow,
        5 => ResistorColor::Green,
        6 => ResistorColor::Blue,
        7 => ResistorColor::Violet,
        8 => ResistorColor::Grey,
        9 => ResistorColor::White,
        _ => ResistorColor::Black
    }
}

pub fn value_to_color_string(value: usize) -> String {
    match value {
        0 => ResistorColor::Black.to_string(),
        1 => ResistorColor::Brown.to_string(),
        2 => ResistorColor::Red.to_string(),
        3 => ResistorColor::Orange.to_string(),
        4 => ResistorColor::Yellow.to_string(),
        5 => ResistorColor::Green.to_string(),
        6 => ResistorColor::Blue.to_string(),
        7 => ResistorColor::Violet.to_string(),
        8 => ResistorColor::Grey.to_string(),
        9 => ResistorColor::White.to_string(),
        _ => String::from("value out of range")
    }
}

pub fn colors() -> Vec<ResistorColor> {
    
    let mut colors = Vec::new();
    for n in 0..10 {
        colors.push(value_to_color(n));
    }
    return colors
}
