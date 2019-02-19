use sha2::digest::FixedOutput;
use sha2::{Digest, Sha256};
use std::path::Path;

pub type Hash = String;

fn hash(input: Vec<u8>) -> String {
  let mut hash = Sha256::default();
  hash.input(input);
  format!("{:x}", hash.fixed_result())
}

fn get_file_hash(path: &Path, root: &Path) -> Hash {
  let contents = std::fs::read(path).unwrap();
  let contents_hash = hash(contents);
  let local_filename = path.strip_prefix(root).unwrap().to_str().unwrap();
  let out = hash(
    [local_filename, contents_hash.as_str()]
      .join(" ")
      .into_bytes(),
  );
  println!("{} file {:?}", out, local_filename);
  out
}

pub fn get_dir_hash(path: &Path, root: &Path) -> std::io::Result<Hash> {
  // List the directory contents
  let local_dirname = path.strip_prefix(root).unwrap().to_str().unwrap();
  let mut hashes = vec![];
  for entry in std::fs::read_dir(path)? {
    let entry = entry?;
    let path = entry.path();
    if path.is_dir() {
      let hash = get_dir_hash(&path, root).unwrap();
      hashes.push(hash);
    } else {
      let hash = get_file_hash(&path, root);
      hashes.push(hash);
    }
  }
  let out = hash(hashes.join(" ").into_bytes());
  println!("{} dir  {:?}", out, local_dirname);
  Ok(out)
}
