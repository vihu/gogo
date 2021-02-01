#### gogo

A mnemonic terminal url opener. Also a personal minimal terminal bookmark manager.

- Will use firefox as the default browser for opening links.
- You can use `gogo set_browser other_browser` to change preferred browser (assuming you have other_browser  available somewhere in your path of course).
- Backs up your bookmarks to a self contained directory which you can configure by setting `GOGODB_PATH` env variable.

#### Requirments

- Install [rust](https://www.rust-lang.org/tools/install) (tested with 1.46+).

#### Installation

```
$ cargo install gogo
```

#### Usage

- Add `GOGODB_PATH=/path/to/gogo.db` to your bashrc/zshrc.
- `URL` value must be parseable according to [URL standard](https://url.spec.whatwg.org/).

Some example commands you can try:

```
$ gogo add github https://github.com
$ gogo add hn https://news.ycombinator.com/
$ gogo add reddit https://old.reddit.com
$ gogo open github
$ gogo open hn
$ gogo open reddit
$ gogo list
$ gogo rm hn
$ gogo set_browser librewolf
$ gogo get_browser
```

#### Help

```
$ gogo --help
gogo 1.0
A mnemonic url opener

USAGE:
    gogo [SUBCOMMAND]

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
