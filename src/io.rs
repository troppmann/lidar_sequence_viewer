use std::{fs::{File, self, ReadDir}, io::Read, path::{Path, PathBuf}};
use bevy::prelude::Vec3;
use nom::{IResult, multi::many0, sequence::tuple, number::complete::{le_f32, le_u16}};
use simple_error::bail;

#[derive(PartialEq, Debug)]
pub struct Point{
    pub position: Vec3,
    pub remission: f32,
}

#[derive(Debug)]
pub struct Label{
    pub label: u16,
    pub instance_id: u16,
}

#[derive(Debug)]
pub struct Frame{
    pub points: Vec<Point>,
    pub labels: Option<Vec<Label>>,
}

#[derive(PartialEq, Clone, Copy)]
pub enum LoadState{
    NotRequested,
    Requested,
    Loaded,
}

pub struct Sequence {
    pub point_folder: PathBuf, 
    pub label_folder: Option<PathBuf>,
    pub frames: Vec<Option<Frame>>,
    pub load_states: Vec<LoadState>,
    pub frame_count: usize,
}

fn count_frame_files_in_dir(dir: ReadDir)-> usize{
    let frame_files = dir.into_iter() .filter_map(|x| x.ok().map(|entry| entry.path())).filter(|path| match path.extension() {
        Some(x) => x == "bin",
        None => false,
    });
    frame_files.count()
}


// TODO: create own error types
pub fn read_sequence_from_dir(dir_path: PathBuf)-> Result<Sequence, Box<dyn std::error::Error>>{
    let velodyne = dir_path.join("velodyne");
    let mut frame_count = 0;
    if velodyne.is_dir() {
        let dir = fs::read_dir(&velodyne)?;
        frame_count = count_frame_files_in_dir(dir);

    }
    let labels = dir_path.join("labels");
    let label_folder = match labels.is_dir() {
        true => Some(labels),
        false => None,
    };

    if frame_count == 0 {
        bail!("Seqeunce folder d'ont have '.bin' files.")
    }
    Ok(Sequence{
        point_folder: velodyne,
        label_folder,
        frame_count,
        load_states: vec![LoadState::NotRequested; frame_count],
        frames: std::iter::repeat_with(|| None).take(frame_count).collect(),
    })
}


pub fn read_frame(points_path: PathBuf, labels_path: Option<PathBuf>) -> Result<Frame, Box<dyn std::error::Error>>{
    println!("{points_path:?}");
    let mut f = File::open(points_path)?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    let (_, points) = parse_points(&buffer).map_err(|e| e.to_owned())?;
    let mut labels = None;
    if let Some(path) = labels_path { 
        let mut f = File::open(path)?; 
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer)?;
        let (_, label_data) = parse_labels(&buffer).map_err(|e| e.to_owned())?;
        labels = Some(label_data);
    }
    Ok(Frame{points, labels})
}

fn parse_points(input: &[u8]) -> IResult<&[u8], Vec<Point>>{
    many0(read_point)(input)
}

fn read_point(input: &[u8]) -> IResult<&[u8], Point>{
    let (input, (x,z,y,remission)) = tuple((le_f32,le_f32,le_f32,le_f32))(input)?;
    Ok((input, Point{position:Vec3 {x, y, z},remission}))
}

fn parse_labels(input: &[u8]) -> IResult<&[u8], Vec<Label>>{
    many0(read_label)(input)
}

fn read_label(input: &[u8]) -> IResult<&[u8], Label>{
    let (input, (label, instance_id)) = tuple((le_u16, le_u16))(input)?;
    Ok((input, Label{label, instance_id}))
}


