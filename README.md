#### gogo

A mnemonic terminal url opener.

#### Requirments

- Install [rust](https://www.rust-lang.org/tools/install).
- Install [firefox](https://www.mozilla.org/en-US/firefox/new/).

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
    add     add url
    help    Prints this message or the help of the given subcommand(s)
    open    opens mnemonic url
```

```
$ gogo open --help
gogo-open 
opens mnemonic url

USAGE:
    gogo open <open>

ARGS:
    <open>    The url to open

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```

```
$ gogo add --help
gogo-add 
add url

USAGE:
    gogo add <name> <val>

ARGS:
    <name>    url name
    <val>     url value

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information
```
