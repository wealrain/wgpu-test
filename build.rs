use std::env;

use anyhow::*;
use fs_extra::{copy_items, dir::CopyOptions};

fn main() -> Result<()> {
    //如果res/下发生变化，重新运行脚本
    println!("cargo:rerun-if-changed=res/"); 

    let out_dir = env::var("OUT_DIR")?;
    let mut copy_options = CopyOptions::new();
    copy_options.overwrite = true;
    let mut path_to_copy = Vec::new();
    path_to_copy.push("res/");
    copy_items(&path_to_copy, out_dir, &copy_options)?;

    Ok(())
}