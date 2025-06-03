from enum import Enum
from typing import List, Mapping, Optional, Union
from collections.abc import Iterator

GameMods = Union[int, str, GameMod, List[Union[GameMod, str, int]]]
GameMod = Mapping[str, Union[str, Optional[GameModSettings]]]
"""
Must contain item `'acronym': str` and optionally `'settings': GameModSettings`
"""
GameModSettings = Mapping[str, Union[bool, float, str]]

class GameMode(Enum):
    """
    Enum for a beatmap's gamemode
    """

    Osu = 0
    Taiko = 1
    Catch = 2
    Mania = 3

class HitResultPriority(Enum):
    """
    While generating remaining hitresults, decide how they should be distributed.
    """

    BestCase = 0
    WorstCase = 1
    Fastest = 2

class Beatmap:
    """
    Class containing all beatmap data relevant for difficulty and performance calculation

    The kwargs must include any of the following:
        `'path': str`
            The path to a .osu file
        `'content': Union[str, bytearray]`
            The content of a .osu file as string or bytes
        `'bytes': bytearray`
            The content of a .osu file as bytes

    ## Raises

    Throws an exception if the map could not be parsed or the map's mode cannot be converted to
    the specified mode
    """

    def __init__(self, **kwargs) -> None: ...
    def convert(self, mode: GameMode, mods: Optional[GameMods]) -> None:
        """
        Convert the beatmap to the specified mode

        ## Raises

        Throws an exception if conversion fails or mods are invalid
        """

    def is_suspicious(self) -> bool:
        """
        Check whether hitobjects appear too suspicious for further calculation.

        Sometimes a beatmap isn't created for gameplay but rather to test
        the limits of osu! itself. Difficulty- and/or performance calculation
        should likely be avoided on these maps due to potential performance
        issues.
        """

    @property
    def bpm(self) -> float: ...
    @property
    def version(self) -> int: ...
    @property
    def is_convert(self) -> bool: ...
    @property
    def stack_leniency(self) -> float: ...
    @property
    def ar(self) -> float: ...
    @property
    def cs(self) -> float: ...
    @property
    def hp(self) -> float: ...
    @property
    def od(self) -> float: ...
    @property
    def slider_multiplier(self) -> float: ...
    @property
    def slider_tick_rate(self) -> float: ...
    @property
    def mode(self) -> GameMode: ...
    @property
    def n_breaks(self) -> int: ...
    @property
    def n_objects(self) -> int: ...
    @property
    def n_circles(self) -> int: ...
    @property
    def n_sliders(self) -> int: ...
    @property
    def n_spinners(self) -> int: ...
    @property
    def n_holds(self) -> int: ...

