class Beatmap:
    """
    A class containing a parsed beatmap.

    The kwargs must include any of the following:
        `'path': str`
            The path to a .osu file
        `'content': str | bytearray`
            The content of a .osu file as string or bytes
        `'bytes': bytearray`
            The content of a .osu file as bytes

    The kwargs may include any of the following:
        `'ar': float`
            Specify a custom approach rate
        `'cs': float`
            Specify a custom circle size
        `'hp': float`
            Specify a custom drain rate
        `'od': float`
            Specify a custom overall difficulty

    ## Raises

    Throws an exception if the map could not be parsed or an invalid kwarg was given
    """

    def __init__(self, **kwargs) -> None: ...

    def set_ar(self, ar: float) -> None:
        """
        Specify a custom approach rate
        """
        ...

    def set_cs(self, cs: float) -> None:
        """
        Specify a custom circle size
        """
        ...

    def set_hp(self, hp: float) -> None:
        """
        Specify a custom drain rate
        """
        ...

    def set_od(self, od: float) -> None:
        """
        Specify a custom overall difficulty
        """
        ...


class Calculator:
    """
    A class containing various attributes to calculate strains or map, difficulty, or performance attributes.

    The kwargs may include any of the following:
        `'mode': int`
            Must be 0 for osu!standard, 1 for taiko, 2 for catch, or 3 for mania
        `'mods': int`
            Bitflags for mods, see https://github.com/ppy/osu-api/wiki#mods
        `'acc': float`
            The accuracy between 0.0 and 100.0
        `'n_geki': int`
            The amount of gekis i.e. n320 in mania
        `'n_katu': int`
            The amount of katu i.e. tiny droplet misses in catch and n200 in mania
        `'n300': int`
            The amount of n300
        `'n100': int`
            The amount of n100
        `'n50': int`
            The amount of n50
        `'n_misses': int`
            The amount of misses
        `'combo': int`
            The max combo of the score
        `'passed_object': int`
            The amount of passed objects, handy for partial plays like fails
        `'clock_rate': float`
            Specify a custom clock rate
        `'difficulty': DifficultyAttributes`
            If you perform multiple calculations and neither map, mode, mods, nor passed objects amount change,
            pass the difficulty attributes from a previous calculation so that they don't have to be recalculated

    ## Raises

    Throws an exception if an invalid kwarg was given
    """

    def __init__(self, **kwargs) -> None: ...

    def set_mode(self, mode: int) -> None:
        """
        Must be 0 for osu!standard, 1 for taiko, 2 for catch, or 3 for mania
        """
        ...

    def set_mods(self, mods: int) -> None:
        """
        Bitflags for mods, see https://github.com/ppy/osu-api/wiki#mods
        """
        ...

    def set_acc(self, acc: float) -> None:
        """
        The accuracy between 0.0 and 100.0
        """
        ...

    def set_n_geki(self, n_geki: int) -> None:
        """
        The amount of gekis i.e. n320 in mania
        """
        ...

    def set_n_katu(self, n_katu: int) -> None:
        """
        The amount of katu i.e. tiny droplet misses in catch and n200 in mania
        """
        ...

    def set_n300(self, n300: int) -> None:
        """
        The amount of n300
        """
        ...

    def set_n100(self, n100: int) -> None:
        """
        The amount of n100
        """
        ...

    def set_n50(self, n50: int) -> None:
        """
        The amount of n50
        """
        ...

    def set_n_misses(self, n_misses: int) -> None:
        """
        The amount of misses
        """
        ...

    def set_combo(self, combo: int) -> None:
        """
        The max combo of the score
        """
        ...

    def set_passed_objects(self, passed_objects: int) -> None:
        """
        The amount of passed objects, handy for partial plays like fails
        """
        ...

    def set_clock_rate(self, clock_rate: float) -> None:
        """
        Specify a custom clock rate
        """
        ...

    def set_difficulty(self, difficulty: DifficultyAttributes) -> None:
        """
        If you perform multiple calculations and neither map, mode, mods, nor passed objects change,
        pass the difficulty attributes from a previous calculation so that they don't have to be recalculated
        """
        ...

    def map_attributes(self, map: Beatmap) -> BeatmapAttributes:
        """
        Based on the specified mods and clock rate, calculate the beatmap attributes for the given map
        """
        ...

    def difficulty(self, map: Beatmap) -> DifficultyAttributes:
        """
        Based on all specified parameters, calculate the difficulty attributes for the given map
        """
        ...

    def performance(self, map: Beatmap) -> PerformanceAttributes:
        """
        Based on all specified parameters, calculate the performance attributes for the given map
        """
        ...

    def strains(self, map: Beatmap) -> Strains:
        """
        Based on all specified parameters, calculate the strains for the given map
        """
        ...


