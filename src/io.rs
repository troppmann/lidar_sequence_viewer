use std::{fs::{File}, io::Read, path::Path};
use nom::{IResult, multi::many0, sequence::tuple, number::complete::le_f32};

#[derive(PartialEq, Debug)]
pub struct Point{
    x: f32,
    y: f32,
    z: f32,
    remission: f32,
}

#[derive(PartialEq, Debug)]
pub struct Frame(pub Vec<Point>);

pub fn read_frame(path: &Path) -> Result<Frame, Box<dyn std::error::Error>>{
    let mut f = File::open(path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let (_, points) = parse_points(&buffer).map_err(|e| e.to_owned())?;
    Ok(Frame(points))
}

fn parse_points(input: &[u8]) -> IResult<&[u8], Vec<Point>>{
    many0(read_point)(input)
}

fn read_point(input: &[u8]) -> IResult<&[u8], Point>{
    let (input, (x,y,z,remission)) = tuple((le_f32,le_f32,le_f32,le_f32))(input)?;
    Ok((input, Point{x,y,z,remission}))
}