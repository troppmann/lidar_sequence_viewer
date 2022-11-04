use std::path::Path;

pub mod io;
use io::read_frame;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new("../SemanticKITTI/dataset/sequences/00/velodyne/000000.bin");
    let frame = read_frame(path)?;
    println!("{:?}", frame.0[0]);
    Ok(())
}



