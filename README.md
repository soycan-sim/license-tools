# license-tools

Some useful command line tools for generating and dealing with software licenses.

# Usage

```
license-tools 0.0.1
Command line tools for generating and dealing with software licenses

USAGE:
    license-tools [SUBCOMMAND]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information

SUBCOMMANDS:
    gen     Generate license file(s)
    help    Print this message or the help of the given subcommand(s)
```

```
license-tools-gen 
Generate license file(s)

USAGE:
    license-tools gen [OPTIONS] <LICENSE>

ARGS:
    <LICENSE>    SPDX license identifier or expression

OPTIONS:
        --ascii                    Replace unicode characters with ascii
        --copyright <COPYRIGHT>    Set copyright holder. Defaults to 'user.name' from git config [default: your name]
        --format <FORMAT>          Format to output. Possible options: 'plain', 'markdown', 'html' [default: plain]
    -h, --help                     Print help information
        --no-copyright             Don't add a copyright notice
        --out <OUT>                Path or comma-separated list of paths to output to [default: LICENSE]
        --year <YEAR>              Set copyright year. Defaults to current year [default: current year]
```

# License

license-tools is dual-licensed under Apache 2.0 and MIT terms.