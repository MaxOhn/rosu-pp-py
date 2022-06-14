# v0.5.2 (2022-06-14)
- Bumped patch version of dependencies, including a [rosu-pp](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md#v052-2022-06-14) update

## v0.5.1 (2022-03-22)
- Updated to [rosu-pp v0.5.1](https://github.com/MaxOhn/rosu-pp/blob/main/CHANGELOG.md)
- `Calculator` now accepts the kwargs `ar`, `cs`, `hp`, and `od` to adjust the map's attributes
- Additionally to the kwargs, `Calculator` also has new methods `set_ar(v)`, `set_cs(v)`, `set_hp(v)`, and `set_od(v)`.
- `ScoreParams` has the additional field `clockRate` to specify a custom clock rate, providable as kwarg or through the setter
- `CalculateResult` now also includes a `clockRate` field

## v0.4.0 (2021-12-29)
- Initial release with rosu-pp v0.4.0