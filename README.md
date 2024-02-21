<br />
<br />

<br />
<br />

## Reference implementation of hkt Protocol

![Buildkite](https://img.shields.io/buildkite/0eae07525f8e44a19b48fa937813e2c21ee04aa351361cd851)
![Stable Status][stable-release]
![Prerelease Status][prerelease]
[![codecov][codecov-badge]][codecov-url]
[![Discord chat][discord-badge]][discord-url]
[![Telegram Group][telegram-badge]][telegram-url]

[stable-release]: https://img.shields.io/github/v/release/hktprotocol/hktcore?label=stable
[prerelease]: https://img.shields.io/github/v/release/hktprotocol/hktcore?include_prereleases&label=prerelease
[ci-badge-master]: https://badge.buildkite.com/a81147cb62c585cc434459eedd1d25e521453120ead9ee6c64.svg?branch=master
[ci-url]: https://buildkite.com/hktprotocol/hktcore
[codecov-badge]: https://codecov.io/gh/hktprotocol/hktcore/branch/master/graph/badge.svg
[codecov-url]: https://codecov.io/gh/hktprotocol/hktcore
[discord-badge]: https://img.shields.io/discord/490367152054992913.svg
[discord-url]: https://hkt.chat
[telegram-badge]: https://cdn.jsdelivr.net/gh/Patrolavia/telegram-badge@8fe3382b3fd3a1c533ba270e608035a27e430c2e/chat.svg
[telegram-url]: https://t.me/cryptohkt

## About hkt

hkt's purpose is to enable community-driven innovation to benefit people around the world.

To achieve this purpose, _hkt_ provides a developer platform where developers and entrepreneurs can create apps that put users back in control of their data and assets, which is the foundation of ["Open Web" movement][open-web-url].

One of the components of _hkt_ is the hkt Protocol, an infrastructure for server-less applications and smart contracts powered by a blockchain.
hkt Protocol is built to deliver usability and scalability of modern PaaS like Firebase at fraction of the prices that blockchains like Ethereum charge.

Overall, _hkt_ provides a wide range of tools for developers to easily build applications:

- [JS Client library][js-api] to connect to hkt Protocol from your applications.
- [Rust][rust-sdk] and [AssemblyScript][as-sdk] SDKs to write smart contracts and stateful server-less functions.
- [Numerous examples][examples-url] with links to hack on them right inside your browser.
- [Lots of documentation][docs-url], with [Tutorials][tutorials-url] and [API docs][api-docs-url].

[open-web-url]: https://techcrunch.com/2016/04/10/1301496/
[js-api]: https://github.com/hkt/hkt-api-js
[rust-sdk]: https://github.com/hkt/hkt-sdk-rs
[as-sdk]: https://github.com/hkt/hkt-sdk-as
[examples-url]: https://hkt.dev
[docs-url]: https://docs.hkt.org
[tutorials-url]: https://docs.hkt.org/tutorials/welcome
[api-docs-url]: https://docs.hkt.org/api/rpc/introduction

## Join the Network

The easiest way to join the network, is by using the `hktup` command, which you can install as follows:

```bash
pip3 install --user hktup
```

You can join all the active networks:

- mainnet: `hktup run mainnet`
- testnet: `hktup run testnet`
- betanet: `hktup run betanet`

Check the `hktup` repository for [more details](https://github.com/hkt/hktup) how to run with or without docker.

To learn how to become validator, checkout [documentation](https://docs.hkt.org/docs/develop/node/validator/staking-and-delegation).

## Contributing

The workflow and details of setup to contribute are described in [CONTRIBUTING.md](CONTRIBUTING.md), and security policy is described in [SECURITY.md](SECURITY.md).
To propose new protocol changes or standards use [Specification & Standards repository](https://github.com/hktprotocol/NEPs).

## Getting in Touch

We use Zulip for semi-synchronous technical discussion, feel free to chime in:

https://hkt.zulipchat.com/

For non-technical discussion and overall direction of the project, see our Discourse forum:

https://gov.hkt.org
