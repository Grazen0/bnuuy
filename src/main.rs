use std::{fs, io, path::Path};

fn bnuuyfy(dir: &Path, bnuuy: &[u8], bnuuys: &mut usize) -> io::Result<()> {
    let result = fs::write(dir.join("bnuuy.png"), bnuuy);

    if result.is_ok() {
        *bnuuys += 1;
        println!("{} bnuuys", bnuuys);
    }

    for entry in fs::read_dir(dir)? {
        if let Ok(entry) = entry {
            let path = entry.path();
            if path.is_dir() {
                let _ = bnuuyfy(&path, bnuuy, bnuuys);
            }
        }
    }

    Ok(())
}

fn main() {
    let bnuuy = include_bytes!("../bnuuy.png");
    let mut bnuuys = 0;

    println!("bnuuy commence");
    let _ = bnuuyfy(&Path::new("."), bnuuy, &mut bnuuys);
    println!("bnuuy completion");
}
