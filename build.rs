// Copy scripts director next to the executable

use std::env;
use std::path::Path;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=scripts");
    let input_path = Path::new(&env::var("CARGO_MANIFEST_DIR").unwrap()).join("scripts");
    let output_path = Path::new(&get_output_path()).join("scripts");
    if output_path.exists() {
        std::fs::remove_dir_all(&output_path).unwrap();
    }
    std::fs::create_dir_all(&output_path).unwrap();
    copy_dir(&input_path, &output_path);
}

fn get_output_path() -> PathBuf {
    let manifest_dir_string = env::var("CARGO_MANIFEST_DIR").unwrap();
    let build_type = env::var("PROFILE").unwrap();
    let path = Path::new(&manifest_dir_string)
        .join("target")
        .join(build_type);
    return PathBuf::from(path);
}

fn copy_dir(src: &Path, dst: &Path) {
    for entry in src.read_dir().unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            let new_dst = dst.join(path.file_name().unwrap());
            std::fs::create_dir_all(&new_dst).unwrap();
            copy_dir(&path, &new_dst);
        } else {
            std::fs::copy(&path, dst.join(path.file_name().unwrap())).unwrap();
        }
    }
}
