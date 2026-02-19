Currently undergoing heavy Maintanence!
Please check the [Disclaimer](#disclaimer)

# humble-cli  

<!--toc:start-->
- [humble-cli](#humble-cli)
  - [✨ Features](#features)
  - [🔧 Install](#🔧-install)
  - [🚀 Usage](#🚀-usage)
  - [Disclaimer](#disclaimer)
<!--toc:end-->

The missing command-line interface for downloading your Humble Bundle purchases!

## ✨ Features

- List all your Humble Bundle purchases
- List entries in a bundle, their file formats, and file size
- Download items in a bundle separately, and optionally filter them with
  - file format (e.g., EPUB, PDF)
  - file size
- Easily see which of your bundles have unclaimed keys
- Check your Humble Bundle Choices in current and previous months
- Search through all your purchases for a specific product

## 🔧 Install

**Option 1:** Download the binaries in the [Releases][releases] page.
Windows, macOS and Linux are supported.

**Option 2:** Install it via `cargo`:

```sh
cargo install humble-cli
```

## 🚀 Usage

To start, go to the [Humble Bundle website][hb-site] and log in.
Then find the cookie value for `_simpleauth_sess`.
This is required to interact with Humble Bundle API.

See this guide on how to find the cookie value for your browser: [Chrome][guide-chrome], [Firefox][guide-firefox], [Safari][guide-safari].

Use `humble-cli auth "<YOUR SESSION KEY>"` to store the authentication key locally for other subcommands.

After that you will have access to the following sub-commands:

```
$ humble-cli --help
The missing Humble Bundle CLI

USAGE:
    humble-cli <SUBCOMMAND>

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    auth            Set the authentication session key
    completion      Generate shell completions
    details         Print details of a certain bundle [aliases: info]
    download        Selectively download items from a bundle [aliases: d]
    help            Print this message or the help of the given subcommand(s)
    list            List all your purchased bundles [aliases: ls]
    list-choices    List your current Humble Choices
    search          Search through all bundle products for keywords

Note: `humble-cli -h` prints a short and concise overview while `humble-cli --help` gives all
details.
```

[hb-site]: https://www.humblebundle.com/

## Disclaimer

This repo is a fork of [smbl64/humble-cli](https://github.com/smbl64/humble-cli)
from back when it used to be in Rust. (It is not been rewritten in Go)
So all the credit of the initial base goes them.
My Goal is to reformat the code, and upgrade it as I go.
(Upgrading to clap v4 etc)
The code *should* work as is.
