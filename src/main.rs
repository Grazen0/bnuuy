use std::{fs, io, path::Path};

fn bnuuyfy(dir: &Path, bnuuy: &[u8], bnuuys: &mut usize) -> io::Result<()> {
    let result = fs::write(dir.join("bnuuy.png"), bnuuy);
    *bnuuys += 1;

    match result {
        Ok(_) => println!("{} bnuuys", bnuuys),
        Err(_) => eprintln!("bnuuy {} failed", bnuuys),
    }

    for entry in fs::read_dir(dir)? {
        let path = entry?.path();
        if path.is_dir() {
            bnuuyfy(&path, bnuuy, bnuuys)?;
        }
    }

    Ok(())
}

fn main() {
    let bnuuy = include_bytes!("../bnuuy.png");
    let mut bnuuys = 0;

    println!("bnuuy commence");
    bnuuyfy(&Path::new("."), bnuuy, &mut bnuuys).unwrap();
    println!("bnuuy completion");
}
