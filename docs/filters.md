# Filters

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

## Minimum number of docks

You can search by number of places you can dock a ship

``` shell,script(name="min-docks",expected_exit_code=0)
ed-system-search --min-docks 15 "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-docks",stream=stdout)
62 Ursae Majoris
Aditjargl
Adityan
Albarib
Alchita
Alpha Tucanae
Aluriates
Amenta
Aranbarahun
Arevakimos
Arikara
BD+65 1846
BD-19 3629A
Balmung
Beker
Beta Circini
Candecama
Charunder
Chujohimba
Gebel
Gliese 867.1
Gliese 868
Gliese 900.1
Grebegus
HIP 115929
HIP 21559
HIP 21778
HIP 22281
HIP 38129
HIP 41308
HIP 78983
HIP 9316
HR 1475
Inara
Jang Di
Kakmburra
Kalb
Kamocan
Kotilekui
LHS 1914
LHS 2310
LHS 274
LTT 11244
Laksak
Lalande 10797
Laukese
Mula Wendes
Mullag
NLTT 9447
Nahuatl
Nyx
Oduduro
Peraesii
Pini
Plutarch
Puelchana
Rind
Scirth
Sol
T'iensei
Tinigua
Uchaluroja
Urcia
Yab Yum
Zeta Microscopii
```

if it's not a number it'll fail

``` shell,script(name="min-docks-err",expected_exit_code=1)
ed-system-search --min-docks banana "$EDSM_GZ_PATH"
```

``` text,verify(script_name="min-docks-err",stream=stderr)
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
