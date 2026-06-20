# Trve Bevy

> A collection of opinionated plugins and tools for making Bevy games.

## Usage

This repository consists of several crates that can be specified as git dependencies in your `Cargo.toml` file. It is recommended that you use either `tag` or `rev` to specify a version. For example:

```toml
trve_bevy_localization = { git = "https://github.com/TrveSoft/trve_bevy.git", tag = "trve_bevy_localization-v0.1.0" }
```

NOTE: When using git dependencies, cargo will not automatically update them using SemVer rules. It is up to you to update the `tag` or `rev` then running `cargo update`.

Refer to ["Specifying dependencies from git repositories"](https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html#specifying-dependencies-from-git-repositories) for more information.

You can also depend the main `trve_bevy` crate to access more general components and utilities:

```toml
trve_bevy = { git = "https://github.com/TrveSoft/trve_bevy.git", tag = "trve_bevy-v0.2.0" }
```

## Bevy compatibility

| `bevy` Version | `trve_bevy` Version |
| -------------- | ------------------- |
| 0.19           | 0.2                 |
| 0.18           | 0.1                 |

## License

Except where noted (below and/or in individual files), all code in this repository is dual-licensed under either:

- MIT License ([LICENSE-MIT](LICENSE-MIT) or [http://opensource.org/licenses/MIT](http://opensource.org/licenses/MIT))
- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or [http://www.apache.org/licenses/LICENSE-2.0](http://www.apache.org/licenses/LICENSE-2.0))

at your option.

### Your contributions

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.
