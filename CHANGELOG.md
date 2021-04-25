# Changelog

## 0.4.0 (2021-04-25)

### Added

* `impl std::error::Error for SourceError` when `ED: std::error::Error`. ([#16])

[#16]: https://github.com/azriel91/srcerr/pull/16

## 0.3.0 (2021-04-20)

### Added

* `"codespan"` feature to re-export [`codespan`] \(disabled by default\). ([#15])

### Changed

* Update [`codespan-reporting`] version from `0.11.0` to `0.11.1`. ([#15])
* Update documentation in `README.md` and `lib.rs`. ([#15])

[`codespan`]: https://docs.rs/codespan

[#15]: https://github.com/azriel91/srcerr/pull/15

## 0.2.0 (2021-02-03)

### Changed

* Removed all formatting logic, and back onto [`codespan-reporting`]. ([#11], [#12])

[#11]: https://github.com/azriel91/srcerr/issues/11
[#12]: https://github.com/azriel91/srcerr/pull/12

[`codespan-reporting`]: https://docs.rs/codespan-reporting

## 0.1.0 (2020-11-20)

### Added

* Support for plain text formatted errors. ([#3], [#4])
* Support for errors with long expression contexts. ([#3], [#4])
* Support for errors with single line expression contexts. ([#3], [#4])
* Support for errors with multi-line expression contexts. ([#3], [#4])
* Support for ANSI colored output. ([#7], [#8])

[#3]: https://github.com/azriel91/srcerr/issues/3
[#4]: https://github.com/azriel91/srcerr/pull/4
[#7]: https://github.com/azriel91/srcerr/issues/7
[#8]: https://github.com/azriel91/srcerr/pull/8
