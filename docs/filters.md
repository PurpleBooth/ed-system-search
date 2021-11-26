# Filters

## Exclude rare commodity systems

Exclude rare commodity systems

``` shell,script(name="exclude-rare-commodity",expected_exit_code=0)
ed-system-search --exclude-rare-commodity --max-distance-from-sol 10 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="exclude-rare-commodity",stream=stdout)
Barnard's Star
Duamta
Luhman 16
Ross 154
Sirius
Sol
WISE 0855-0714
Wolf 359
```

## Exclude permit locked systems

Exclude permit locked systems

``` shell,script(name="exclude-permit-locked",expected_exit_code=0)
ed-system-search --exclude-permit-locked --max-distance-from-sol 10 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="exclude-permit-locked",stream=stdout)
Alpha Centauri
Barnard's Star
Duamta
Luhman 16
Ross 154
WISE 0855-0714
Wolf 359
```

## Exclude systems with player faction

Exclude systems with player faction

``` shell,script(name="exclude-player-faction",expected_exit_code=0)
ed-system-search --exclude-player-faction --max-distance-from-sol 10  "$EDSM_GZ_PATH"
```

``` text,verify(script_name="exclude-player-faction",stream=stdout)
Ross 154
Sol
```

## Minimum number of large docks

You can search by number of places you can dock on a large ship

``` shell,script(name="min-docks-large",expected_exit_code=0)
ed-system-search --min-docks-large 6 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-docks-large",stream=stdout)
Alioth
Sol
```

if it's not a number it'll fail

``` shell,script(name="min-docks-large-err",expected_exit_code=1)
ed-system-search --min-docks-large banana "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-docks-large-err",stream=stderr)
Error: Cli(InvalidCount(ParseIntError { kind: InvalidDigit }))
```

## Minimum number of starports

Same as large without asteroid stations

``` shell,script(name="min-starports",expected_exit_code=0)
ed-system-search --min-starports 6 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-starports",stream=stdout)
Alioth
Sol
```

if it's not a number it'll fail

``` shell,script(name="min-starports-err",expected_exit_code=1)
ed-system-search --min-starports banana "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-starports-err",stream=stderr)
Error: Cli(InvalidCount(ParseIntError { kind: InvalidDigit }))
```

## Minimum number of docks

You can search by number of places you can dock a ship

``` shell,script(name="min-docks",expected_exit_code=0)
ed-system-search --min-docks 15 --max-distance-from-sol 10 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-docks",stream=stdout)
Sol
```

if it's not a number it'll fail

``` shell,script(name="min-docks-err",expected_exit_code=1)
ed-system-search --min-docks banana "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-docks-err",stream=stderr)
Error: Cli(InvalidCount(ParseIntError { kind: InvalidDigit }))
```

## Minimum population

You can search by the population of the system

``` shell,script(name="min-population",expected_exit_code=0)
ed-system-search --min-population 25000000000 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-population",stream=stdout)
Blatrimpe
G 203-47
Lave
```

if it's not a number it'll fail

``` shell,script(name="min-population-err",expected_exit_code=1)
ed-system-search --min-population banana "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-population-err",stream=stderr)
Error: Cli(InvalidCount(ParseIntError { kind: InvalidDigit }))
```

## Allegiance

You can search by the population of the system

``` shell,script(name="allegiance",expected_exit_code=0)
ed-system-search --max-distance-from-sol 10 --allegiance Federation -- "$EDSM_GZ_PATH"
```

``` text,verify(script_name="allegiance",stream=stdout)
Duamta
Sol
```

## Government

You can search by the population of the system

``` shell,script(name="government",expected_exit_code=0)
ed-system-search --max-distance-from-sol 10 --government Democracy -- "$EDSM_GZ_PATH"
```

``` text,verify(script_name="government",stream=stdout)
Ross 154
Sol
```

## Max number of factions

You can skip systems with more than a number of factions

