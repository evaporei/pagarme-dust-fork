# Cat-wannabe

A Rust version of the unix cat.

## Usage

```shell
cargo run my_file my_second_file
```

This will concatenate all files and put in standard output.

If one of the files doesn't exist, it will print `<name_of_file>: No such file or directory. (os code 2)` along with the other files.
