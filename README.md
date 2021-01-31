#### gogo

A mnemonic terminal url opener.

#### Installation

```
$ cargo install gogo
```

#### Usage

```
$ gogo add github "github.com"
$ gogo open github
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
