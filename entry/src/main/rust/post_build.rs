use ::std::{error::Error, env, fs, path::Path, process::{Command, Output}};

fn main() -> Result<(), Box<dyn Error>> {
    let target_dir = env::var("CRATE_TARGET_DIR")?;
    let script_file = Path::new(&target_dir[..]).join(".deploy.cmd");
    #[cfg(unix)]
    {
        use ::std::os::unix::fs::PermissionsExt;
        let mut perms = fs::metadata(script_file)?.permissions();
        permissions.set_mode(0o700);
        fs::set_permissions(script_file, perms)?;
    }
    let output = {
        let script_file = script_file.to_string_lossy();
        println!("调试：SCRIPT_FILE={}", script_file);
        let mut command = Command::new(if cfg!(windows) { "cmd" } else { "bash" });
        if cfg!(windows) {
            command.arg("/C");
        }
        command.arg(&script_file[..]);
        command.output()?
    };
    fs::remove_file(script_file)?;
    let Output {stdout, stderr, ..} = output;
    let (stdout, stderr) = if cfg!(windows) {
        (encoding_rs::GBK.decode(&stdout).0, encoding_rs::GBK.decode(&stderr).0)
    } else {
        (String::from_utf8_lossy(&stdout), String::from_utf8_lossy(&stderr))
    };
    println!("{}", stdout);
    eprintln!("{}", stderr);
    Ok(())
}
