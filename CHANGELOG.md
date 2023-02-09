# v0.9.4 (2023-02-09)

- Updated to [rosu-pp v0.9.4](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v094-2023-02-09).

## v0.9.3 (2023-01-28)

- Updated to [rosu-pp v0.9.3](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v093-2023-01-28). Only includes some bug fixes.
- Fixed the hitobjects counts of map attributes on converted maps.

## v0.9.1 (2022-10-29)

- Updated to [rosu-pp v0.9.1](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v091-2022-10-26) including the big changes in [v0.9.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v090-2022-10-24)
- The binding interface is rewritten completely, see the readme.

## v0.8.0 (2022-08-02)
- Updated to [rosu-pp v0.8.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v080-2022-08-02)
- The calculation result now contains a `timePreempt` field for osu!standard and `greatHitWindow` for
osu!standard, osu!taiko, and osu!mania.
- Fixed map attributes when mods were interacting with custom clock rates

## v0.7.2 (2022-07-16)
- Apply mods before acc to fix pp values sometimes calculating incorrectly for osu!catch ([#3] - [@tsunyoku])

## v0.7.1 (2022-07-12)
- Updated to [rosu-pp v0.7.1](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v071-2022-07-12)

## v0.7.0 (2022-07-07)
- Removed the `GameMode` class again; use simple numbers instead (0/1/2/3)
- Updated to [rosu-pp v0.7.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v070-2022-07-06)
- Added `strains` method for `Calculator` which returns instances of `Strains`

## v0.6.0 (2022-07-05)
- Updated to [PyO3 v0.16](https://github.com/PyO3/pyo3/blob/main/CHANGELOG.md#0165---2022-05-15) from v0.15
- Updated to [rosu-pp v0.6](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v060-2022-07-05)
- Added the class `GameMode` to serve as enum e.g. `GameMode.Taiko`
- `ScoreParams` now have an additional field `mode: Optional[GameMode]` which can be used to convert to other modes

## v0.5.2 (2022-06-14)
- Bumped patch version of dependencies, including a [rosu-pp](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v052-2022-06-14) update

## v0.5.1 (2022-03-22)
- Updated to [rosu-pp v0.5.1](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md)
- `Calculator` now accepts the kwargs `ar`, `cs`, `hp`, and `od` to adjust the map's attributes
- Additionally to the kwargs, `Calculator` also has new methods `set_ar(v)`, `set_cs(v)`, `set_hp(v)`, and `set_od(v)`.
- `ScoreParams` has the additional field `clockRate` to specify a custom clock rate, providable as kwarg or through the setter
- `CalculateResult` now also includes a `clockRate` field

## v0.4.0 (2021-12-29)
- Initial release with rosu-pp v0.4.0

[@tsunyoku]: https://github.com/tsunyoku

[#3]: https://github.com/MaxOhn/rosu-pp-py/pull/3
