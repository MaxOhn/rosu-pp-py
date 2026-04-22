import pytest

import rosu_pp_py as rosu
from rosu_pp_py import ArgsError


class TestPerformanceCreation:
    def test_default(self):
        perf = rosu.Performance()
        assert perf is not None

    def test_with_mods(self):
        perf = rosu.Performance(mods=8 + 64)
        assert perf is not None

    def test_with_accuracy(self):
        perf = rosu.Performance(accuracy=99.5)
        assert perf is not None

    def test_with_combo(self):
        perf = rosu.Performance(combo=500)
        assert perf is not None

    def test_with_misses(self):
        perf = rosu.Performance(misses=1)
        assert perf is not None

    def test_with_hitresult_priority(self):
        perf = rosu.Performance(hitresult_priority=rosu.HitResultPriority.BestCase)
        assert perf is not None

    def test_with_hitresult_generator(self):
        perf = rosu.Performance()
        perf.set_hitresult_generator(rosu.HitResultGenerator.Fast, rosu.GameMode.Osu)


class TestPerformanceSetters:
    def test_set_mods(self, perf):
        perf.set_mods(8 + 64)

    def test_set_clock_rate(self, perf):
        perf.set_clock_rate(1.5)

    def test_set_ar(self, perf):
        perf.set_ar(10.0, True)

    def test_set_cs(self, perf):
        perf.set_cs(4.0, False)

    def test_set_hp(self, perf):
        perf.set_hp(5.0, True)

    def test_set_od(self, perf):
        perf.set_od(8.0, False)

    def test_set_accuracy(self, perf):
        perf.set_accuracy(99.0)

    def test_set_combo(self, perf):
        perf.set_combo(500)

    def test_set_misses(self, perf):
        perf.set_misses(2)

    def test_set_n300(self, perf):
        perf.set_n300(100)

    def test_set_n100(self, perf):
        perf.set_n100(50)

    def test_set_n50(self, perf):
        perf.set_n50(10)

    def test_set_n_geki(self, perf):
        perf.set_n_geki(50)

    def test_set_n_katu(self, perf):
        perf.set_n_katu(30)

    def test_set_large_tick_hits(self, perf):
        perf.set_large_tick_hits(20)

    def test_set_small_tick_hits(self, perf):
        perf.set_small_tick_hits(10)

    def test_set_slider_end_hits(self, perf):
        perf.set_slider_end_hits(5)

    def test_set_legacy_total_score(self, perf):
        perf.set_legacy_total_score(1000000)

    def test_set_hitresult_priority(self, perf):
        perf.set_hitresult_priority(rosu.HitResultPriority.WorstCase)

    def test_set_hitresult_generator(self, perf):
        perf.set_hitresult_generator(rosu.HitResultGenerator.Closest, rosu.GameMode.Osu)

    def test_set_hitresult_generator_all_modes(self, perf):
        perf.set_hitresult_generator(rosu.HitResultGenerator.Fast, None)


class TestPerformanceCalculate:
    def test_calculate_with_beatmap(self, perf, osu_map):
        attrs = perf.calculate(osu_map)
        assert attrs is not None
        assert attrs.pp >= 0
        assert attrs.difficulty is not None

    def test_calculate_with_difficulty_attrs(self, diff, perf, osu_map):
        diff_attrs = diff.calculate(osu_map)
        perf.set_accuracy(99.0)
        attrs = perf.calculate(diff_attrs)
        assert attrs is not None
        assert attrs.pp >= 0

    def test_calculate_with_performance_attrs(self, perf, osu_map):
        attrs1 = perf.calculate(osu_map)
        perf.set_accuracy(100)
        attrs2 = perf.calculate(attrs1)
        assert attrs2 is not None
        assert attrs2.pp >= 0

    def test_calculate_invalid_arg_raises(self, perf):
        with pytest.raises(ArgsError):
            perf.calculate("invalid")

    def test_difficulty_builder(self, perf):
        diff = perf.difficulty()
        assert diff is not None
