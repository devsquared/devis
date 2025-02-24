use color_eyre::eyre::{Ok, Result};

pub fn create_note(name: String, with_toc: bool) -> Result<()> {
    println!("we would create note called: {:?}", name);
    if with_toc {
        println!("with a table of contents!");
    };

    Ok(())
}