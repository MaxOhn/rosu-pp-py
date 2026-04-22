import pathlib

import pytest

import rosu_pp_py as rosu

FIXTURES = pathlib.Path(__file__).parent / "fixtures"


@pytest.fixture
def osu_map():
    return rosu.Beatmap(path=str(FIXTURES / "test_map.osu"))


@pytest.fixture
def osu_map_content():
    content = (FIXTURES / "test_map.osu").read_text()
    return rosu.Beatmap(content=content)


@pytest.fixture
def osu_map_bytes():
    content = (FIXTURES / "test_map.osu").read_bytes()
    return rosu.Beatmap(bytes=content)


@pytest.fixture
def diff():
    return rosu.Difficulty()


@pytest.fixture
def perf():
    return rosu.Performance()
