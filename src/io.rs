use std::{fs::{File, self}, io::Read, path::Path};
use bevy::prelude::Vec3;
use nom::{IResult, multi::many0, sequence::tuple, number::complete::le_f32};

#[derive(PartialEq, Debug)]
pub struct Point{
    pub position: Vec3,
    pub remission: f32,
}

#[derive(PartialEq, Debug)]
pub struct Frame(pub Vec<Point>);

pub struct Sequence {
    pub frames: Vec<Frame>,
    pub frame_count: usize,
}

impl Sequence{
    pub fn new(frames: Vec<Frame>) -> Self{
        let frame_count = frames.len(); 
        Self { frames: frames, frame_count}
    }
}

pub fn read_sequence_from_dir(path: &Path)-> Result<Sequence, Box<dyn std::error::Error>>{
    let paths = fs::read_dir(path)?;
    let frame_files = paths.into_iter() .filter_map(|x| x.ok().map(|entry| entry.path())).filter(|path| match path.extension() {
        Some(x) => x == "bin",
        None => false,
    });
    Ok(Sequence::new(frame_files.map(|path| read_frame(&path).unwrap()).collect()))
}


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
    let (input, (x,z,y,remission)) = tuple((le_f32,le_f32,le_f32,le_f32))(input)?;
    Ok((input, Point{position:Vec3 {x, y, z},remission}))
}