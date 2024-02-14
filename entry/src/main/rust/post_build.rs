use ::std::{error::Error, env, fs, path::Path, process::Command};

fn main() -> Result<(), Box<dyn Error>> {
    let target_dir = env::var("CRATE_TARGET_DIR")?;
    let script_file = Path::new(&target_dir[..]).join(".deploy.cmd");
    {
        let script_file = script_file.to_string_lossy();
        println!("调试：SCRIPT_FILE={}", script_file);
        Command::new("cmd").args([
            "/C",
            &script_file[..]
        ]).output()?;
    }
    fs::remove_file(script_file)?;
    Ok(())
}
