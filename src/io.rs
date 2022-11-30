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

#[derive(PartialEq, Clone, Copy)]
pub enum LoadState{
    NotRequested,
    Requested,
    Loaded,
}

pub struct Sequence {
    pub folder: String, 
    pub frames: Vec<Option<Frame>>,
    pub load_states: Vec<LoadState>,
    pub frame_count: usize,
}



pub fn read_sequence_from_dir(path: String)-> Result<Sequence, Box<dyn std::error::Error>>{
    let paths = fs::read_dir(&path)?;
    let frame_files = paths.into_iter() .filter_map(|x| x.ok().map(|entry| entry.path())).filter(|path| match path.extension() {
        Some(x) => x == "bin",
        None => false,
    });
    let frame_count = frame_files.count();
    Ok(Sequence{
        folder: path,
        frame_count,
        load_states: vec![LoadState::NotRequested; frame_count],
        frames: std::iter::repeat_with(|| None).take(frame_count).collect(),
    })
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