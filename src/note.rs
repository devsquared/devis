use std::{ffi::OsStr, fs::File, io::{BufWriter, Write}, path::PathBuf};
use std::result::Result::Ok;

use color_eyre::eyre::{eyre, Result};

pub fn create_note(
    name: String,
    path: Option<PathBuf>,
    with_toc: bool,
    default_note_dir: Option<PathBuf>
) -> Result<()> {
    let final_path = match path {
        Some(p) => normalize_path(p, name.clone()),
        None => {
            match default_note_dir {
                Some(dp) => normalize_path(dp, name.clone()),
                None => return Err(eyre!("unable to find note path")), // handle this better
            }
        },
    };

    // TODO: rust's file::create truncates if the file exists; in this case, let's present the user with a "File already exists: Overwrite? (Y/N)"

    let new_note = File::create(final_path);
    match new_note {
        Ok(file) => {
            write_content(file, name, with_toc)
        },
        Err(e) => return Err(e.into()),
    }
}

fn write_content(file: File, name: String, with_toc: bool) -> Result<()> {
    let mut buf_writer = BufWriter::new(file);

    writeln!(buf_writer, "# {}", name)?;
    writeln!(buf_writer, "Have fun with this note!")?;
    writeln!(buf_writer, "")?;
    if with_toc {
        writeln!(buf_writer, "---")?;
        writeln!(buf_writer, "- [Secion 1](#section-1)")?;
        writeln!(buf_writer, "---")?;
        writeln!(buf_writer, "")?;
    }
    writeln!(buf_writer, "## Section 1")?;
    writeln!(buf_writer, "")?;

    Ok(())
}

// TODO: test this
fn normalize_path(mut path: PathBuf, name: String) -> PathBuf {
    if !check_extension_is_md(&path) {
        let base_filename = name + ".md";
        let filename = scrub_filename(base_filename);
        path.push(filename);
    }

    path
}

// TODO: test this
fn check_extension_is_md(path: &PathBuf) -> bool {
    let ext = path.extension().and_then(OsStr::to_str);

    if ext == Some("md") {
        return true;
    }

    false
}

// TODO: test this
fn scrub_filename(mut base: String) -> String {
    // first, just remove unwanted chars
    base.retain(|x| {!['#', '%', '&', '{', '}', '\\', '<', '>', '*',
        '?', '/', '$', '!', '\'', '\"', ':', '@', '+', '`', '|', '='].contains(&x)});
    // then, replace others
    base.chars().map(|c| match c {
        ' ' => '-',
        _ => c
    }).collect()
}