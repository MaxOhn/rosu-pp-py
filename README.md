# rosu-pp-py

Difficulty and performance calculation for all [osu!](https://osu.ppy.sh/) modes.

This is a python binding to the Rust library [rosu-pp](https://github.com/MaxOhn/rosu-pp) which was bootstrapped through [PyO3](https://github.com/PyO3/PyO3).
Since all the heavy lifting is done by Rust, rosu-pp-py comes with a very fast performance.
Check out rosu-pp's [README](https://github.com/MaxOhn/rosu-pp/blob/main/README.md) for more info.

## Exposed types

The library exposes the following classes:

- `Calculator`: Contains various parameters to calculate strains or map, difficulty, or performance attributes
- `Beatmap`: Contains a parsed beatmap
- [`BeatmapAttributes`](https://github.com/MaxOhn/rosu-pp-py/blob/81e1d6f28064b832661a4940a3896a2089f76b6b/rosu_pp_py.pyi#L199-L231): Contains various attributes about the map itself
- [`DifficultyAttributes`](https://github.com/MaxOhn/rosu-pp-py/blob/81e1d6f28064b832661a4940a3896a2089f76b6b/rosu_pp_py.pyi#L234-L284): Contains various attributes about the difficulty based on the mode
- [`PerformanceAttributes`](https://github.com/MaxOhn/rosu-pp-py/blob/81e1d6f28064b832661a4940a3896a2089f76b6b/rosu_pp_py.pyi#L287-L313): Contains various attributes about the performance and difficulty based on the mode
- [`Strains`](https://github.com/MaxOhn/rosu-pp-py/blob/81e1d6f28064b832661a4940a3896a2089f76b6b/rosu_pp_py.pyi#L316-L346): Contains strain values for each skill based on the mode

Additionally, the following error types are exposed:
- `ParseError`: Failed to parse a beatmap
- `KwargsError`: Invalid kwargs were provided

## How to use rosu-pp-py

1) The first step is to create a new `Beatmap` instance by providing appropriate kwargs.
Either of the kwargs `path`, `content`, or `bytes` **must** be given. The kwargs `ar`, `cs`, `hp`, and `od` are optional.
With the setters `set_ar`, `set_cs`, `set_hp`, and `set_od` you can specify custom attributes.
```py
map = Beatmap(path = "/path/to/file.osu", ar = 9.87)
map.set_od(1.23)

with open("/path/to/file.osu", "rb") as file:
    map = Beatmap(bytes = file.read())

with open("/path/to/file.osu") as file:
    map = Beatmap(content = file.read())
```

2) Next, you need to create an instance of `Calculator` by providing the appropriate kwargs again.
Any of the following kwargs are allowed: `mode`, `mods`, `acc`, `n_geki`, `n_katu`, `n300`, `n100`, `n50`, `n_misses`, `combo`, `passed_objects`, `clock_rate`, and `difficulty`.
Each of these also have a setter method e.g. `set_n_misses`.
```py
calc = Calculator(mode = 2, acc = 98.76)
calc.set_mods(8 + 64) # HDDT
```

3) The last step is to call any of the methods `map_attributes`, `difficulty`, `performance`, or `strains` on the calculator and provide them a `Beatmap`.

## Example

```py
from rosu_pp_py import Beatmap, Calculator

map = Beatmap(path = "./maps/100.osu")
calc = Calculator(mods = 8)

# Calculate an SS on HD
max_perf = calc.performance(map)

# The mods are still set to HD
calc.set_acc(99.11)
calc.set_n_misses(1)
calc.set_combo(200)

# A good way to speed up the calculation is to provide
# the difficulty attributes of a previous calculation
# so that they don't need to be recalculated.
# **Note** that this should only be done if neither
# the map, mode, mods, nor passed objects amount changed.
calc.set_difficulty(max_perf.difficulty)

curr_perf = calc.performance(map)
print(f'PP: {curr_perf.pp}/{max_perf.pp} | Stars: {max_perf.difficulty.stars}')

map_attrs = calc.map_attributes(map)
print(f'BPM: {map_attrs.bpm}')

strains = calc.strains(map)
print(f'Maximum aim strain: {max(strains.aim)}')
```

## Installing rosu-pp-py

Installing rosu-pp-py requires a [supported version of Python and Rust](https://github.com/PyO3/PyO3#usage).

Once [Python](https://www.python.org/downloads/) and [Rust](https://www.rust-lang.org/learn/get-started) and ready to go, you can install the project with pip:

```sh
$ pip install rosu-pp-py
```
or
```
$ pip install git+https://github.com/MaxOhn/rosu-pp-py
```

## Learn More
- [rosu-pp documentation](https://docs.rs/rosu-pp/latest/rosu_pp/)
- [Rust documentation](https://www.rust-lang.org).
- [PyO3 documentation](https://pyo3.rs/).
- [Python documentation](https://docs.python.org/3/).
