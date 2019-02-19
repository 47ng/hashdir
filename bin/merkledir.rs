extern crate merkledir;
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "merkledir", about = "Generate a Merkle hash of a directory")]
struct Args {
  /// Input directory
  path: PathBuf,
}

fn main() {
  let args = Args::from_args();
  let root = args.path;
  let hash = merkledir::get_dir_hash(&root, &root).unwrap();
  println!("{}      Final hash", hash);
}
