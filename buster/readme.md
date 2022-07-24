# web server directory probing

inspired by dirbuster and gobuster

``` buster 0.1.0
simple web server probing

USAGE:
    buster.exe [OPTIONS] <URL> <FILE>

ARGS:
    <URL>     Url to fuzz
    <FILE>    Input file

OPTIONS:
    -e, --exclude-status-code <EXCLUDE_STATUS_CODE>    Exclude status code [default: 404]
    -h, --help                                         Print help information
    -t, --threads <THREADS>                            Number of threads to use [default: 8]
    -V, --version                                      Print version information