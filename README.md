# gpodderid3

This is a small program that reads the gpodder database file and trys to tag
mp3s in the download path with the right information.

It just changes the title or album tag if empty.

## Install

```
cargo build
```

## Usage

```
USAGE:
    gpodderid3 [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -d, --database <database>    Sets gpodder database [default: gpodder.db]
    -p, --path <path>            Sets download path [default: .]
```

## Contributing

PRs accepted.

## License

GNU GENERAL PUBLIC LICENSE
