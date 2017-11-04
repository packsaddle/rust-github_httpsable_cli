# github_httpsable_cli

[![crates version][crates-image]][crates-url] [![Travis-CI Status][travis-image]][travis-url] [![Appveyor Status][appveyor-image]][appveyor-url] ![license][license-image]

> Execute git command with https-url (for GitHub).

See [git_httpsable](https://github.com/packsaddle/rust-git_httpsable) for the programmatic API.
See [git_httpsable_cli](https://github.com/packsaddle/rust-git_httpsable_cli) for generic cli.

## Example

```bash
$ GITHUB_ACCESS_TOKEN=__your_access_token__ \
  github-httpsable clone https://example.com/git/repo ./target_dir
# => git clone https://__your_access_token__:x-oauth-user@example.com/git/repo ./target_dir
```

```bash
$ GIT_HTTPSABLE_USERNAME=__your_access_token__ \
  github-httpsable push https://example.com/git/repo your_branch
# => git push https://__your_access_token__:x-oauth-user@example.com/git/repo your_branch
```

NOTE: Both `GITHUB_ACCESS_TOKEN` and `GIT_HTTPSABLE_USERNAME` are ok.


## Install

Download from [Latest release](https://github.com/packsaddle/rust-github_httpsable_cli/releases/latest) for your own environment.

or

```
$ cargo install github_httpsable_cli
```

## changelog

[changelog](./changelog.md)

## License

MIT/Apache-2.0 © [Sanemat](http://sane.jp)

[travis-url]: https://travis-ci.org/packsaddle/rust-github_httpsable_cli
[travis-image]: https://img.shields.io/travis/packsaddle/rust-github_httpsable_cli/master.svg?style=flat-square&label=travis
[appveyor-url]: https://ci.appveyor.com/project/sanemat/rust-github-httpsable-cli/branch/master
[appveyor-image]: https://img.shields.io/appveyor/ci/sanemat/rust-github-httpsable-cli/master.svg?style=flat-square&label=appveyor
[crates-url]: https://crates.io/crates/github_httpsable_cli
[crates-image]: https://img.shields.io/crates/v/github_httpsable_cli.svg?style=flat-square
[license-image]: https://img.shields.io/crates/l/github_httpsable_cli.svg?style=flat-square
