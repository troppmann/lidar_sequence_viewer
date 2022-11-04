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

fn match_to_vecs(){
    let a = [1, 2, 3, 4, 5];
    let b = [1, 1, 3, 3, 5];

    let matching = a.iter().zip(&b).filter(|&(a, b)| a == b).count();
    println!("{}", matching);
}