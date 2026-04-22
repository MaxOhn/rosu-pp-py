import pytest

import rosu_pp_py as rosu
from rosu_pp_py import ArgsError, ParseError


class TestBeatmapCreation:
    def test_from_path(self, osu_map):
        assert osu_map is not None
        assert osu_map.mode == rosu.GameMode.Osu

    def test_from_content(self, osu_map_content):
        assert osu_map_content is not None
        assert osu_map_content.mode == rosu.GameMode.Osu

    def test_from_bytes(self, osu_map_bytes):
        assert osu_map_bytes is not None
        assert osu_map_bytes.mode == rosu.GameMode.Osu

    def test_no_kwargs_raises(self):
        with pytest.raises(ArgsError):
            rosu.Beatmap()

    def test_invalid_path_raises(self):
        with pytest.raises(ParseError):
            rosu.Beatmap(path="/nonexistent/path/file.osu")


class TestBeatmapProperties:
    @pytest.fixture
    def map(self, osu_map):
        return osu_map

    def test_mode(self, map):
        assert map.mode == rosu.GameMode.Osu

    def test_bpm(self, map):
        assert map.bpm > 0

    def test_version(self, map):
        assert map.version > 0

    def test_is_convert(self, map):
        assert map.is_convert is False

    def test_stack_leniency(self, map):
        assert map.stack_leniency == 0.5

    def test_ar(self, map):
        assert map.ar == pytest.approx(9.3, abs=0.01)

    def test_cs(self, map):
        assert map.cs == 4.5

    def test_hp(self, map):
        assert map.hp == 5.0

    def test_od(self, map):
        assert map.od == pytest.approx(8.8, abs=0.01)

    def test_slider_multiplier(self, map):
        assert map.slider_multiplier == 1.7

    def test_slider_tick_rate(self, map):
        assert map.slider_tick_rate == 1.0

    def test_n_objects(self, map):
        assert map.n_objects == 47

    def test_n_circles(self, map):
        assert map.n_circles == 43

    def test_n_sliders(self, map):
        assert map.n_sliders == 4

    def test_n_spinners(self, map):
        assert map.n_spinners == 0

    def test_n_breaks(self, map):
        assert map.n_breaks == 1

    def test_n_holds(self, map):
        assert map.n_holds == 0

    def test_repr(self, map):
        assert "Beatmap" in repr(map)


class TestBeatmapConversion:
    def test_convert_to_mania(self, osu_map):
        osu_map.convert(rosu.GameMode.Mania, None)
        assert osu_map.mode == rosu.GameMode.Mania

    def test_convert_with_mods(self, osu_map):
        osu_map.convert(rosu.GameMode.Taiko, "HD")
        assert osu_map.mode == rosu.GameMode.Taiko
