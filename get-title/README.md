# Get-title

Takes a list of urls and prints their titles

Tomnomnom's get-title rewritten in Rust :crab: :rocket:!

## Quick links

[# Installation](#Installation) <br>
[# Usage](#Usage) <br>

## Installation

Run the following command in your terminal/shell

```sh
$ cargo install --git="https://github.com/knassar702/hacks" get-title
```

**OR**
Dowload The source and run: 

```sh
$ cargo build --release && mv target/release/get-title ~/.cargo/bin/. 
```

now you should be able to run `get-title` in your terminal

## Usage

Basic usage would be like :

```sh
cat urls.txt | get-title
```

### Options 

run ``get-title --help`` to see available options:
<br>
```sh
$ get-title --help

get-title 0.1.0
Takes a list of urls and prints their titles

USAGE:
    get-title [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
    -c <c>        max concurrent [default: 40]
```
