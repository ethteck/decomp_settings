# Decomp Settings File

[There](https://github.com/matt-kempster/m2c) [are](https://github.com/simonlindholm/decomp-permuter) [a](https://github.com/decompals/frogress) [lot](https://github.com/Decompollaborate/spimdisasm) [of](https://github.com/Decompollaborate/mapfile_parser) [decompilation](https://github.com/ethteck/splat) [tools](https://github.com/ethteck/coddog).

Common metadata like the location of the .map file and target file are often needed by these tools, which unfortunately requires lots of repeated information. Whether it's through settings files or needing to manually specify common settings as cli args, there is a lot of redundancy that can be addressed.

The idea with this project is to centralize settings so tools can mutually benefit from a common standard. I also want to support a library that can read this settings file and provide these settings to tools.

## [Spec](test/decomp.yaml)

More shoon

## Library

The settings library is written in Rust and has Python bindings.

![PyPI - Version](https://img.shields.io/pypi/v/decomp_settings)
![Crates.io Version](https://img.shields.io/crates/v/decomp_settings?style=flat)
