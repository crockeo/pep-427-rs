# pep-427-rs

Toolkit to provide structured information about Python's canonical binary distribution format: wheels!
This repo is named after the historical spec [PEP-0427](https://peps.python.org/pep-0427/),
but it is implemented based on the [up-to-date spec](https://packaging.python.org/en/latest/specifications/binary-distribution-format/).

It includes the following features:

- Wheel name parsing.
- `WHEEL` file parsing.

Targeted features which are not yet implemented:

- `METADATA` file parsing.
- `RECORD` file parsing.
- Opening `.whl` files to allow parsing w/o fully unpacking the file.

This library does not and will not support installing wheels.

## Usage

This library is currently not available on [crates.io](https://crates.io).
To add it to your project, you need to add a git dependency:

```toml
[dependencies.py_wheel]
git = "https://github.com/crockeo/pep-427-rs"
rev = "<HEAD of main>"
```

## License

MIT Open Source, see [LICENSE](./LICENSE).
