# Contributing

Fixes and additions are welcome. QOICodec is a Rust implementation of the Quite OK Image Format, with a command-line encoder and decoder.

## Before you start

For a small fix, you can open a pull request directly. For a larger change or a new feature, open an issue first so we can agree on the approach before you spend time on it.

When reporting a bug, include the command you ran, what you expected, what happened, and a small input or reproduction if you can share one.

## Making a change

1. Fork the repository and create a branch for your change.
2. Keep the change focused. For format behavior, check it against the [QOI specification](https://qoiformat.org/qoi-specification.pdf).
3. Run the checks before opening a pull request:

   ```bash
   cargo fmt --check
   cargo test
   cargo build
   ```

4. Open a pull request describing what changed and how you checked it. Link a related issue if there is one.

If a check fails, mention that in the pull request rather than leaving it unexplained. There is no required commit-message format.

## Code and tests

Use `cargo fmt` to format Rust code. Add or update tests when a change affects encoding, decoding, or error handling. Keep the implementation consistent with the existing module structure and avoid unrelated changes.
