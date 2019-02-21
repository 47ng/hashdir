extern crate hashdir;

fn main() {
  let path = std::env::current_dir().unwrap();
  let node = hashdir::DirNode::from_path(&path, &path).unwrap();
  println!("{:#?}", node);
}
