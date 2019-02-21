use hashdir::DirNode;

fn main() {
  let path = std::env::current_dir().unwrap();
  let node = DirNode::from_path(&path, &path).unwrap();
  println!("{:#?}", node);
}
