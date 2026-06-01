# transliterator

Transliterate Serbian Cyrillic ↔ Latin from files or stdin.

A Rust port of [oblakstudio/transliterator](https://github.com/oblakstudio/transliterator),
shipped as a single static binary called `transliterate`.

## Install

Prebuilt binaries for Linux (musl, x86_64 + aarch64), macOS (Intel + Apple
Silicon) and Windows are published on the [releases page]. Linux users can
also install `.deb` or `.rpm` packages from the same release.

[releases page]: https://github.com/oblakstudio/transliterator-cli/releases

Or build from source:

```sh
cargo install --path .
```

## Usage

```sh
echo "Београд" | transliterate lat        # → Beograd
echo "Београд" | transliterate cut-lat    # → Beograd  (diacritic-free)
echo "Đorđe"   | transliterate cir        # → Ђорђе
echo "Četiri"  | transliterate cut-lat    # → Cetiri

transliterate lat input.txt --out output.txt
transliterate lat a.txt b.txt c.txt       # files are concatenated in order
```

Subcommands:

- `lat` — Cyrillic → Serbian Latin (with diacritics)
- `cut-lat` — Cyrillic *and* diacritic Latin → URL-safe Latin
- `cir` — Serbian Latin → Cyrillic

With no files, input is read from stdin. With `--out PATH`, output is
written to a file instead of stdout.

## License

Dual-licensed under either [MIT](LICENSE-MIT) or
[Apache 2.0](LICENSE-APACHE), at your option.
