use std::io::Cursor;
use std::path::PathBuf;
use std::{env, fs};

fn main() -> anyhow::Result<()> {
    let archive = fs::read(env::current_exe()?)?;
    let target_dir = PathBuf::from(".");
    zip_extract::extract(Cursor::new(archive), &target_dir, true)?;

    Ok(())
}
