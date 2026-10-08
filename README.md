# Kestrel

## A fast, open-source security scanner for your machine.

Kestrel checks your system against practical security controls and provides actionable results.

<img width="912" height="608" alt="Terminal window showing the output of `kestrel scan`" src="https://github.com/user-attachments/assets/a0a86294-a161-427c-b490-be0a60cb6fc8" />

🚧 **Early development:** Kestrel currently focuses on macOS.

## Install

```
cargo install --git https://github.com/lkdm/kestrel
```

## Usage

Run all available checks:

```
kestrel scan
```

Run specific checks by name or ID:

```
kestrel scan --checks sip-enabled filevault-enabled
```

List available checks:

```
kestrel list
```

List check names:

```
kestrel list names
```

List check IDs:

```
kestrel list ids
```

This can be useful for scripting. For example, to run all checks whose names contain `firewall`:

```
kestrel scan --checks $(kestrel list names | grep firewall)
```

You can also use stable check IDs when scripting against a specific check:

```
kestrel scan --checks 7f4a48c8-8df0-4ad4-a95d-e2ac3d6a74b1
```

View command documentation:

```
kestrel --help
```

## Development

Kestrel is written in Rust and is open source.

Build and run from the repository:

```
cargo run -- scan
```

Run the test suite:

```
cargo test
```
