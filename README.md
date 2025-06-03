# rosu-pp-py

Library to calculate difficulty and performance attributes for all [osu!] modes.

This is a python binding to the [Rust] library [rosu-pp] which was bootstrapped through [PyO3].
As such, its performance is much faster than a native python library.

## Usage

The library exposes multiple classes:
- [`Beatmap`](https://github.com/MaxOhn/rosu-pp-py/blob/e3a15a8364be630176d29459cd4caa3ef6775561/rosu_pp_py.pyi#L31-L102): Parsed `.osu` file
- [`GameMode`](https://github.com/MaxOhn/rosu-pp-py/blob/5c7c2f90dd904d01ec163a9a26b2efcb525db13a/rosu_pp_py.pyi#L12-L20)
- Calculators
  - [`Difficulty`](https://github.com/MaxOhn/rosu-pp-py/blob/5c7c2f90dd904d01ec163a9a26b2efcb525db13a/rosu_pp_py.pyi#L110-L274): Class to calculate difficulty attributes, strains, or create gradual calculators
  - [`Performance`](https://github.com/MaxOhn/rosu-pp-py/blob/5c7c2f90dd904d01ec163a9a26b2efcb525db13a/rosu_pp_py.pyi#L276-L506): Performance attributes calculator
  - [`GradualDifficulty`](https://github.com/MaxOhn/rosu-pp-py/blob/5c7c2f90dd904d01ec163a9a26b2efcb525db13a/rosu_pp_py.pyi#L508-L532): Calculator to calculate difficulty attributes after each hitobject
  - [`GradualPerformance`](https://github.com/MaxOhn/rosu-pp-py/blob/5c7c2f90dd904d01ec163a9a26b2efcb525db13a/rosu_pp_py.pyi#L534-L560): Calculator to calculator performance attributes after each hitresult
  - [`BeatmapAttributesBuilder`](https://github.com/MaxOhn/rosu-pp-py/blob/5c7c2f90dd904d01ec163a9a26b2efcb525db13a/rosu_pp_py.pyi#L562-L693): Beatmap attributes calculator
- Results
  - [`DifficultyAttributes`](https://github.com/MaxOhn/rosu-pp-py/blob/15470e5d75615cf58b90cae739a67794e4cca851/rosu_pp_py.pyi#L769-L1019)
  - [`Strains`](https://github.com/MaxOhn/rosu-pp-py/blob/15470e5d75615cf58b90cae739a67794e4cca851/rosu_pp_py.pyi#L1110-L1192): Strain values of a difficulty calculation, suitable to plot difficulty over time
  - [`PerformanceAttributes`](https://github.com/MaxOhn/rosu-pp-py/blob/15470e5d75615cf58b90cae739a67794e4cca851/rosu_pp_py.pyi#L1021-L1108)
  - [`BeatmapAttributes`](https://github.com/MaxOhn/rosu-pp-py/blob/15470e5d75615cf58b90cae739a67794e4cca851/rosu_pp_py.pyi#L1194-L1240)
- [`HitResultPriority`](https://github.com/MaxOhn/rosu-pp-py/blob/e3a15a8364be630176d29459cd4caa3ef6775561/rosu_pp_py.pyi#L22-L29): Passed to `Performance`, decides whether specified accuracy should be realized through good or bad hitresults
- [`ScoreState`](https://github.com/MaxOhn/rosu-pp-py/blob/5c7c2f90dd904d01ec163a9a26b2efcb525db13a/rosu_pp_py.pyi#L695-L767): Hitresults and max combo of a score, found in `PerformanceAttributes` and passed to gradual calculators

## Example

### Calculating performance

```py
import rosu_pp_py as rosu

# either `path`, `bytes`, or `content` must be specified when parsing a map
map = rosu.Beatmap(path = "/path/to/file.osu")

# Optionally convert to a specific mode for optionally given mods
map.convert(rosu.GameMode.Mania, "6K")

# Whereas osu! simply times out on malicious maps, rosu-pp does not. To
# prevent potential performance/memory issues, it is recommended to check
# beforehand whether a map is too suspicious for further calculation.
if map.is_suspicious():
    exit()

perf = rosu.Performance(
    # various kwargs available
    accuracy = 98.76,
    lazer = False, # defaults to True if not specified
    misses = 2,
    combo = 700,
    # If only accuracy is given but no specific hitresults, it is recommended
    # to generate hitresults via `HitResultPriority.Fastest`. Otherwise,
    # finding the best hitresults can be very slow.
    hitresult_priority=rosu.HitResultPriority.Fastest,
)

# Each kwarg can also be specified afterwards through setters
perf.set_accuracy(99.11) # override previously specified accuracy
perf.set_mods(8 + 64)    # HDDT
perf.set_clock_rate(1.4)

# Second argument of map attributes specifies whether mods still need to be accounted for
# `True`: mods already considered; `False`: value should still be adjusted
perf.set_ar(10.5, True)
perf.set_od(5, False)

# Calculate for the map
attrs = perf.calculate(map)

# Note that calculating via map will have to calculate difficulty attributes
# internally which is fairly expensive. To speed it up, you can also pass in
# previously calculated attributes, but be sure they were calculated for the
# same difficulty settings like mods, clock rate, custom map attributes, ...

perf.set_accuracy(100)
perf.set_misses(None)
perf.set_combo(None)

# Calculate a new set of attributes by re-using previous attributes instead of the map
max_attrs = perf.calculate(attrs)

print(f'PP: {attrs.pp}/{max_attrs.pp} | Stars: {max_attrs.difficulty.stars}')
```

### Gradual calculation

```py
import rosu_pp_py as rosu

# Parsing the map, this time through the `content` kwarg
with open("/path/to/file.osu") as file:
    map = rosu.Beatmap(content = file.read())

# Specifying some difficulty parameters
diff = rosu.Difficulty(
    mods = 16 + 1024, # HRFL
    clock_rate = 1.1,
    ar = 10.2,
    ar_with_mods = True,
)

# Gradually calculating *difficulty* attributes
gradual_diff = diff.gradual_difficulty(map)

for i, attrs in enumerate(gradual_diff, 1):
    print(f'Stars after {i} hitobjects: {attrs.stars}')

# Gradually calculating *performance* attributes
gradual_perf = diff.gradual_performance(map)
i = 1

while True:
    state = rosu.ScoreState(
        max_combo = i,
        n300 = i,
        n100 = 0,
        # ...
    )

    attrs = gradual_perf.next(state)

    if attrs is None:
        # All hitobjects have been processed
        break

    print(f'PP: {attrs.pp}')
    i += 1
```

## Mods

Wherever mods are specified, their type should coincide with the following alias definition:
```py
GameMods = Union[int, str, GameMod, List[Union[GameMod, str, int]]]
GameMod = dict[str, Union[str, GameModSettings]]
GameModSettings = dict[str, Union[bool, float, str]]
```

That means, mods can be given either through their [(legacy) bitflags](https://github.com/ppy/osu-api/wiki#reference),
a string for acronyms, a "GameMod" `dict`, or a sequence whose items are either
a "GameMod" `dict`, a single acronym string, or bitflags for a single mod.

A "GameMod" `dict` **must** have the item `'acronym': str` and an optional item `'settings': GameModSettings`.

Some examples for valid mods look as follows:

```py
mods = 8 + 64              # Hidden, DoubleTime
mods = "hRNcWIez"          # HardRock, Nightcore, Wiggle, Easy
mods = { 'acronym': "FI" } # FadeIn
mods = [
    1024,
    'nf',
    {
        'acronym': "AC",
        'settings': {
            'minimum_accuracy': 95,
            'restart': True
        }
    }
] # Flashlight, NoFail, AccuracyChallenge

import json
mods_json = '[{"acronym": "TC"}, {"acronym": "HT", "settings": {"speed_change": 0.6}}]'
mods = json.loads(mods_json) # Traceable, HalfTime
```

## Installing rosu-pp-py

Installing rosu-pp-py requires a [supported version of Python and Rust](https://github.com/PyO3/PyO3#usage).

If a [pre-built wheel](https://pypi.org/project/rosu-pp-py/#files) is available for your architecture, you can even skip the Rust part.

Once [Python] and (optionally) [Rust](https://www.rust-lang.org/learn/get-started) are ready to go, you can install the project with pip:

```sh
$ pip install rosu-pp-py
```

or

```
$ pip install git+https://github.com/MaxOhn/rosu-pp-py
```

## Learn More
- [rosu-pp]
- [Rust]
- [PyO3]

[osu!]: https://osu.ppy.sh/home
[Rust]: https://www.rust-lang.org
[rosu-pp]: https://github.com/MaxOhn/rosu-pp
[PyO3]: https://github.com/PyO3/pyo3
[Python]: https://www.python.org/downloads/