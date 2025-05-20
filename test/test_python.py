from dataclasses import dataclass
import decomp_settings


def test_read_config():
    config = decomp_settings.read_config("test/decomp.yaml")
    assert config.platform == "n64"


def test_scan_for_config():
    try:
        decomp_settings.scan_for_config()
    except Exception:
        assert True
        return
    assert False, "Expected an exception"


def test_scan_for_config_from():
    config = decomp_settings.scan_for_config_from("test/subdir")
    assert config.platform == "n64"


def test_read_config_arbitrary_tool():

    @dataclass
    class Other:
        stuff: int

    @dataclass
    class ArbitraryToolOpts:
        meowp: int
        others: list[dict[str, Other]]

    config = decomp_settings.read_config("test/arbitrary_tool.yaml")
    tools = config.tools
    assert tools is not None
    arbitrary_tool_enum = tools.get("arbitrary_tool")
    assert arbitrary_tool_enum is not None
    assert isinstance(arbitrary_tool_enum, decomp_settings.ToolOpts.Other)

    print(dir(arbitrary_tool_enum))

    # Method where we just get the raw dict
    arbitrary_tool = arbitrary_tool_enum.raw()
    assert isinstance(arbitrary_tool, dict)
    assert arbitrary_tool.get("meowp") == 125
    assert arbitrary_tool.get("others")[0].get("thing").get("stuff") == 1
    assert arbitrary_tool.get("others")[1].get("thing2").get("stuff") == 2

    # Method where we convert the dict to a dataclass
    arbitrary_tool_opts = ArbitraryToolOpts(**arbitrary_tool)
    assert arbitrary_tool_opts.meowp == 125
    # We still have to get "stuff" because we didn't create Others from the dict
    assert arbitrary_tool_opts.others[0].get("thing").get("stuff") == 1
    assert arbitrary_tool_opts.others[1].get("thing2").get("stuff") == 2

    # TODO: Use https://github.com/konradhalas/dacite to convert the dict to a dataclass
