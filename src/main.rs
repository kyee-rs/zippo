use anyhow::anyhow;
use inquire::{Confirm, Text};
use std::env;
use std::fs::File;
use zip::ZipArchive;

fn main() -> anyhow::Result<()> {
    let archive = File::open(env::current_exe()?)?;
    let mut archive = ZipArchive::new(archive)?;

    for idx in 0..archive.len() {
        let entry = archive.by_index(idx)?;
        if entry.is_file() {
            let name = entry
                .enclosed_name()
                .ok_or(anyhow!(
                    "Failed to read the content. This archive is potentially unsafe."
                ))?
                .to_string_lossy()
                .to_string();

            println!("- {name}")
        }
    }

    let confirm = Confirm::new("Are you sure you want to extract this archive? ")
        .with_default(true)
        .with_help_message(
            "This archive was bundled using Grizzly v0.1.2-rc-2 (https://github.com/12subnet/grizzly)"
        )
        .prompt();
    let path = Text::new("Extract to: ")
        .with_default(".")
        .with_placeholder(".")
        .prompt()?;

    match confirm {
        Ok(true) => {
            println!("Extracting to {}...", path);
            archive.extract(path)?;

            Ok(())
        }
        Ok(false) => Ok(()),
        Err(_) => Err(anyhow!("Failed to read your input.")),
    }
}
