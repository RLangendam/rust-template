Fuzzing scaffold
=================

This folder is a scaffold for `cargo-fuzz` targets. To initialize fuzzing locally:

1. Enter the Nix development shell:

```
nix develop
```

2. Install `cargo-fuzz` if absent:

```
cargo install cargo-fuzz --locked
```

3. Create a new fuzz target and run it:

```
cargo fuzz add basic
cargo fuzz run basic
```

Note: `cargo-fuzz` may require a nightly toolchain for some targets. See the `cargo-fuzz` documentation for details.