class BeatmapAttributes:
    """
    Various attributes of a beatmap

    ## Attributes

    `'ar': float`
        Approach rate
    `'cs': float`
        Circle size
    `'hp': float`
        Drain rate
    `'od': float`
        Overall difficulty
    `'ar_hit_window': float`
        Time in ms the the circle is visible ("time preempt")
    `'od_hit_window': float`
        Time in ms to get an n300 hitresult ("great hit window")
    `'clock_rate': float`
        Clock rate
    `'bpm': float`
        Beats per minute
    `'mode': int`
        Gamemode integer
    `'version': int`
        Version of the .osu file
    `'n_circles': int`
        Amount of circles
    `'n_sliders': int`
        Amount of sliders
    `'n_spinners': int`
        Amount of spinners
    """


class DifficultyAttributes:
    """
    All difficulty attributes depending on the mode.

    ## Attributes

    The parentheses indicate for which mode the optional values will be available.

    `'mode': int`
        Gamemode integer
    `'stars': float`
        Star rating
    `'max_combo': int`
        Max combo
    `'aim': Optional[float]`
        Aim based portion of the star rating (O)
    `'speed': Optional[float]`
        Speed based portion of the star rating (O)
    `'flashlight': Optional[float]`
        Flashlight based portion of the star rating (O)
    `'slider_factor': Optional[float]`
        Nerf factor for aim based on slider difficulty (O)
    `'speed_note_count': Optional[float]`
        Amount of notes that are considered as difficult regarding speed (O)
    `'ar': Optional[float]`
        Approach rate (O, T)
    `'od': Optional[float]`
        Overall difficulty (O)
    `'n_circles': Optional[int]`
        Amount of circles (O)
    `'n_sliders': Optional[int]`
        Amount of sliders (O)
    `'n_spinners': Optional[int]`
        Amount of spinners (O)
    `'stamina': Optional[float]`
        Stamina based portion of the star rating (T)
    `'color': Optional[float]`
        Color based portion of the star rating (T)
    `'rhythm': Optional[float]`
        Rhythm based portion of the star rating (T)
    `'peak': Optional[float]`
        Combination of stamina, color, and rhythm ratings (T)
    `'hit_window': Optional[float]`
        Great hit window (T, M)
    `'n_fruits': Optional[int]`
        Amount of fruits (C)
    `'n_droplets': Optional[int]`
        Amount of droplets (C)
    `'n_tiny_droplets': Optional[int]`
        Amount of tiny droplets (C)
    """


class PerformanceAttributes:
    """
    All performance attributes depending on the mode.

    ## Attributes

    The parentheses indicate for which mode the optional values will be available.

    `'mode': int`
        Gamemode integer
    `'pp': float`
        Performance points
    `'difficulty': DifficultyAttributes`
        Difficulty attributes based on the mode
    `'pp_acc': Optional[float]`
        Accuracy based portion of the performance points (O, T)
    `'pp_aim': Optional[float]`
        Aim based portion of the performance points (O)
    `'pp_speed': Optional[float]`
        Speed based portion of the performance points (O)
    `'pp_flashlight': Optional[float]`
        Flashlight based portion of the performance points (O)
    `'effective_miss_count': Optional[float]`
        Approximated misses including actual misses and assumed slider breaks (O, T)
    `'pp_difficulty': Optional[float]`
        Difficulty based portion of the performance points (T, M)
    """


class Strains:
    """
    All strain values depending on the mode

    ## Attributes

    The parentheses indicate for which mode the optional values will be available.

    `'mode': int`
        Gamemode integer
    `'section_len': float`
        Time in ms between two strain points
    `'aim': Optional[List[float]]`
        Aim strain values (O)
    `'aim_no_sliders': Optional[List[float]]`
        Aim strain values with sliders (O)
    `'speed': Optional[List[float]]`
        Speed strain values (O)
    `'flashlight': Optional[List[float]]`
        Flashlight strain values (O)
    `'color': Optional[List[float]]`
        Color strain values (T)
    `'stamina': Optional[List[float]]`
        Stamina strain values (T)
    `'rhythm': Optional[List[float]]`
        Rhythm strain values (T)
    `'movement': Optional[List[float]]`
        Movement strain values (C)
    `'strains': Optional[List[float]]`
        Strain values (M)
    """
