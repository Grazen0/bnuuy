use std::{fs, io, path::Path};

fn bnuuyfy(dir: &Path, bnuuy: &[u8], bnuuys: &mut usize) -> io::Result<()> {
    if !dir.is_dir() {
        return Ok(());
    }

    let _ = fs::write(dir.join("bnuuy.png"), bnuuy);
    println!("{} bnuuys", bnuuys);
    *bnuuys += 1;

    for entry in fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            bnuuyfy(&path, bnuuy, bnuuys)?;
        }
    }

    Ok(())
}

fn main() {
    let bnuuy = include_bytes!("../bnuuy.png");
    let mut bnuuys = 1;

    println!("bnuuy commence");
    bnuuyfy(&Path::new("."), bnuuy, &mut bnuuys).unwrap();
    println!("bnuuy completion");
}
