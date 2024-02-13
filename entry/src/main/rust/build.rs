use ::std::{error::Error, env, fs, path::Path};

fn main() -> Result<(), Box<dyn Error>> {
    let cc_target = env::var("TARGET")?;
    let pkg_dir = env::var("CARGO_MANIFEST_DIR")?;
    println!("调试：CC_TARGET={}; \n\tPKG_DIR={}", cc_target, pkg_dir);
    let libs_dir = Path::new(&pkg_dir[..]).join("../../../libs").join(if cc_target == "aarch64-unknown-linux-ohos" {
        "arm64-v8a"
    } else if cc_target == "armv7-unknown-linux-ohos" {
        "armeabi-v7a"
    } else if cc_target == "x86_64-unknown-linux-ohos" {
        "x86_64"
    } else {
        return Err(format!("未能识别的交叉编译目标：{}", cc_target).into());
    });
    println!("调试：LIBS_DIR={}", fs::canonicalize(&libs_dir)?.display());
    if !libs_dir.exists() {
        fs::create_dir_all(&libs_dir)?;
    }
    let out_dir = env::var("OUT_DIR")?;
    let profile_dir = Path::new(&out_dir[..]).join("../../..");
    let pkg_name = env::var("CARGO_PKG_NAME")?;
    let so_name = format!("lib{}.so", pkg_name);
    println!("调试：SO_NAME={}; \n\tPROFILE_DIR={}", so_name, fs::canonicalize(&profile_dir)?.display());
    fs::copy(
        profile_dir.as_path().join(&so_name),
        libs_dir.as_path().join(&so_name)
    )?;
    Ok(())
}
