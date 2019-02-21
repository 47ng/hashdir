# hashdir

[![Build Status](https://travis-ci.com/47ng/hashdir.svg?branch=master)](https://travis-ci.com/47ng/hashdir)
[![Crates.io](https://img.shields.io/crates/v/hashdir.svg)](https://crates.io/crates/hashdir)
[![Crates.io](https://img.shields.io/crates/l/hashdir.svg)](LICENSE)

Generate a cryptographic view of a directory's contents.

## Usage

```rust
use hashdir::DirNode;

fn main() {
  let path = std::env::current_dir().unwrap();
  let node = DirNode::from_path(&path, &path).unwrap();
  println!("{:#?}", node);
}
```

JSON Output (with `serde_json`):

```
{
  "path": "./",
  "hash": "QZxKfmJir+ZeG3K2vNxQGgcSHjhsCroJuRGNZUth0HA=",
  "children": [
    {
      "path": "./lib.rs",
      "hash": "X65gIOpFCSJzOZTlKoSlPp2Zg02F8n6c6nYopAOXHcc=",
      "size": 3157
    },
    {
      "path": "./flat.json",
      "hash": "NAKMmbFrkytNFmsGz8pHuugiBY14DnjWGyBNwkUNR7A=",
      "size": 382
    }
  ]
}
```
