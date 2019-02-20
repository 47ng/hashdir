extern crate hashdir;
extern crate structopt;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
  name = "hashdir",
  about = "Generate a cryptographic view of a directory's contents"
)]
struct Args {
  /// Input directory
  path: PathBuf,
}

fn main() {
  let args = Args::from_args();
  let root = args.path;
  let root_node = hashdir::DirNode::from_path(&root, &root).unwrap();
  println!("{}", serde_json::to_string_pretty(&root_node).unwrap());
}
