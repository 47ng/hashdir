#[macro_use]
extern crate custom_error;
#[macro_use]
extern crate serde_derive;

use sha2::digest::FixedOutput;
use sha2::{Digest, Sha512Trunc256};
use std::path::Path;

pub type Hash = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Node {
  File(FileNode),
  Dir(DirNode),
}

// --

custom_error! { pub NodeError
  IO { source: std::io::Error } = "",
  Foo { source: std::path::StripPrefixError } = "",
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileNode {
  /// Relative path of the file to the specified root
  pub path: String,

  /// SHA-256 hash (lowercase hex) of:
  /// - the relative path to the specified root
  /// - the contents of the file
  ///
  /// Example:
  /// hash = sha256("./path/to/foo.txt 01234567...89abcdef")
  ///                                 ^--- Note the space separator
  pub hash: Hash,

  /// Size of the file in bytes
  pub size: u64,
}

impl FileNode {
  pub fn from_path(path: &Path, root: &Path) -> Result<Self, NodeError> {
    // Bad outcomes:
    // - not a file
    // - does not exist
    // - cannot read
    // - cannot convert path to unicode String
    // - cannot get relative path
    // - cannot retrieve metadata

    let contents = std::fs::read(path)?;
    let contents_hash = hash(contents);
    let local_filename = path.strip_prefix(root)?.to_str().unwrap();
    let hash = hash(
      [local_filename, contents_hash.as_str()]
        .join(" ")
        .into_bytes(),
    );
    Ok(Self {
      path: String::from("./".to_owned() + local_filename),
      hash,
      size: path.metadata()?.len(),
    })
  }
}

// --

#[derive(Debug, Serialize, Deserialize)]
pub struct DirNode {
  /// Relative path of the directory to the specified root
  pub path: String,

  /// SHA-256 hash (lowercase hex) of the hashes of the children,
  /// separated by spaces.
  pub hash: Hash,

  pub children: Vec<Node>,
}

impl DirNode {
  pub fn from_path(path: &Path, root: &Path) -> Result<Self, NodeError> {
    // Bad outcomes:
    // - not a directory
    // - does not exist
    // - cannot read
    // - cannot convert path to unicode String
    // - cannot get relative path
    // - cannot list children

    let local_dirname = path.strip_prefix(root)?.to_str().unwrap();
    let mut children = Vec::new();

    for entry in std::fs::read_dir(path)? {
      let child = entry?;
      let path = child.path();
      if path.is_dir() {
        let node = DirNode::from_path(&path, root)?;
        children.push(Node::Dir(node));
      } else {
        let node = FileNode::from_path(&path, root)?;
        children.push(Node::File(node));
      }
    }
    let hashes: Vec<String> = children
      .iter()
      .map(|node| match &node {
        Node::Dir(n) => n.hash.clone(),
        Node::File(n) => n.hash.clone(),
      })
      .collect();
    let hash = hash(hashes.join(" ").into_bytes());
    Ok(Self {
      path: String::from("./".to_owned() + local_dirname),
      hash,
      children,
    })
  }
}

// --

fn hash(input: Vec<u8>) -> String {
  let mut hash = Sha512Trunc256::default();
  hash.input(input);
  format!("sha512/256-{}", base64::encode(&hash.fixed_result()))
}

// -----------------------------------------------------------------------------
