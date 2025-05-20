from pathlib import Path
from typing import Any

def scan_for_config() -> Config:
    """
    Looks for a configuration file named `decomp.yaml` starting from the current directory and going to all parent directories.
    """

def scan_for_config_from(start: Path | str) -> Config:
    """
    Looks for a configuration file named `decomp.yaml` starting from the given directory and going to all parent directories.
    """

def read_config(path: Path | str) -> Config:
    """
    Reads a configuration file from the given path.
    """

class Config:
    """
    The top-level configuration struct. This represents the entirety of the decomp.yaml format.
    """

    name: str
    """
    Human-readable name of the project. Example: Paper Mario
    """
    github: str | None
    """
    The repository URL of the project
    """
    website: str | None
    """
    The website for the project
    """
    discord: str | None
    """
    An invite link to the project's Discord server
    """
    platform: str
    """
    The platform the project is for. Example: n64
    """
    build_system: str | None
    """
    The build system used by the project. Example: [ninja, make]
    """
    versions: list[Version]
    """
    A list of all Versions inside the project
    """
    tools: dict[str, ToolOpts] | None
    """
    Settings for various tools
    """

    def get_version_by_name(self, version: str) -> Version | None: ...

class Version:
    """
    Represents a single version of a project. A project can have one or several versions. For example, one might decompile the JP 1.0, JP 1.1, and US 1.1 versions of a game. These each would be a Version.
    """

    name: str
    """
    The shorthand identifier for the version, meant to be easily memorable and typable. Example: us10
    """
    fullname: str
    """
    The human-readable name of the version. Example: US 1.0
    """
    sha1: str | None
    """
    The sha1 hash of the target executable. This can be used by tools to ensure the correct executable is being worked with.
    """
    paths: dict[str, str]
    """
    A map of path names to paths that tools may care about. Common paths would be baserom, asm, build, map, expected, etc.
    """

class ToolOpts:
    """
    Represents a tool and its settings
    """

    class Decompme: ...
    class Permuter: ...
    class Frogress: ...
    class Other: ...

    def raw(self) -> Any|None: ...

class DecompmeOpts:
    """
    Settings for the [decomp.me](https://decomp.me) platform
    """

    preset: int

class PermuterOpts:
    """
    Settings for [decomp-permuter](https://github.com/simonlindholm/decomp-permuter)
    """

    decompme_compilers: dict[str, str]

class FrogressOpts:
    """
    Settings for [frogress](https://github.com/decompals/frogress)
    """

    project: str
    """
    The project slug as defined in the frogress database
    """
    versions: dict[str, FrogressVersionOpts]
    """
    The frogress settings for each version of the project
    """

class FrogressVersionOpts:
    """
    Settings for a version in frogress's schema
    """

    version: str
