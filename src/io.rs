use std::{fs::{File, self, ReadDir}, io::{Read, self}, path::PathBuf};
use bevy::prelude::Vec3;
use nom::{IResult, multi::many0, sequence::tuple, number::complete::{le_f32, le_u16}};

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

pub enum FrameReadError{
    ReadFile(io::Error),
    ParseFile(String),
}

pub enum SequenceReadError{
    ReadFolder(io::Error),
    MissingFilesWithExtension(String),
    LabelFilesCountMissmatch{
        expected: usize,
        received: usize,
    },
}

impl std::fmt::Display for SequenceReadError{
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self{
            SequenceReadError::ReadFolder(error) => write!(formatter, "Cannot read folder {}", error),
            SequenceReadError::MissingFilesWithExtension(extension) => write!(formatter, "No '{}'-Files in Folder.", extension),
            SequenceReadError::LabelFilesCountMissmatch{expected, received} => 
                write!(formatter, "The amount of label files missmatch the sequences frame amount.\n Label Files: {}\n Frames in Seqeunce:{}", received, expected),
        }
    }
}

fn count_files_with_extension(dir: ReadDir, extension: &str)-> usize{
    let frame_files = dir.into_iter() .filter_map(|x| x.ok().map(|entry| entry.path())).filter(|path| match path.extension() {
        Some(x) => x == extension,
        None => false,
    });
    frame_files.count()
}
pub fn is_valid_label_dir(dir_path: PathBuf, frame_count: usize) -> Result<(), SequenceReadError>{
    let read_dir = fs::read_dir(&dir_path).map_err(|e| SequenceReadError::ReadFolder(e))?;
    let label_count = count_files_with_extension(read_dir, "label");
    if label_count == 0 {
        return Err(SequenceReadError::MissingFilesWithExtension("label".to_string()));
    } else if label_count != frame_count {
        return Err(SequenceReadError::LabelFilesCountMissmatch { 
            expected: label_count, 
            received: frame_count });
    }
    Ok(())
}
//TODO make better
pub fn read_sequence_from_dir(dir_path: PathBuf)-> Result<Sequence, SequenceReadError>{
    let mut velodyne = dir_path.join("velodyne");
    let mut frame_count = 0;
    if velodyne.is_dir() {
        let read_dir = fs::read_dir(&velodyne).map_err(|e| SequenceReadError::ReadFolder(e))?;
        frame_count = count_files_with_extension(read_dir, "bin");
    } 
    if frame_count == 0 {
        let read_dir = fs::read_dir(&dir_path).map_err(|e| SequenceReadError::ReadFolder(e))?;
        frame_count = count_files_with_extension(read_dir, "bin");
        if frame_count == 0 {
            return Err(SequenceReadError::MissingFilesWithExtension("bin".into()));
        }
        velodyne = dir_path.clone();
    }
    let labels = dir_path.join("labels");
    let mut label_folder = None;
    if labels.is_dir() {
        let read_dir = fs::read_dir(&labels).map_err(|e| SequenceReadError::ReadFolder(e))?;
        let label_count = count_files_with_extension(read_dir, "label");
        if label_count == frame_count {
            label_folder = Some(labels);
        }
    }
    Ok(Sequence{
        point_folder: velodyne,
        label_folder,
        frame_count,
        load_states: vec![LoadState::NotRequested; frame_count],
        frames: std::iter::repeat_with(|| None).take(frame_count).collect(),
    })
}


pub fn read_frame(points_path: PathBuf, labels_path: Option<PathBuf>) -> Result<Frame, FrameReadError>{
    let mut f = File::open(points_path).map_err(|e| FrameReadError::ReadFile(e))?;
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer).map_err(|e| FrameReadError::ReadFile(e))?;
    let (_, points) = parse_points(&buffer).map_err(|e| FrameReadError::ParseFile(e.to_string()))?;
    let mut labels = None;
    if let Some(path) = labels_path { 
        let mut f = File::open(path).map_err(|e| FrameReadError::ReadFile(e))?; 
        let mut buffer = Vec::new();
        f.read_to_end(&mut buffer).map_err(|e| FrameReadError::ReadFile(e))?;
        let (_, label_data) = parse_labels(&buffer).map_err(|e| FrameReadError::ParseFile(e.to_string()))?;
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


