# gogo

A mnemonic terminal url opener. Also a personal minimal terminal bookmark
manager.

If you live mostly in the terminal and want quick access to some of your most
frequently visited websites, this little tool can help you achieve that.

An example scenario:

1. I often visit `https://crates.io`.
2. I often search for a crate on crates.io.

To do so in the terminal, I run **once** ```gogo add cr https://crates.io```.
This allows me to do:

```shell
$ gogo cr
opening: "https://crates.io"
```

Some websites support `/search?q={query}`, for those you can also do:
```shell
$ gogo search cr serde
opening: "https://crates.io/search?q=serde"
```

#### Requirements

- Install [rust](https://www.rust-lang.org/tools/install) (tested with 1.46+).
- Export `GOGODB_PATH` env var to something like: `/path/to/gogo.sqlite`.
- Supply a browser executable with `gogo set_browser /path/to/browser`.

PS: For MacOS browser, try this (Firefox as an example):
```shell
$ gogo set_browser /Applications/Firefox.app/Contents/MacOS/firefox-bin
```

#### Installation

```shell
$ cargo install gogo
```

#### Tips

- `gogo ls` will print an ascii table:

```shell
$ gogo ls
+-----------+--------------------+
| key       | val                |
+-----------+--------------------+
| cr        | https://crates.io  |
+-----------+--------------------+
```

- `gogo check` will print the url for mnemonic:

```shell
$ gogo check cr
value: "https://crates.io"
```

- `gogo import /path/to/exported_csv` and `gogo export` work as expected and
  output a CSV file.

- If you switch systems, just satisfy the requirements and copy over your
  `gogo.sqlite` db to your new machine.

#### Help

The help is self documenting:

```shell
$ gogo --help
A mnemonic url opener

Usage: gogo [mnemonic] [COMMAND]

Commands:
  open         Open url using mnemonic
  set_browser  Allow setting preferred browser
  rm           Remove mnemonic
  check        Check mnemonic
  import       Import CSV
  ls           List mnemonic url mapping
  get_browser  Get currently configured browser
  export       Export database to CSV
  search       Construct /search?q= query for known mnemonic
  add          Add url mnemonic mapping
  help         Print this message or the help of the given subcommand(s)

Arguments:
  [mnemonic]  The mnemonic to open

Options:
  -h, --help     Print help information
  -V, --version  Print version information
```

All subcommands have their own help sections, for example:

```shell
$ gogo add --help
Add url mnemonic mapping

Usage: gogo add <name> <val>

Arguments:
  <name>  url name
  <val>   url value

Options:
  -h, --help  Print help information
```
