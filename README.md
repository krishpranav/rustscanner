# rustscanner
A simple port scanner built using rust-lang


[![forthebadge](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

## Building:
```
cargo build
./target/debug/rustscanner
```

## Usage:
./rustscanner -h
```
USAGE:
    rustscanner [FLAGS] [OPTIONS]

FLAGS:
    -h, --help       Prints help information
    -u, --udp        Scanning with UDP
    -V, --version    Prints version information

OPTIONS:
    -c, --concurrency <concurrency>    Number of concurrent scans [default: 65535]
    -i, --ip <ip>                      Scanned IP address
    -f, --outfile <outfile>            Result output file address
    -p, --port <port>                  Port Range <port,port,port> or <port-port> [default:
                                       21,22,23,25,69,79,80,88,110,113,119,220,443,1433,1521,2082,2083,2086,2087,2095,2096,2077,2078,3306,3389,5432,6379,8080,9000,9001,9200,9300,11211,27017]
    -t, --timeout <timeout>            Timeout  Milliseconds [default: 800]

```
