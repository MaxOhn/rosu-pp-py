# v3.1.0 (2025-06-03)

Bumped to [`rosu-pp v3.1.0`](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v310-2025-06-03):
- Added the method `Beatmap.is_suspicious`. 
  Some maps are not meant to be played but just test the limits of osu! itself.
  Calculating attributes on these maps may be very expensive so it is
  recommended to always check an unknown map's suspicion before difficulty
  and/or performance calculation.
- Added the variant `HitResultPriority.Fastest`.
  It is highly recommended to specify this variant for performance calculation
  if only accuracy is given but no specific hitresults. Otherwise, generating
  hitresults that best match the given accuracy may be very slow.

## v3.0.0 (2025-04-07)

Updated all modes' difficulty and performance calculation. See osu!'s newspost for more info: <https://osu.ppy.sh/home/news/2025-03-06-performance-points-star-rating-updates>

rosu-pp changelog: <https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v300-2025-04-07>

- Removed properties:
  - `DifficultyAttributes.od`
  - `DifficultyAttributes.peak`

- Added properties:
  - `DifficultyAttributes.aim_difficult_slider_count` (osu!standard)
  - `DifficultyAttributes.reading` (osu!taiko)
  - `DifficultyAttributes.meh_hit_window` (osu!standard)
  - `PerformanceAttributes.speed_deviation` (osu!standard)
  - `Strains.reading` (osu!taiko)
  - `BeatmapAttributes.od_meh_hit_window` (osu!standard)

- Adjustments:
  - The property `DifficultyAttributes.great_hit_window` is no longer available for osu!mania but it is now available for osu!standard
  - The property `DifficultyAttributes.ok_hit_window` is now also available for osu!standard

## v2.0.1 (2024-12-05)

- Fixed the `lazer` argument not being passed to the calculation ([#12])
- Fixed the type error when passing mods to `Beatmap.convert` ([#13])
- The `settings` property for mods can now be `None` ([#13])

## v2.0.0 (2024-12-04)

Updated all modes' difficulty and performance calculation. See osu!'s newspost for more info: <https://osu.ppy.sh/home/news/2024-10-28-performance-points-star-rating-updates>

rosu-pp changelog: <https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v200-2024-12-03>

- __Breaking changes:__
  - Renamed some properties:
    - `BeatmapAttributes.od_hit_window` -> `od_great_hit_window`
    - `DifficultyAttributes.hit_window` -> `great_hit_window`
    - `BeatmapAttributes.ar_hitwindow` -> `ar_hit_window`

- __Additions:__
  - `Difficulty` and `Performance` now accept the kwarg `lazer: bool` (defaults to `true` if unspecified);
    Performance calculation for osu!standard and osu!mania now differs between lazer and stable so this is
    important to specify.
  - `Performance` now accepts the kwargs `large_tick_hits: int`, `small_tick_hits: int`, `slider_end_hits: int`;
    each of them being necessary to specify for osu!standard scores on lazer.
  - `ScoreState` now has the additional properties
    - `osu_large_tick_hits: int`
    - `osu_small_tick_hits: int`
    - `slider_end_hits: int`
  - The method `Beatmap.convert` now takes an optional second argument for gamemods
  - Added the property `BeatmapAttributes.od_ok_hit_window`
  - Added properties to `DifficultyAttributes`:
    - `aim_difficult_strain_count` (osu!standard)
    - `speed_difficult_strain_count` (osu!standard)
    - `mono_stamina_factor` (osu!taiko)
    - `n_hold_notes` (osu!mania)
    - `n_large_ticks` (osu!standard)
    - `ok_hit_window` (osu!taiko)
  - Added the property `PerformanceAttributes.estimated_unstable_rate` (osu!taiko)
  - Added the property `Strains.single_color_stamina` (osu!taiko)

## v1.1.0 (2024-07-12)

- Updated to [rosu-pp v1.1.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v110-2024-07-10)
- Mods can now be specified through more types than just `int` ([#9]). Instead, it has to coincide with the following alias defintions:
  ```py
  GameMods = Union[int, str, GameMod, List[Union[GameMod, str, int]]]
  GameMod = dict[str, Union[str, GameModSettings]]
  GameModSettings = dict[str, Union[bool, float, str]]
  ```
  That means, mods can be given either through their [(legacy) bitflags](https://github.com/ppy/osu-api/wiki#reference), a string for acronyms, a "GameMod" `dict`, or a sequence whose items are either a "GameMod" `dict`, a single acronym string, or bitflags for a single mod.

  A "GameMod" `dict` **must** have the item `'acronym': str` and an optional item `'settings': GameModSettings`.

## v1.0.1 (2024-05-05)

- `PerformanceAttributes`' field `pp_accuracy` was accidentally actually named `pp_acc`; now it's definitely `pp_accuracy` ([#7])

## v1.0.0 (2024-04-03)

- Updated to [rosu-pp v1.0.0](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v100-2024-04-02)
- Breaking changes ahead! There are now multiple different calculators:
  - `Difficulty` to calculate `DifficultyAttributes`, `Strains`, or create gradual calculators
  - `Performance` to calculate `PerformanceAttributes`
  - `BeatmapAttributesBuilder` to calculate `BeatmapAttributes`
  - `GradualDifficulty` to calculate `DifficultyAttributes` for each hitobject
  - `GradualPerformance` to calculate `PerformanceAttributes` for each hitresult

## v0.9.4 (2023-02-09)

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
[#7]: https://github.com/MaxOhn/rosu-pp-py/pull/7
[#9]: https://github.com/MaxOhn/rosu-pp-py/pull/9
[#12]: https://github.com/MaxOhn/rosu-pp-py/pull/12
[#13]: https://github.com/MaxOhn/rosu-pp-py/pull/13