``` shell,script(name="max-number-of-factions",expected_exit_code=0)
ed-system-search --max-number-of-factions 1 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="max-number-of-factions",stream=stdout)
4 Sextantis
Bleae Thua KY-L c7-12
Bleae Thua WD-M b49-1
Bleia Dryiae EE-E d13-178
Bleia Dryiae HK-Y c17-9
Blua Eaec RD-Z d1-1228
Blua Eaec US-Z b46-4
Blua Eaec WW-E c14-1293
Boelts UB-P b24-98
Boelts YK-P c21-5
Boelts ZN-Y b5-69
Byeia Eurk IE-L b49-4
Byeia Eurk OC-I b37-13
Capricorni Sector KC-V c2-13
Clooku AA-Q b37-41
Clooku QA-E c28-713
Clooku VJ-E b16-27
Col 285 Sector MH-V d2-50
Col 359 Sector UM-T c4-6
Droju OH-T a99-0
Dryio Flyuae KV-P b8-112
Dryooe Flyou NQ-G b27-103
Dryooe Flyou WB-T b47-10
Eoch Flyuae PL-D c138
Eoch Flyuae QK-E d12-2118
Eoch Flyuae ZU-Y b17-16
Eol Prou GE-A c1-291
Flyiedge JE-Z b46-9
Flyiedge VN-W c4-51
Gria Drye IR-F a38-10
HIP 58832
HIP 89396
HIP 90024
Hyades Sector RI-T c3-11
IC 1287 Sector RO-Q b5-1
Lysoosms YS-U d2-61
Mbooni
NGC 6530 Sector ZE-X b2-0
Nyeajaae NB-Q b52-14
Nyeajaae SC-B b33-7
Plaa Aescs QD-T c3-28
Pru Euq WO-D b53-8
Prua Phoe MI-B b17-5
Prua Phoe PI-I b55-3
Prua Phoe VF-M d8-1046
Skaudai GM-S b35-5
Skaude DR-A c2-1
Skaude ZK-X e1-203
Smojue IY-Q b32-1
Smojue PZ-R c4-5
Snake Sector OD-S b4-2
Stuelou UT-E b17-51
Stuelou VV-X c17-395
Trifid Sector GW-W d1-220
```

if it's not a number it'll fail

``` shell,script(name="max-number-of-factions-err",expected_exit_code=1)
ed-system-search --max-number-of-factions banana "$EDSM_GZ_PATH"
```

``` text,verify(script_name="max-number-of-factions-err",stream=stderr)
Error: Cli(InvalidCount(ParseIntError { kind: InvalidDigit }))
```

## Max distance from sol system

You can search by distance from sol

``` shell,script(name="max-distance-from-sol",expected_exit_code=0)
ed-system-search --max-distance-from-sol 10 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="max-distance-from-sol",stream=stdout)
Alpha Centauri
Barnard's Star
Duamta
Luhman 16
Ross 154
Sirius
Sol
WISE 0855-0714
Wolf 359
```

if it's not a number it'll fail

``` shell,script(name="max-distance-from-sol-err",expected_exit_code=1)
ed-system-search --max-distance-from-sol banana "$EDSM_GZ_PATH"
```

``` text,verify(script_name="max-distance-from-sol-err",stream=stderr)
Error: Cli(InvalidFloat(ParseFloatError { kind: Invalid }))
```

## Max distance from reference system

You can search by distance from reference system

``` shell,script(name="max-distance-from-reference",expected_exit_code=0)
ed-system-search --reference=Sol --max-distance-from-reference 10 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="max-distance-from-reference",stream=stdout)
Alpha Centauri
Barnard's Star
Duamta
Luhman 16
Ross 154
Sirius
Sol
WISE 0855-0714
Wolf 359
```

if it's not a number it'll fail

``` shell,script(name="max-distance-from-reference-err",expected_exit_code=1)
ed-system-search --reference=Sol --max-distance-from-reference banana "$EDSM_GZ_PATH"
```

``` text,verify(script_name="max-distance-from-reference-err",stream=stderr)
Error: Cli(InvalidFloat(ParseFloatError { kind: Invalid }))
```

If the reference isn't found it'll fail

``` shell,script(name="missing-reference-err",expected_exit_code=1)
ed-system-search --reference=Missing --max-distance-from-reference 10 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="missing-reference-err",stream=stderr)
Error: Cli(SystemNotFound("Missing"))
```
