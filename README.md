#### gogo

A mnemonic terminal url opener. Also a personal minimal terminal bookmark manager.

- Will use firefox as the default browser for opening links.
- You can use `gogo set_browser other_browser` to change preferred browser (assuming you have other_browser  available somewhere in your path of course).
- Backs up your bookmarks to a self contained directory which you can configure by setting `GOGODB_PATH` env variable.

#### Requirements

- Install [rust](https://www.rust-lang.org/tools/install) (tested with 1.46+).

#### Installation

```
$ cargo install gogo
```

#### Usage

- Add `GOGODB_PATH=/path/to/gogo.sqlite` to your bashrc/zshrc.
- `URL` value must be parseable according to [URL standard](https://url.spec.whatwg.org/).

#### Examples

- ##### Export `GOGODB_PATH` to somewhere...
    ```
    $ export GOGODB_PATH="/tmp/gogo.sqlite"
    ```

- ##### You start with no mappings...
    ```
    $ gogo list
    key: _browser added, value: firefox
    +----------+-----+
    | Mnemonic | URL |
    +----------+-----+
    ```

- ##### Change to a sane default browser...
    ```
    $ gogo set_browser librewolf
    key: _browser added, value: librewolf
    ```

- ##### A failure case...
    ```
    $ gogo gh
    No match found, please use add command first!
    gogo add name actual_url
    ```

- ##### Add a mapping...
    ```
    $ gogo add gh https://github.com
    key: gh added, value: https://github.com
    ```

- ##### Check your mappings...
    ```
    $ gogo list
    +----------+--------------------+
    | Mnemonic | URL                |
    +----------+--------------------+
    | gh       | https://github.com |
    +----------+--------------------+
    ```

- ##### You can directly open once a mapping is set...
    ```
    $ gogo gh
    gh maps to https://github.com, opening librewolf...
    ```

- ##### This also works...
    ```
    $ gogo open gh
    gh maps to https://github.com, opening librewolf...
    ```

- ##### You can also search some specific URLs which support querying
    ```
    $ gogo add crates https://crates.io
    key: crates added, value: https://crates.io
    $ gogo search crates gogo 
    searching crates which maps to https://crates.io for gogo...
    ```

#### Help

```
$ gogo --help
gogo 1.0
A mnemonic url opener

USAGE:
    gogo [mnemonic] [SUBCOMMAND]

ARGS:
    <mnemonic>    The mnemonic to open

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    add            Add url mnemonic mapping
    get_browser    Get currently configured browser
    help           Prints this message or the help of the given subcommand(s)
    list           List mnemonic url mapping
    open           Open url using mnemonic
    rm             Remove mnemonic
    search         Construct /search?q= query for known mnemonic
    set_browser    Allow setting preferred browser

```

All subcommands have their own help sections, for example:

```
$ gogo add --help
gogo-add 
Add url mnemonic mapping

USAGE:
    gogo add <name> <val>

ARGS:
    <name>    url name
    <val>     url value

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```
