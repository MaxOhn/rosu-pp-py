import pytest
import rosu_pp_py as rosu
from rosu_pp_py import ArgsError


class TestDifficultyCreation:
    def test_default(self):
        diff = rosu.Difficulty()
        assert diff is not None

    def test_with_mods_bitflags(self):
        diff = rosu.Difficulty(mods=8 + 64)
        assert diff is not None

    def test_with_mods_string(self):
        diff = rosu.Difficulty(mods="HRDT")
        assert diff is not None

    def test_with_clock_rate(self):
        diff = rosu.Difficulty(clock_rate=1.5)
        assert diff is not None

    def test_with_ar_override(self):
        diff = rosu.Difficulty(ar=10.0, fixed_ar=True)
        assert diff is not None

    def test_with_lazer(self):
        diff = rosu.Difficulty(lazer=True)
        assert diff is not None

    def test_invalid_kwarg_raises(self):
        with pytest.raises(ArgsError):
            rosu.Difficulty(invalid_kwarg=123)


class TestDifficultySetters:
    def test_set_mods(self, diff):
        diff.set_mods(8 + 64)
        assert diff is not None

    def test_set_clock_rate(self, diff):
        diff.set_clock_rate(1.5)

    def test_set_ar(self, diff):
        diff.set_ar(10.0, True)

    def test_set_cs(self, diff):
        diff.set_cs(4.0, False)

    def test_set_hp(self, diff):
        diff.set_hp(5.0, True)

    def test_set_od(self, diff):
        diff.set_od(8.0, False)

    def test_set_passed_objects(self, diff):
        diff.set_passed_objects(10)

    def test_set_hardrock_offsets(self, diff):
        diff.set_hardrock_offsets(True)

    def test_set_lazer(self, diff):
        diff.set_lazer(False)


class TestDifficultyCalculate:
    def test_calculate(self, diff, osu_map):
        attrs = diff.calculate(osu_map)
        assert attrs is not None
        assert attrs.stars >= 0
        assert attrs.mode == rosu.GameMode.Osu

    def test_strains(self, diff, osu_map):
        strains = diff.strains(osu_map)
        assert strains is not None
        assert strains.mode == rosu.GameMode.Osu


class TestDifficultyGradual:
    def test_gradual_difficulty(self, diff, osu_map):
        gradual = diff.gradual_difficulty(osu_map)
        assert gradual is not None

        results = list(gradual)
        assert isinstance(results, list)

    def test_gradual_difficulty_next(self, diff, osu_map):
        gradual = diff.gradual_difficulty(osu_map)
        first = gradual.next()
        assert first is not None
        assert first.n_circles >= 1

    def test_gradual_difficulty_nth(self, diff, osu_map):
        gradual = diff.gradual_difficulty(osu_map)
        result = gradual.nth(0)
        assert result is not None
        assert result.n_circles >= 1

    def test_gradual_difficulty_n_remaining(self, diff, osu_map):
        gradual = diff.gradual_difficulty(osu_map)
        assert gradual.n_remaining >= 0

    def test_gradual_performance(self, diff, osu_map):
        gradual = diff.gradual_performance(osu_map)
        assert gradual is not None

    def test_gradual_performance_next(self, diff, osu_map):
        gradual = diff.gradual_performance(osu_map)
        state = rosu.ScoreState(max_combo=51, n300=47, n100=0, n50=0, misses=0)
        result = gradual.next(state)
        assert result is not None
        assert result.pp >= 0

    def test_gradual_performance_nth(self, diff, osu_map):
        gradual = diff.gradual_performance(osu_map)
        state = rosu.ScoreState(max_combo=51, n300=47, n100=0, n50=0, misses=0)
        result = gradual.nth(state, 0)
        assert result is not None
        assert result.pp >= 0
