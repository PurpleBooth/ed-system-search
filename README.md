# ed-system-search

*ed-system-search* is a tool to find interesting systems in Elite:
Dangerous.

## Usage

``` shell,script(name="help",expected_exit_code=0)
ed-system-search --help
```

``` text,verify(script_name="help",stream=stdout)
ed-system-search 0.4.0
Billie Thompson <billie@billiecodes.com>
Find interesting systems in the EDSM data dump

USAGE:
    ed-system-search [OPTIONS] <edsm-path>

ARGS:
    <edsm-path>    This is the path to th EDSM dump

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

OPTIONS:
        --max-distance-from-sol <LIGHT_SECONDS>
            Filter the systems that are further than this distance from sol

        --min-docks <COUNT>
            Filter the systems that are have less than the given number of docks

        --min-docks-large <COUNT>
            Filter the systems that are have less than the given number of docks with room for large
            ships

```
