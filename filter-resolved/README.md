# filter-resolved
Takes domains on stdin and output them on stdout if they resolve.<br><br>
tomnomnom's filter-resolved rewritten in Rust :rocket: :crab:
## Quick links :link::
[# Install](#Install)<br>
[# Usage](#Usage)<br>

## Install :cd:
```sh
$ cargo install --git="https://github.com/knassar702/hacks" filter-resolved
```
now you should be able to run `filter-resolved` in your terminal

## Usage :
```sh
$ cat domains.txt | filter-resolved
```

### Options :
```sh
$ filter-resolved -h
filter-resolved 0.1.0
Takes domains on stdin and output them on stdout if they resolve

USAGE:
    filter-resolved [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <c>        concurrency level [default: 20]
```
