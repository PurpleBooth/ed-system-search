# Filters

You can search by number of places you can dock on a large ship

``` shell,script(name="min-docks-large",expected_exit_code=0)
ed-system-search --min-docks-large 6 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-docks-large",stream=stdout)
Alioth
Sol
```

if it's not a number it'll fail

``` shell,script(name="min-docks-large",expected_exit_code=1)
ed-system-search --min-docks-large banana "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-docks-large",stream=stderr)
Error: InvalidCount(ParseIntError { kind: InvalidDigit })
```
