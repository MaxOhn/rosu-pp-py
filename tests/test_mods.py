import rosu_pp_py as rosu


class TestModsBitflags:
    def test_single_mod(self):
        perf = rosu.Performance(mods=8)
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None

    def test_combined_mods(self):
        perf = rosu.Performance(mods=8 + 64)
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None


class TestModsString:
    def test_single_acronym(self):
        perf = rosu.Performance(mods="HR")
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None

    def test_multiple_acronyms(self):
        perf = rosu.Performance(mods="HRDT")
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None

    def test_invalid_acronym_ignored(self):
        perf = rosu.Performance(mods="INVALID")
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None


class TestModsDict:
    def test_simple_dict(self):
        perf = rosu.Performance(mods={"acronym": "FI"})
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None

    def test_dict_with_settings(self):
        perf = rosu.Performance(
            mods={
                "acronym": "AC",
                "settings": {"minimum_accuracy": 95, "restart": True},
            }
        )
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None


class TestModsList:
    def test_list_of_bits(self):
        perf = rosu.Performance(mods=[1024, "nf"])
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None

    def test_list_of_dicts(self):
        perf = rosu.Performance(
            mods=[
                {"acronym": "TC"},
                {"acronym": "HT", "settings": {"speed_change": 0.6}},
            ]
        )
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None

    def test_mixed_list(self):
        perf = rosu.Performance(
            mods=[
                1024,
                "nf",
                {"acronym": "AC", "settings": {"minimum_accuracy": 95}},
            ]
        )
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None


class TestModsNone:
    def test_no_mods(self):
        perf = rosu.Performance()
        attrs = perf.calculate(rosu.Beatmap(path="tests/fixtures/test_map.osu"))
        assert attrs is not None
