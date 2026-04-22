import pytest
import rosu_pp_py as rosu


class TestGameMode:
    def test_enum_values(self):
        assert rosu.GameMode.Osu == 0
        assert rosu.GameMode.Taiko == 1
        assert rosu.GameMode.Catch == 2
        assert rosu.GameMode.Mania == 3

    def test_equality(self):
        assert rosu.GameMode.Osu == rosu.GameMode.Osu
        assert rosu.GameMode.Osu != rosu.GameMode.Taiko

    def test_default(self):
        _diff = rosu.Difficulty()
        _perf = rosu.Performance()

    def test_repr(self):
        assert "GameMode" in repr(rosu.GameMode.Osu)


class TestHitResultPriority:
    def test_enum_values(self):
        assert rosu.HitResultPriority.BestCase == 0
        assert rosu.HitResultPriority.WorstCase == 1

    def test_equality(self):
        assert rosu.HitResultPriority.BestCase == rosu.HitResultPriority.BestCase
        assert rosu.HitResultPriority.BestCase != rosu.HitResultPriority.WorstCase


class TestHitResultGenerator:
    def test_enum_values(self):
        assert rosu.HitResultGenerator.Fast == 0
        assert rosu.HitResultGenerator.Closest == 1

    def test_equality(self):
        assert rosu.HitResultGenerator.Fast == rosu.HitResultGenerator.Fast
        assert rosu.HitResultGenerator.Fast != rosu.HitResultGenerator.Closest


class TestScoreState:
    def test_default(self):
        state = rosu.ScoreState()
        assert state.max_combo == 0
        assert state.n300 == 0

    def test_with_kwargs(self):
        state = rosu.ScoreState(max_combo=500, n300=400, n100=50, n50=10, misses=2)
        assert state.max_combo == 500
        assert state.n300 == 400
        assert state.n100 == 50
        assert state.n50 == 10
        assert state.misses == 2

    def test_setters(self):
        state = rosu.ScoreState()
        state.max_combo = 100
        state.n300 = 50
        state.n100 = 10
        state.n50 = 5
        state.misses = 1
        state.n_geki = 20
        state.n_katu = 10
        state.legacy_total_score = 1000000

        assert state.max_combo == 100
        assert state.n_geki == 20
        assert state.n_katu == 10
        assert state.legacy_total_score == 1000000

    def test_repr(self):
        state = rosu.ScoreState(max_combo=100)
        assert "ScoreState" in repr(state)


class TestDifficultyAttributes:
    def test_basic_attrs(self, diff, osu_map):
        attrs = diff.calculate(osu_map)
        assert attrs.mode == rosu.GameMode.Osu
        assert attrs.max_combo >= 0
        assert attrs.is_convert is False


class TestPerformanceAttributes:
    def test_basic_attrs(self, perf, osu_map):
        attrs = perf.calculate(osu_map)
        assert attrs.pp >= 0
        assert attrs.difficulty is not None

    def test_state(self, perf, osu_map):
        attrs = perf.calculate(osu_map)
        assert attrs.state is not None


class TestBeatmapAttributes:
    def test_builder(self):
        builder = rosu.BeatmapAttributesBuilder()
        assert builder is not None

    def test_builder_with_mode(self):
        builder = rosu.BeatmapAttributesBuilder(mode=rosu.GameMode.Mania)
        assert builder is not None

    def test_build(self, osu_map):
        builder = rosu.BeatmapAttributesBuilder()
        builder.set_map(osu_map)
        attrs = builder.build()
        assert attrs is not None
        assert attrs.ar >= 0
        assert attrs.od >= 0

    def test_builder_with_mods(self):
        builder = rosu.BeatmapAttributesBuilder(mods="HR")
        builder.set_mode(rosu.GameMode.Osu, False)
        attrs = builder.build()
        assert attrs is not None


class TestStrains:
    def test_strains(self, diff, osu_map):
        strains = diff.strains(osu_map)
        assert strains.mode == rosu.GameMode.Osu
        assert strains.section_length > 0
        assert strains.aim is not None


class TestErrors:
    def test_parse_error(self):
        with pytest.raises(Exception):
            rosu.Beatmap(path="/nonexistent/file.osu")

    def test_args_error(self):
        with pytest.raises(Exception):
            rosu.Performance(invalid_kwarg=123)
