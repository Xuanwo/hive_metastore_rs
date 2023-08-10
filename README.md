# hive_metastore &emsp; [![Build Status]][actions] [![Latest Version]][crates.io]

[Build Status]: https://img.shields.io/github/actions/workflow/status/Xuanwo/hive_metastore_rs/ci.yml
[actions]: https://github.com/Xuanwo/hive_metastore_rs/actions?query=branch%3Amain
[Latest Version]: https://img.shields.io/crates/v/hive_metastore.svg
[crates.io]: https://crates.io/crates/hive_metastore

`hive_metastore` is the hive metastore client for Rust.

```rust
// Replace with quick Start here
```

## Thrift Runtime

We are using [`tent-rs`](https://github.com/tent-rs/) provided [`tent-thrift`](https://crates.io/crates/tent-thrift) as our thrift runtime.

To use this crate, you need to add `tent-thrift` to your `Cargo.toml` in this way:

```toml
thrift = {package = "tent-thrift", version= "0.18.1"}
```

## Thrift IDL

`thrift/hms.thrift` is copied from [apache/hive](https://github.com/apache/hive/blob/rel/release-3.1.3/standalone-metastore/src/main/thrift/hive_metastore.thrift).

We removed `fb303` so that we can build without `fb303.thrift` been required.

To re-generate code, please run:

```shell
thrift -out ./src -gen rs -r thrift_idl/hms.thrift
```

We deliver the generated code in this repo, so users don't need to run this command.

## Contributing

Check out the [CONTRIBUTING.md](./CONTRIBUTING.md) guide for more details on getting started with contributing to this project.

## Getting help

Submit [issues](https://github.com/Xuanwo/hive_metastore_rs/issues/new/choose) for bug report or asking questions in [discussion](https://github.com/Xuanwo/hive_metastore_rs/discussions/new?category=q-a).

#### License

<sup>
Licensed under <a href="./LICENSE">Apache License, Version 2.0</a>.
</sup>
