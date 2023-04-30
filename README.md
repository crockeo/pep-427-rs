# pep-427-rs

Toolkit to provide access to Python's canonical binary distribution format:
[wheels](https://packaging.python.org/en/latest/specifications/binary-distribution-format/)!

Note that this is named after the historical spec [PEP-0427](https://peps.python.org/pep-0427/),
but it is based on the up-to-date bdist format spec on https://packaging.python.org.

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
