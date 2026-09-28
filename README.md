# Kestrel
## A fast, open-source security scanner for your machine.

Kestrel checks your system against practical security controls and gives you actionable results.

🚧 Early development is currently focused on macOS.

### Install

```sh
cargo install --git https://github.com/lkdm/kestrel
```

### Usage

```sh
# run all available checks
kestrel scan

# list available checks
kestrel list

# documentation
kestrel --help

# run specific checks
kestrel scan --checks sip-enabled filevault-enabled

# run checks matching "firewall"
kestrel scan --checks $(kestrel list --names | grep firewall)
```

### Development

Kestrel is written in Rust and is open source.
