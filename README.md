# ed-system-search

*ed-system-search* is a tool to find interesting systems in Elite:
Dangerous.

It expects the populated systems dump to have been [downloaded from
EDSM](https://www.edsm.net/dump/systemsPopulated.json.gz)

## Usage

``` shell,script(name="help",expected_exit_code=0)
ed-system-search --help
```

``` text,verify(script_name="help",stream=stdout)
ed-system-search 1.0.16

Billie Thompson <billie@billiecodes.com>

Find interesting systems in the EDSM data dump

USAGE:
    ed-system-search [FLAGS] [OPTIONS] <edsm-path>

ARGS:
    <edsm-path>    This is the path to th EDSM dump in .json.gz format

FLAGS:
        --exclude-permit-locked     Exclude permit locked systems
        --exclude-player-faction    Exclude systems that contain a player faction
        --exclude-rare-commodity    Exclude systems that sell rare commodities
    -h, --help                      Print help information
    -V, --version                   Print version information

OPTIONS:
        --allegiance <MAJOR_FACTION>
            Filter by allegiance

        --government <GOVERNMENT_TYPE>
            Filter by government

        --max-distance-from-reference <LIGHT_SECONDS>
            Filter the systems that are further than this distance from the reference

        --max-distance-from-sol <LIGHT_SECONDS>
            Filter the systems that are further than this distance from sol

        --max-number-of-factions <COUNT>
            Filter the systems that have more factions than the number given

        --min-docks <COUNT>
            Filter the systems that are have less than the given number of docks

        --min-docks-large <COUNT>
            Filter the systems that are have less than the given number of docks with room for large
            ships

        --min-population <COUNT>
            Filter the systems that are have less than the given population

        --min-starports <COUNT>
            Filter the systems that are have less than the given number of starports

        --reference <SYSTEM_NAME>
            A reference system used by other filters
```

## Installing

See the [releases
page](https://github.com/PurpleBooth/ed-system-search/releases/latest)
we build for windows, linux and mac (all x86_64), alternatively use brew

``` shell,skip()
brew install PurpleBooth/repo/ed-system-search
```

## More Examples

You can see more examples in [Filters guide](./docs/filters.md)
