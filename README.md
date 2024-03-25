# rosu-pp-py

Library to calculate difficulty and performance attributes for all [osu!] modes.

This is a python binding to the [Rust] library [rosu-pp] which was bootstrapped through [PyO3].
As such, its performance is much faster than a native python library.

## Usage

The library exposes multiple classes:
- [`Beatmap`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L23-L105): Parsed `.osu` file
- [`GameMode`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L5-L13)
- Calculators
  - [`Difficulty`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L107-L255): Class to calculate difficulty attributes, strains, or create gradual calculators
  - [`Performance`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L257-L443): Performance attributes calculator
  - [`GradualDifficulty`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L445-L469): Calculator to calculate difficulty attributes after each hitobject
  - [`GradualPerformance`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L471-L497): Calculator to calculator performance attributes after each hitresult
  - [`BeatmapAttributesBuilder`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L499-L621): Beatmap attributes calculator
- Results
  - [`DifficultyAttributes`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L670-L857)
  - [`Strains`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L932-L1002): Strain values of a difficulty calculation, suitable to plot difficulty over time
  - [`PerformanceAttributes`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L859-L930)
  - [`BeatmapAttributes`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L1004-L1034)
- [`HitResultPriority`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L15-L21): Passed to `Performance`, decides whether specified accuracy should be realized through good or bad hitresults
- [`ScoreState`](https://github.com/MaxOhn/rosu-pp-py/blob/a34cc3a1c239da3174986e78c849f5c2a23a8746/rosu_pp_py.pyi#L623-L668): Hitresults and max combo of a score, found in `PerformanceAttributes` and passed to gradual calculators

## Example

### Calculating performance

```py
import rosu_pp_py as rosu

map = rosu.Beatmap(
    path = "/path/to/file.osu", # either `path`, `bytes`, or `content` must be specified
    mode = rosu.GameMode.Mania, # optionally convert to a specified mode
)

# Or convert afterwards like `map.convert(rosu.GameMode.Taiko)`

perf = rosu.Performance(
    # various kwargs available
    accuracy = 98.76,
    misses = 2,
    combo = 700,
    hitresult_priority = rosu.HitResultPriority.WorstCase, # favors bad hitresults
)

# Each kwarg can also be specified afterwards through setters
perf.set_accuracy(99.11) # override previously specified accuracy
perf.set_mods(8 + 64)    # HDDT
perf.set_clock_rate(1.4)

# 2nd arg of map attributes specifies whether mods still need to be accounted for
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

## Installing rosu-pp-py

Installing rosu-pp-py requires a [supported version of Python and Rust](https://github.com/PyO3/PyO3#usage).

Once [Python] and [Rust](https://www.rust-lang.org/learn/get-started) are ready to go, you can install the project with pip:

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