class Difficulty:
    """
    Builder for a difficulty calculation

    The kwargs may include any of the following:
        `'mods': GameMods`
            Specify mods.

            Relevant type aliases:
                `GameMods = Union[int, str, GameMod, List[Union[GameMod, str, int]]]`

                `GameMod = dict[str, Union[str, GameModSettings]]`
                    `GameMod` *must* have an item `'acronym': str` and an optional
                    item `'settings': GameModSettings`

                `GameModSettings = dict[str, Union[bool, float, str]]`

            See https://github.com/ppy/osu-api/wiki#mods
        `'clock_rate': float`
            Adjust the clock rate used in the calculation.

            If none is specified, it will take the clock rate based on the mods
            i.e. 1.5 for DT, 0.75 for HT and 1.0 otherwise.

            Clamped between 0.01 and 100.
        `'ar': float`
            Override a beatmap's set AR.

            Only relevant for osu! and osu!catch.

            Clamped between -20 and 20.
        `'ar_with_mods': bool`
            Determines if the given AR value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'cs': float`
            Override a beatmap's set CS.

            Only relevant for osu! and osu!catch.

            Clamped between -20 and 20.
        `'cs_with_mods': bool`
            Determines if the given CS value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'hp': float`
            Override a beatmap's set HP.

            Clamped between -20 and 20.
        `'hp_with_mods': bool`
            Determines if the given HP value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'od': float`
            Override a beatmap's set OD.

            Clamped between -20 and 20.
        `'od_with_mods': bool`
            Determines if the given OD value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'passed_objects': int`
            Amount of passed objects for partial plays, e.g. a fail.

            If you want to calculate the difficulty after every few objects,
            instead of using `Difficulty` multiple times with different
            `passed_objects`, you should use `GradualDifficulty`.
        `'hardrock_offsets': bool`
            Adjust patterns as if the HR mod is enabled.

            Only relevant for osu!catch.
        `'lazer': bool`
            Whether the calculated attributes belong to an osu!lazer or
            osu!stable score.

            Defaults to `true`.
    """

    def __init__(self, **kwargs) -> None: ...
    def calculate(self, map: Beatmap) -> DifficultyAttributes:
        """
        Perform the difficulty calculation
        """

    def strains(self, map: Beatmap) -> Strains:
        """
        Perform the difficulty calculation but instead of evaluating strain
        values, return them as is.

        Suitable to plot the difficulty over time.
        """

    def performance(self) -> Performance:
        """
        Use the current difficulty settings to create a performance calculator
        """

    def gradual_difficulty(self, map: Beatmap) -> GradualDifficulty:
        """
        Returns a gradual difficulty calculator for the current difficulty settings
        """

    def gradual_performance(self, map: Beatmap) -> GradualPerformance:
        """
        Returns a gradual performance calculator for the current difficulty settings
        """

    def set_mods(self, mods: Optional[GameMods]) -> None: ...
    def set_clock_rate(self, clock_rate: Optional[float]) -> None: ...
    def set_ar(self, ar: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set AR.

        Only relevant for osu! and osu!catch.

        Clamped between -20 and 20.

        `with_mods` determines if the given AR value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_cs(self, cs: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set CS.

        Only relevant for osu! and osu!catch.

        Clamped between -20 and 20.

        `with_mods` determines if the given CS value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_hp(self, hp: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set HP.

        Clamped between -20 and 20.

        `with_mods` determines if the given HP value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_od(self, od: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set OD.

        Clamped between -20 and 20.

        `with_mods` determines if the given OD value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_passed_objects(self, passed_objects: Optional[int]) -> None: ...
    def set_hardrock_offsets(self, hardrock_offsets: Optional[bool]) -> None: ...
    def set_lazer(self, lazer: Optional[bool]) -> None: ...

class Performance:
    """
    Builder for a performance calculation

    The kwargs may include any of the following:
        `'mods': GameMods`
            Specify mods.

            Relevant type aliases:
                `GameMods = Union[int, str, GameMod, List[Union[GameMod, str, int]]]`

                `GameMod = dict[str, Union[str, GameModSettings]]`
                    `GameMod` *must* have an item `'acronym': str` and an optional
                    item `'settings': GameModSettings`

                `GameModSettings = dict[str, Union[bool, float, str]]`

            See https://github.com/ppy/osu-api/wiki#mods
        `'clock_rate': float`
            Adjust the clock rate used in the calculation.

            If none is specified, it will take the clock rate based on the mods
            i.e. 1.5 for DT, 0.75 for HT and 1.0 otherwise.

            Clamped between 0.01 and 100.
        `'ar': float`
            Override a beatmap's set AR.

            Only relevant for osu! and osu!catch.

            Clamped between -20 and 20.
        `'ar_with_mods': bool`
            Determines if the given AR value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'cs': float`
            Override a beatmap's set CS.

            Only relevant for osu! and osu!catch.

            Clamped between -20 and 20.
        `'cs_with_mods': bool`
            Determines if the given CS value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'hp': float`
            Override a beatmap's set HP.

            Clamped between -20 and 20.
        `'hp_with_mods': bool`
            Determines if the given HP value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'od': float`
            Override a beatmap's set OD.

            Clamped between -20 and 20.
        `'od_with_mods': bool`
            Determines if the given OD value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'passed_objects': int`
            Amount of passed objects for partial plays, e.g. a fail.

            If you want to calculate the difficulty after every few objects,
            instead of using `Difficulty` multiple times with different
            `passed_objects`, you should use `GradualDifficulty`.
        `'hardrock_offsets': bool`
            Adjust patterns as if the HR mod is enabled.

            Only relevant for osu!catch.
        `'lazer': bool`
            Whether the calculated attributes belong to an osu!lazer or
            osu!stable score.

            Defaults to `true`.
        `'accuracy': float`
            Set the accuracy between `0.0` and `100.0`.
        `'combo': int`
            Specify the max combo of the play.

            Irrelevant for osu!mania.
        `'large_tick_hits': int`
            The amount of "large tick" hits.

            Only relevant for osu!standard.

            The meaning depends on the kind of score:
            - if set on osu!stable, this value is irrelevant and can be `0`
            - if set on osu!lazer *without* `CL`, this value is the amount of hit
              slider ticks and repeats
            - if set on osu!lazer *with* `CL`, this value is the amount of hit
              slider heads, ticks, and repeats
        `'small_tick_hits': int`
            The amount of "small tick" hits.

            These are essentially the slider end hits for lazer scores without
            slider accuracy.

            Only relevant for osu!standard.
        `'slider_end_hits': int`
            The amount of slider end hits.

            Only relevant for osu!standard in lazer.
        `'n_geki': int`
            Specify the amount of gekis of a play.

            Only relevant for osu!mania for which it repesents the amount of n320.
        `'n_katu': int`
            Specify the amount of katus of a play.

            Only relevant for osu!catch for which it represents the amount of tiny
            droplet misses and osu!mania for which it repesents the amount of n200.
        `'n300': int`
            Specify the amount of 300s of a play.
        `'n100': int`
            Specify the amount of 100s of a play.
        `'n50': int`
            Specify the amount of 50s of a play.

            Irrelevant for osu!taiko.
        `'misses': int`
            Specify the amount of misses of a play.
        `'hitresult_priority': HitResultPriority`
            Specify how hitresults should be generated.

            Defaults to `HitResultPriority.BestCase`.
    """

    def __init__(self, **kwargs) -> None: ...
    def calculate(
        self, arg: Union[DifficultyAttributes, PerformanceAttributes, Beatmap]
    ) -> PerformanceAttributes:
        """
        Calculate performance attributes.

        If a beatmap is passed as argument, difficulty attributes will have to
        be calculated internally which is a comparably expensive task. Hence,
        passing previously calculated attributes should be prefered whenever
        available.

        However, be careful that the passed attributes have been calculated
        for the same difficulty settings like mods, clock rate, beatmap,
        custom ar, ... otherwise the final attributes will be incorrect.
        """

    def difficulty(self) -> Difficulty:
        """
        Use the current difficulty settings to create a difficulty calculator
        """

    def set_mods(self, mods: Optional[GameMods]) -> None: ...
    def set_clock_rate(self, clock_rate: Optional[float]) -> None: ...
    def set_ar(self, ar: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set AR.

        Only relevant for osu! and osu!catch.

        Clamped between -20 and 20.

        `with_mods` determines if the given AR value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_cs(self, cs: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set CS.

        Only relevant for osu! and osu!catch.

        Clamped between -20 and 20.

        `with_mods` determines if the given CS value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_hp(self, hp: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set HP.

        Clamped between -20 and 20.

        `with_mods` determines if the given HP value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_od(self, od: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set OD.

        Clamped between -20 and 20.

        `with_mods` determines if the given OD value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_passed_objects(self, passed_objects: Optional[int]) -> None: ...
    def set_hardrock_offsets(self, hardrock_offsets: Optional[bool]) -> None: ...
    def set_lazer(self, lazer: Optional[bool]) -> None: ...
    def set_accuracy(self, accuracy: Optional[float]) -> None: ...
    def set_combo(self, combo: Optional[int]) -> None: ...
    def set_large_tick_hits(self, n_large_ticks: Optional[int]) -> None: ...
    def set_small_tick_hits(self, n_large_ticks: Optional[int]) -> None: ...
    def set_slider_end_hits(self, n_slider_ends: Optional[int]) -> None: ...
    def set_n_geki(self, n_geki: Optional[int]) -> None: ...
    def set_n_katu(self, n_katu: Optional[int]) -> None: ...
    def set_n300(self, n300: Optional[int]) -> None: ...
    def set_n100(self, n100: Optional[int]) -> None: ...
    def set_n50(self, n50: Optional[int]) -> None: ...
    def set_misses(self, misses: Optional[int]) -> None: ...
    def set_hitresult_priority(
        self, hitresult_priority: Optional[HitResultPriority]
    ) -> None: ...

class GradualDifficulty(Iterator):
    """
    Gradually calculate difficulty attributes after each hitobject
    """

    def __init__(self, difficulty: Difficulty, map: Beatmap) -> None: ...
    def next(self) -> Optional[DifficultyAttributes]:
        """
        Advances the iterator and returns the next attributes.
        """

    def nth(self, n: int) -> Optional[DifficultyAttributes]:
        """
        Returns the `n`th attributes of the iterator.

        Note that the count starts from zero, so `nth(0)` returns the first
        value, `nth(1)` the second, and so on.
        """

    @property
    def n_remaining(self) -> int:
        """
        The amount of remaining items.
        """

class GradualPerformance:
    """
    Gradually calculate performance attributes after each hitresult
    """

    def __init__(self, difficulty: Difficulty, map: Beatmap) -> None: ...
    def next(self, state: ScoreState) -> Optional[PerformanceAttributes]:
        """
        Process the next hit object and calculate the performance attributes
        for the resulting score state.
        """

    def nth(self, state: ScoreState, n: int) -> Optional[PerformanceAttributes]:
        """
        Process everything up to the next `n`th hitobject and calculate the
        performance attributes for the resulting score state.

        Note that the count is zero-indexed, so `n=0` will process 1 object,
        `n=1` will process 2, and so on.
        """

    @property
    def n_remaining(self) -> int:
        """
        The amount of remaining items.
        """

class BeatmapAttributesBuilder:
    """
    Calculator for beatmap attributes considering various settings

    The kwargs may include any of the following:
        `'mode': GameMode`
            Specify a gamemode
        `'is_convert': bool`
            Specify whether it's a converted map
        `'mods': GameMods`
            Specify mods.

            Relevant type aliases:
                `GameMods = Union[int, str, GameMod, List[Union[GameMod, str, int]]]`

                `GameMod = dict[str, Union[str, GameModSettings]]`
                    `GameMod` *must* have an item `'acronym': str` and an optional
                    item `'settings': GameModSettings`

                `GameModSettings = dict[str, Union[bool, float, str]]`

            See https://github.com/ppy/osu-api/wiki#mods
        `'clock_rate': float`
            Adjust the clock rate used in the calculation.

            If none is specified, it will take the clock rate based on the mods
            i.e. 1.5 for DT, 0.75 for HT and 1.0 otherwise.

            Clamped between 0.01 and 100.
        `'ar': float`
            Override a beatmap's set AR.

            Only relevant for osu! and osu!catch.

            Clamped between -20 and 20.
        `'ar_with_mods': bool`
            Determines if the given AR value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'cs': float`
            Override a beatmap's set CS.

            Only relevant for osu! and osu!catch.

            Clamped between -20 and 20.
        `'cs_with_mods': bool`
            Determines if the given CS value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'hp': float`
            Override a beatmap's set HP.

            Clamped between -20 and 20.
        `'hp_with_mods': bool`
            Determines if the given HP value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
        `'od': float`
            Override a beatmap's set OD.

            Clamped between -20 and 20.
        `'od_with_mods': bool`
            Determines if the given OD value should be used before
            or after accounting for mods, e.g. on `true` the value will be
            used as is and on `false` it will be modified based on the mods.
    """

    def __init__(self, **kwargs) -> None: ...
    def build(self) -> BeatmapAttributes:
        """
        Calculate the beatmap attributes
        """

    def set_map(self, map: Beatmap) -> None:
        """
        Consider the map's attributes, mode, and convert status
        """

    def set_mode(self, mode: Optional[GameMode], is_convert: bool) -> None: ...
    def set_mods(self, mods: Optional[GameMods]) -> None: ...
    def set_clock_rate(self, clock_rate: Optional[float]) -> None: ...
    def set_ar(self, ar: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set AR.

        Only relevant for osu! and osu!catch.

        Clamped between -20 and 20.

        `with_mods` determines if the given AR value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_cs(self, cs: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set CS.

        Only relevant for osu! and osu!catch.

        Clamped between -20 and 20.

        `with_mods` determines if the given CS value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_hp(self, hp: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set HP.

        Clamped between -20 and 20.

        `with_mods` determines if the given HP value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

    def set_od(self, od: Optional[float], with_mods: bool) -> None:
        """
        Override a beatmap's set OD.

        Clamped between -20 and 20.

        `with_mods` determines if the given OD value should be used before
        or after accounting for mods, e.g. on `true` the value will be
        used as is and on `false` it will be modified based on the mods.
        """

class ScoreState:
    """
    Aggregation for a score's current state.
    """

    def __init__(self, **kwargs) -> None: ...

    max_combo: int
    """
    Maximum combo that the score has had so far. **Not** the maximum
    possible combo of the map so far.
    
    Note that for osu!catch only fruits and droplets are considered for combo.
    
    Irrelevant for osu!mania.
    """

    osu_large_tick_hits: int
    """
    "Large tick" hits for osu!standard.
    
    The meaning depends on the kind of score:
    - if set on osu!stable, this field is irrelevant and can be `0`
    - if set on osu!lazer *without* `CL`, this field is the amount of hit
      slider ticks and repeats
    - if set on osu!lazer *with* `CL`, this field is the amount of hit
      slider heads, ticks, and repeats
    """

    osu_small_tick_hits: int
    """
    "Small tick" hits for osu!standard.

    These are essentially the slider end hits for lazer scores without
    slider accuracy.
    """

    slider_end_hits: int
    """
    Amount of successfully hit slider ends.
    
    Only relevant for osu!standard in lazer.
    """

    n_geki: int
    """
    Amount of current gekis (n320 for osu!mania).
    """

    n_katu: int
    """
    Amount of current katus (tiny droplet misses for osu!catch / n200 for osu!mania).
    """

    n300: int
    """
    Amount of current 300s (fruits for osu!catch).
    """

    n100: int
    """
    Amount of current 100s (droplets for osu!catch).
    """

    n50: int
    """
    Amount of current 50s (tiny droplets for osu!catch).
    """

    misses: int
    """
    Amount of current misses (fruits + droplets for osu!catch).
    """

class DifficultyAttributes:
    """
    The result of a difficulty calculation
    """

    @property
    def mode(self) -> GameMode:
        """
        The attributes' gamemode
        """

    @property
    def stars(self) -> float:
        """
        The final star rating
        """

    @property
    def is_convert(self) -> bool:
        """
        Whether the map was a convert
        """

    @property
    def aim(self) -> Optional[float]:
        """
        The difficulty of the aim skill.

        Only available for osu!.
        """

    @property
    def aim_difficult_slider_count(self) -> Optional[float]:
        """
        The number of sliders weighted by difficulty.

        Only available for osu!.
        """

    @property
    def speed(self) -> Optional[float]:
        """
        The difficulty of the speed skill.

        Only available for osu!.
        """

    @property
    def flashlight(self) -> Optional[float]:
        """
        The difficulty of the flashlight skill.

        Only available for osu!.
        """

    @property
    def slider_factor(self) -> Optional[float]:
        """
        The ratio of the aim strain with and without considering sliders

        Only available for osu!.
        """

    @property
    def speed_note_count(self) -> Optional[float]:
        """
        The number of clickable objects weighted by difficulty.

        Only available for osu!.
        """

    @property
    def aim_difficult_strain_count(self) -> Optional[float]:
        """
        Weighted sum of aim strains.

        Only available for osu!.
        """

    @property
    def speed_difficult_strain_count(self) -> Optional[float]:
        """
        Weighted sum of speed strains.

        Only available for osu!.
        """

    @property
    def hp(self) -> Optional[float]:
        """
        The health drain rate.

        Only available for osu!.
        """

    @property
    def n_circles(self) -> Optional[int]:
        """
        The amount of circles.

        Only available for osu!.
        """

    @property
    def n_sliders(self) -> Optional[int]:
        """
        The amount of sliders.

        Only available for osu!.
        """

    @property
    def n_large_ticks(self) -> Optional[int]:
        """
        The amount of "large tick" hits.

        Only relevant for osu!standard.

        The meaning depends on the kind of score:
        - if set on osu!stable, this value is irrelevant and can be `0`
        - if set on osu!lazer *with* slider accuracy, this value is the amount
          of hit slider ticks and repeats
        - if set on osu!lazer *without* slider accuracy, this value is the
          amount of hit slider heads, ticks, and repeats
        """

    @property
    def n_spinners(self) -> Optional[int]:
        """
        The amount of spinners.

        Only available for osu!.
        """

    @property
    def stamina(self) -> Optional[float]:
        """
        The difficulty of the stamina skill.

        Only available for osu!taiko.
        """

    @property
    def single_color_stamina(self) -> Optional[float]:
        """
        The difficulty of the single color stamina skill.

        Only available for osu!taiko.
        """

    @property
    def reading(self) -> Optional[float]:
        """
        The difficulty of the reading skill.

        Only available for osu!taiko.
        """

    @property
    def rhythm(self) -> Optional[float]:
        """
        The difficulty of the rhythm skill.

        Only available for osu!taiko.
        """

    @property
    def color(self) -> Optional[float]:
        """
        The difficulty of the color skill.

        Only available for osu!taiko.
        """

    @property
    def n_fruits(self) -> Optional[int]:
        """
        The amount of fruits.

        Only available for osu!catch.
        """

    @property
    def n_droplets(self) -> Optional[int]:
        """
        The amount of droplets.

        Only available for osu!catch.
        """

    @property
    def n_tiny_droplets(self) -> Optional[int]:
        """
        The amount of tiny droplets.

        Only available for osu!catch.
        """

    @property
    def n_objects(self) -> Optional[int]:
        """
        The amount of hitobjects in the map.

        Only available for osu!mania.
        """

    @property
    def n_hold_notes(self) -> Optional[int]:
        """
        The amount of hold notes in the map.

        Only available for osu!mania.
        """

    @property
    def ar(self) -> Optional[float]:
        """
        The approach rate.

        Only available for osu! and osu!catch.
        """

    @property
    def great_hit_window(self) -> Optional[float]:
        """
        The perceived hit window for an n300 inclusive of rate-adjusting mods (DT/HT/etc)

        Only available for osu! and osu!taiko.
        """

    @property
    def ok_hit_window(self) -> Optional[float]:
        """
        The perceived hit window for an n100 inclusive of rate-adjusting mods (DT/HT/etc)

        Only available for osu! and osu!taiko.
        """

    @property
    def meh_hit_window(self) -> Optional[float]:
        """
        The perceived hit window for an n50 inclusive of rate-adjusting mods (DT/HT/etc)

        Only available for osu!.
        """

    @property
    def max_combo(self) -> int:
        """
        The maximum combo on the map.
        """

class PerformanceAttributes:
    """
    The result of a performance calculation
    """

    @property
    def difficulty(self) -> DifficultyAttributes:
        """
        The difficulty attributes.
        """

    @property
    def pp(self) -> float:
        """
        The final performance points.
        """

    @property
    def pp_aim(self) -> Optional[float]:
        """
        The aim portion of the final pp.

        Only available for osu!.
        """

    @property
    def pp_flashlight(self) -> Optional[float]:
        """
        The flashlight portion of the final pp.

        Only available for osu!.
        """

    @property
    def pp_speed(self) -> Optional[float]:
        """
        The speed portion of the final pp.

        Only available for osu!.
        """

    @property
    def pp_accuracy(self) -> Optional[float]:
        """
        The accuracy portion of the final pp.

        Only available for osu! and osu!taiko.
        """

    @property
    def effective_miss_count(self) -> Optional[float]:
        """
        Scaled miss count based on total hits.

        Only available for osu! and osu!taiko.
        """

    @property
    def speed_deviation(self) -> Optional[float]:
        """
        Approximated unstable-rate

        Only available for osu!.
        """

    @property
    def estimated_unstable_rate(self) -> Optional[float]:
        """
        Upper bound on the player's tap deviation.

        Only *optionally* available for osu!taiko.
        """

    @property
    def pp_difficulty(self) -> Optional[float]:
        """
        The strain portion of the final pp.

        Only available for osu!taiko and osu!mania.
        """

    @property
    def state(self) -> Optional[ScoreState]:
        """
        The hitresult score state that was used for performance calculation.

        Only available if *not* created through gradual calculation.
        """

class Strains:
    """
    The result of calculating the strains of a beatmap.

    Suitable to plot the difficulty over time.
    """
    @property
    def mode(self) -> GameMode:
        """
        The strains' gamemode.
        """

    @property
    def section_length(self) -> float:
        """
        Time inbetween two strains in ms.
        """

    @property
    def aim(self) -> Optional[List[float]]:
        """
        Strain peaks of the aim skill in osu!
        """

    @property
    def aim_no_sliders(self) -> Optional[List[float]]:
        """
        Strain peaks of the aim skill without sliders in osu!
        """

    @property
    def speed(self) -> Optional[List[float]]:
        """
        Strain peaks of the speed skill in osu!
        """

    @property
    def flashlight(self) -> Optional[List[float]]:
        """
        Strain peaks of the flashlight skill in osu!
        """

    @property
    def color(self) -> Optional[List[float]]:
        """
        Strain peaks of the color skill in osu!taiko.
        """

    @property
    def reading(self) -> Optional[List[float]]:
        """
        Strain peaks of the reading skill in osu!taiko.
        """

    @property
    def rhythm(self) -> Optional[List[float]]:
        """
        Strain peaks of the rhythm skill in osu!taiko.
        """

    @property
    def stamina(self) -> Optional[List[float]]:
        """
        Strain peaks of the stamina skill in osu!taiko.
        """

    @property
    def single_color_stamina(self) -> Optional[List[float]]:
        """
        Strain peaks of the single color stamina skill in osu!taiko.
        """

    @property
    def movement(self) -> Optional[List[float]]:
        """
        Strain peaks of the movement skill in osu!catch.
        """

    @property
    def strains(self) -> Optional[List[float]]:
        """
        Strain peaks of the strain skill in osu!mania.
        """

class BeatmapAttributes:
    """
    The result of building a `BeatmapAttributesBuilder`.
    """

    @property
    def ar(self) -> float: ...
    @property
    def od(self) -> float: ...
    @property
    def cs(self) -> float: ...
    @property
    def hp(self) -> float: ...
    @property
    def clock_rate(self) -> float: ...
    @property
    def ar_hit_window(self) -> float:
        """
        Hit window for approach rate i.e. TimePreempt in milliseconds.
        """

    @property
    def od_great_hit_window(self) -> float:
        """
        Hit window for overall difficulty i.e. time to hit a 300 ("Great") in milliseconds.
        """

    @property
    def od_ok_hit_window(self) -> float:
        """
        Hit window for overall difficulty i.e. time to hit a 100 ("Ok") in milliseconds.

        Not available for osu!mania.
        """

    @property
    def od_meh_hit_window(self) -> float:
        """
        Hit window for overall difficulty i.e. time to hit a 50 ("Meh") in milliseconds.

        Only available for osu!.
        """
