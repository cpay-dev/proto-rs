use std::{path::Path, process::Command};

fn main() {
  let out = Command::new("cargo")
    .args(["locate-project", "--workspace", "--message-format=plain"])
    .output()
    .expect("failed to run cargo locate-project");
  if !out.status.success() {
    panic!("locate-project failed: {}", out.status);
  }
  let manifest_path = String::from_utf8(out.stdout).unwrap();
  let root_dir = Path::new(manifest_path.trim())
    .parent()
    .unwrap()
    .to_str()
    .unwrap();
  println!("cargo:rustc-env=WORKSPACE_ROOT={}", root_dir);
}
