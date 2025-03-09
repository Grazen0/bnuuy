use std::{fs, path::PathBuf};

const BNUUY: &[u8] = include_bytes!("../bnuuy.png");

fn main() {
    println!("bnuuy commence");

    let mut queue = vec![PathBuf::from(".")];
    let mut bnuuy_count = 0;

    while let Some(dir) = queue.pop() {
        let result = fs::write(dir.join("bnuuy.png"), BNUUY);

        if result.is_ok() {
            bnuuy_count += 1;
            println!("{} bnuuys", bnuuy_count);
        }

        if let Ok(items) = fs::read_dir(dir) {
            for entry in items.flatten() {
                let path = entry.path();
                if path.is_dir() {
                    queue.insert(0, path);
                }
            }
        }
    }

    println!("bnuuy complete");
}
