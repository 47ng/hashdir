# hashdir

Generate a cryptographic view of a directory's contents.

```shell
$ hashdir --help
Generate a cryptographic view of a directory's contents.

USAGE:
    hashdir [OPTIONS] <directory>

ARGUMENTS
    directory             The source directory to scan

OPTIONS:
    -h, --help            Print help information
    -v, --version         Print version information
        --verbose         Print more output (with --version)

  OUTPUT OPTIONS:
    -j, --json            JSON output with metadata, minified
    -p, --json-pretty     JSON output with metadata, prettified
    -r, --root            Only display the resulting root hash
    -b, --cbor            CBOR binary output
                            Note: it is recommended to redirect the output
                            to a file, to avoid messing with the running shell:
                            $ hashdir --cbor ./foo/ > foo-hashdir.bin
        --no-metadata     Don't include files metadata.
                          Default metadata includes file size in bytes.

$ hashdir --version
hashdir 0.1.2 (0132ed39 2019-02-20)

$ hashdir --version --verbose
hashdir 0.1.2 (0132ed39 2019-02-20)
  commit: https://github.com/franky47/hashdir/commit/0132ed3913edff635c3784261dff74f3490fc1d1
  build:  https://travis-ci.com/franky47/hashdir/builds/101546740
```
