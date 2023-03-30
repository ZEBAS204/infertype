> You may want to check out "[TrID - File Identifier](https://mark0.net/soft-trid-e.html)", a free and underestimated CLI program that already performs similar functions with a huge database.

## About

This is a Rust command-line application that wraps infer cargo, a utility to check the file type using the magic number signature. The application makes it easy to test the [infer crate](https://docs.rs/infer/latest/infer/) without integrating it into your application.


## Usage

**Note** you don't need to have Rust installed on your device to use this CLI.

To use it, simply run the binary with the `check` command and provide the path(s) to the file you want to check. For example:

```console
$ infertype.exe check infertype.exe README.md
* "infertype.exe" application/vnd.microsoft portable-executable
* "README.md" Unknown # For this reason I made this CLI
```
