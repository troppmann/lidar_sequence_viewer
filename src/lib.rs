use std::io::Cursor;

use byteorder::{LittleEndian, ReadBytesExt};
use serde::{Deserialize, Serialize};


use nom::{IResult, multi::many0, sequence::tuple, number::complete::le_f32};

#[repr(C)]
#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Point{
    x: f32,
    y: f32,
    z: f32,
    remission: f32,
}

pub fn byteorder_parser(buffer: &[u8]) -> Vec<Point>{
    let number_of_points = buffer.len() / 16;
    let mut points = Vec::with_capacity(number_of_points);
    let mut cursor = Cursor::new(buffer);
    for _ in 0..number_of_points {
        let point = Point{
            x: cursor.read_f32::<LittleEndian>().unwrap(),
            y: cursor.read_f32::<LittleEndian>().unwrap(),
            z: cursor.read_f32::<LittleEndian>().unwrap(),
            remission: cursor.read_f32::<LittleEndian>().unwrap(),
        };
        points.push(point);
    }
    points
}


pub fn num_parser(buffer: &[u8]) -> Vec<Point>{
    read_points(buffer).unwrap().1
}

fn read_points(input: &[u8]) -> IResult<&[u8], Vec<Point>>{
    many0(read_point)(input)
}

fn read_point(input: &[u8]) -> IResult<&[u8], Point>{
    let (input, (x,y,z,remission)) = tuple((le_f32,le_f32,le_f32,le_f32))(input)?;
    Ok((input, Point{x,y,z,remission}))
}