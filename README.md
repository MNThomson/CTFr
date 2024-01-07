# CTFr

[<img alt="github" src="https://img.shields.io/badge/github-MNThomson/CTFr-bc3f48?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/MNThomson/CTFr)
[<img alt="crates.io" src="https://img.shields.io/crates/v/CTFr.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/CTFr)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/MNThomson/CTFr/ci.yml?branch=master&style=for-the-badge&logo=githubactions&logoColor=white" height="20">](https://github.com/MNThomson/CTFr/actions?query=branch%3Amaster)

## Introduction

CTFR is an optimized complete rewrite of [`CTFd`](https://github.com/CTFd/CTFd) written with Rust & HTMX. I've personally stuggled to run CTFd performantly, and often had CTFs I've competed in come to a stand still or `5xx` when they start.

A CTF platform frontend is extremely simplistic (mostly just static HTML), so let's use [`HTMX`](https://htmx.org/)! Of course for the backend, we need performance...so let's use [`Rust`](https://www.rust-lang.org/) (in the spirit of Meme-Driven-Development)!

## Development

CTFr can be run locally for either further development or customization.

> [!NOTE]
> **BEFORE** you run the following steps make sure:
> - You have (a recent version of) Rust installed locally on you machine ```rustup update && rustup install nightly```
> - You have `docker` & `docker-compose` installed and running

```shell
# 1. Clone the repository
git clone https://github.com/MNThomson/CTFr.git && cd CTFr

# 2. In another terminal, run the required postgres docker container
docker-compose up

# 3.1 To start developing, run CTFr
cargo run

# 3.2 Or auto rebuild/run on file save (requires the mold linker & cranelift backend)
cargo r
```

The development environment is now running and accesible at https://localhost:4321/

#### License

<sup>
Licensed under <a href="LICENSE">GPLv3</a>
</sup>
<br>
<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in this crate by you shall be licensed as above, without any additional terms or conditions
</sub>
