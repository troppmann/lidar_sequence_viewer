use std::{fs::{File, self}, io::{self, Read}, env, path::Path};

#[repr(C)]
struct Point{
    x: f32,
    y: f32,
    z: f32,
    remission: f32,
}

fn main() -> io::Result<()> {
    let path = "..SemanticKITTI/dataset/sequences/00/velodyne/000000.bin";
    let mut f = File::open(path)?;
    println!("Path: {}", path);
    let mut buffer = Vec::new();
    f.read_to_end(&mut buffer)?;
    println!("Length in Bytes: {}", buffer.len());
    println!("Number of Points: {}", buffer.len() / 16);
    
    Ok(())
}


fn print_all_files_in_dir(path: &Path) -> io::Result<()>{
    let paths = fs::read_dir(path)?;
    for path in paths{
        println!("Name: {}", path.unwrap().path().display())
    }
    Ok(())
}

fn print_current_path() -> io::Result<()>{
    println!("{:?}",env::current_dir()?.as_path());
    Ok(())
}