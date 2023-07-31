# CodeCTRL

[![Formatting](https://github.com/STBoyden/codectrl/actions/workflows/reformat.yml/badge.svg)](https://github.com/STBoyden/codectrl/actions/workflows/reformat.yml)
[![Clippy](https://github.com/STBoyden/codectrl/actions/workflows/clippy.yml/badge.svg)](https://github.com/STBoyden/codectrl/actions/workflows/clippy.yml)
[![Build & Packaging](https://github.com/STBoyden/codectrl/actions/workflows/build-and-package.yml/badge.svg)](https://github.com/STBoyden/codectrl/actions/workflows/build-and-package.yml)
[![Auto-update Build Containers](https://github.com/STBoyden/codectrl/actions/workflows/update-containers.yml/badge.svg)](https://github.com/STBoyden/codectrl/actions/workflows/update-containers.yml)

## Implementing a logger for a language

By default, these are the officially supported loggers:

| Language | Link                                                                 |
| :------- | :------------------------------------------------------------------- |
| Rust     | [Here](https://github.com/STBoyden/codectrl/tree/main/crates/logger) |
| Go       | [Here](https://github.com/STBoyden/codectrl-go-logger)               |
| C++      | [Here](https://github.com/STBoyden/codectrl-cxx-logger)              |
| Python   | [Here](https://github.com/STBoyden/codectrl-python-logger)           |
| PHP      | [Here](https://github.com/STBoyden/codectrl-php-logger)              |
| NodeJS   | [Here](https://github.com/STBoyden/codectrl-nodejs-logger)           |

All language loggers now need to use gRPC in order to implement the API schema.
The protobuf files are available
[here](https://github.com/STBoyden/codectrl-protobuf-specifications).

Unofficial language loggers:

- None yet (remove me if/when one is added).

## Build requirements

Below you will find the requirements to build on each platform. The supported platform(s)
are:

- [Linux](#linux) - Supported: Ubuntu 22.04, Ubuntu 20.04, Fedora 38, Fedora Rawhide,
  Debian 11, Debian 10 and Debian Sid.
- [Windows](#windows-and-macos)
- [MacOS](#windows-and-macos) - Supported: 13, 12, 11 (Intel and Apple Silicon).

Planned support:

Packages for the supported distributions listed above can be found
[here](https://github.com/STBoyden/codectrl/actions/workflows/build-and-package.yml)
underneath each of the **_completed_** CI jobs.

### Linux

The current _officially_ supported Linux distributions are the following:

- [Fedora (38, Rawhide)](#fedora-and-rhel)
- [RHEL, and compatible distros (7, 8, 9)](#fedora-and-rhel)
- [Ubuntu (22.04, 20.04) and Debian (12, Sid)](#debian-based)

**_NOTE:_** You can use the `./bootstrap-build.sh` or the
`./bootstrap-action.sh` scripts to automatically install the dependencies for
the supported distributions.

Support is planned for the following:

- Arch (and it's derivatives)

#### Fedora and RHEL

Minimum supported Fedora version: 38.

You will need to install the "Development Tools" group. You can do this by running:
`sudo dnf groupinstall "Development Tools" -y`.

##### Build dependencies

Aside from the Cargo toolchain, you will need:

- `freetype-devel `
- `expat-devel`
- `fontconfig-devel`
- `cmake`^[[1](#note-rhel-cmake)]
- A C++ compiler, most likely `g++` or `clang`

<div id="note-rhel-cmake" />

_*Note:*_ you _will_ need to install `cmake3` if you are on RHEL 7 and symlink it to `/usr/bin/cmake`, otherwise the build will fail.

#### Debian-based

There is support for Ubuntu 22.04, Ubuntu 20.04, Debian 11, Debian 10 and
Debian Sid.

##### Build dependencies

Aside from the Cargo toolchain, you will need:

- `build-essential`
- `libfreetype-dev`
- `libfontconfig-dev`
- `cmake`
- A C++ compiler, most likely `g++` or `clang`

### Windows and MacOS

You can build CodeCTRL for Windows and MacOS simply by installing `rustup` via the normal
channel: [here](https://rustup.rs), and issuing a `cargo build --release` at
the root of this project.

MSIs for Windows is automatically generated on every commit of the CodeCTRL
`main` branch and can be found in one of the completed workflow runs
[here](https://github.com/STBoyden/codectrl/actions/workflows/build-and-package.yml).

Similarly, there are generated MacOS app packages on every commit on the CodeCTRL main branch and can be gound on the same link above.
