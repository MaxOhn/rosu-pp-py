# rosu-pp-py

Difficulty and performance calculation for all [osu!](https://osu.ppy.sh/) modes.

This is a python binding to the Rust library [rosu-pp](https://github.com/MaxOhn/rosu-pp) which was bootstrapped through [PyO3](https://github.com/PyO3/PyO3).
Since all the heavy lifting is done by Rust, rosu-pp-py comes with a very fast performance.
Check out rosu-pp's README for more info.

## How to use rosu-pp-py

The library exposes three classes: `Calculator`, `ScoreParams`, `CalculateResult`, and `Strains`.

1) The first step is to create a new `Calculator` instance by providing the constructor the path to a `.osu` beatmap file like so
```py
calculator = Calculator('/path/to/file.osu')
```
Optionally, you can also provide the kwargs `ar`, `cs`, `hp`, or `od` to adjust the map's attributes
or alternatively, after creating the calculator, you can call `set_ar(v)`, `set_cs(v)`, `set_hp(v)`, or `set_od(v)`.
```py
calculator = Calculator('/path/to/file.osu', ar = 10.0)
calculator.set_od(9.2)
```
2) Next, you need to create `ScoreParams`. It has the following fields:
```
mode: Optional[int],
    specify for scores on convert maps, default to the map's native mode
    available values are 0 for standard, 1 for taiko, 2 for catch, and 3 for mania
mods: Optional[int],
    bit value for mods, defaults to 0 (NM) see https://github.com/ppy/osu-api/wiki#mods
acc: Optional[float],
    if neither acc nor hitresults are specified, acc defaults to 100.0
n300: Optional[int],
    defaults to value based on acc
n100: Optional[int],
    defaults to value based on acc
n50: Optional[int],
    defaults to value based on acc
nMisses: Optional[int],
    defaults to 0
nKatu: Optional[int],
    only relevant for osu!ctb
combo: Optional[int],
    defaults to full combo
score: Optional[int],
    only relevant for osu!mania
passedObjects: Optional[int],
    only consider this many hit objects; useful for failed scores; defaults to all objects
clockRate: Optional[float]
    defaults to the mod's clock rate.
```
Note that all fields are optional. If nothing is specified, the parameters are equivalent to the parameters of the best possible NM score.
`ScoreParams` can be created either by calling the constructor without arguments and then set the fields manually like so
```py
params = ScoreParams()
params.acc = 98.76
```
or they can be created by passing kwargs to the constructor directly like so
```py
params = ScoreParams(acc = 98.76)
```
3) The last step is to provide the `ScoreParams` to the `Calculator` through the function `calculate`. This function takes one argument which must be either a single `ScoreParams` or an `Iterable[ScoreParams]`, i.e. anything that python can iterate over like a list, set, ...

## Example

```py
from rosu_pp_py import Calculator, ScoreParams

calculator = Calculator('./maps/1980365.osu')

params1 = ScoreParams(
    mods = 8 + 16,  # HDHR
    acc = 97.89,
    nMisses = 13,
    combo = 1388,
)
params2 = ScoreParams(mods = 24)

# provide params for a single score, returns a list with one element
[result] = calculator.calculate(params1)

# provide multiple params
results = calculator.calculate([params1, params2])

assert result == results[0]

print(f'PP: {results[0].pp}/{results[1].pp} | Stars: {results[1].stars}')
```

## Return object structure

The `Calculator::calculate` function will provide you a **list of `CalculateResult`**, one for each score you specified parameters for. `CalculateResult` contains the difficulty and performance attributes. Most of its attributes are optional based on the map's mode. In the following, O/T/C/M will denote for which mode the given attribute will be present:

```
mode: int
    Gamemode of the map, 0=O, 1=T, 2=C, 3=M. (O/T/C/M)
stars: float
    Star rating of the map. (O/T/C/M)
pp: float
    Performance points of the score. (O/T/C/M)
ppAcc: Optional[float]
    Accuracy based portion of the performance points. (O/T/M)
ppAim: Optional[float]
    Aim based portion of the performance points. (O)
ppFlashlight: Optional[float]
    Flashlight based portion of the performance points. (O)
ppSpeed: Optional[float]
    Speed based portion of the performance points. (O)
ppStrain: Optional[float]
    Strain based portion of the performance points. (T/M)
nFruits: Optional[int]
    The amount of fruits in the map. (C)
nDroplets: Optional[int]
    The amount of droplets in the map. (C)
nTinyDroplets: Optional[int]
    The amount of tiny droplets in the map. (C)
aimStrain: Optional[float]
    Aim based portion of the star rating. (O)
speedStrain: Optional[float]
    Speed based portion of the star rating. (O)
flashlightRating: Optional[float]
    Flashlight based portion of the star rating. (O)
sliderFactor: Optional[float]
    Nerf factor for sliders. (O)
ar: float
    Approach rate of the map. (O/T/C/M)
cs: float
    Circle size of the map. (O/T/C/M)
hp: float
    Health drain rate of the map. (O/T/C/M)
od: float
    Overall difficulty of the map. (O/T/C/M)
bpm: float
    Beats per minute of the map. (O/T/C/M)
clockRate: float
    Clock rate used in calculation i.e. 1.5 for DT, 0.75 for HT, 1.0 for NM or one that was specified (O/T/C/M)
nCircles: Optional[int]
    The amount of circles in the map. (O/T/M)
nSliders: Optional[int]
    The amount of sliders in the map. (O/T/M)
nSpinners: Optional[int]
    The amount of spinners in the map. (O/T/C)
maxCombo: Optional[int]
    The max combo of the map. (O/T/C)
```

## Calculating strains

If you want to plot the difficulty of a map over time, you can calculate the strain values.
The return type of `Calculator::strains` is an instance of the `Strains` class.
Its attributes depend on the map's game mode again and look as follows:
```
sectionLength: float
    The time in milliseconds between two strain values. (O/T/C/M)
aim: List[float]
    Strain values for the aim skill (O)
aimNoSliders: List[float]
    Strain values for the aim skill without sliders (O)
speed: List[float]
    Strain values for the speed skill (O)
flashlight: List[float]
    Strain values for the flashlight skill (O)
color: List[float]
    Strain values for the color skill (T)
rhythm: List[float]
    Strain values for the rhythm skill (T)
staminaLeft: List[float]
    Strain values for the left-stamina skill (T)
staminaRight: List[float]
    Strain values for the right-stamina skill (T)
strains: List[float]
    Strain values for the strain skill (M)
movement: List[float]
    Strain values for the movement skill (C)
```
Here's a small example
```py
from rosu_pp_py import Calculator

calculator = Calculator('./maps/1980365.osu')
strains = calculator.strains(8 + 16) # HDHR
for i,strain in enumerate(strains.aim):
    currTime = i * strains.sectionLength
    print(f'Aim strain at {currTime}ms: {strain}')
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
- [PyO3 documentation](https://pyo3.rs/v0.15.1/).
- [Python documentation](https://docs.python.org/3/).
