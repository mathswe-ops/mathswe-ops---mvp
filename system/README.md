<!-- Copyright (c) 2024 Tobias Briones. All rights reserved. -->
<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp -->

# System

[![Project](public/system-mvp-app-badge.svg)](https://ops.math.software)
&nbsp;
[![GitHub Repository](https://img.shields.io/static/v1?label=GITHUB&message=REPOSITORY&labelColor=555&color=0277bd&style=for-the-badge&logo=GITHUB)](https://github.com/mathswe-ops/mathswe-ops---mvp/blob/main/system)

[![GitHub Project License](https://img.shields.io/github/license/mathswe-ops/mathswe-ops---mvp.svg?style=flat-square)](https://github.com/mathswe-ops/mathswe-ops---mvp/blob/main/LICENSE)

![GitHub Release](public/system-mvp-app-release-badge.svg)

The System CLI application is a part of the MathSwe Ops MVP to automate software
operations in Linux, such as installation, uninstallation, and configuration,
allowing you to set up server VMs and desktop Workstations by running a command.

## MathSwe System Ops MVP

The System MVP provides robust engineering for the essential
(i.e., "Minimum" of MVP) modules while aiming to evolve into the
engineering-grade version, following MathSwe standards for MVPs.

These standards include ensuring the robustness of essential modules,
maintaining a workflow suitable for a pre-release yet stable project (i.e.,
`v0.y.z`, relatively stable), and eventually devising its DSL (Domain Specific
Language) that gives it the engineering grade while the MVP is already serving
in production.

MathSwe Ops System MVP is an application with a reliable and evolving design
that makes cloud VMs and desktop work machines productive from cold OS
installation. It automates cold-start DevOps and staff onboarding as per your
organization's standards.

## Image Ops

An image is a model of a software package with OS operations to install,
uninstall, or reinstall it.

The System app currently implements two major kinds of images, namely, server
and desktop.

While server images are software that works on any machine, including desktop
ones, desktop images only work with a machine with GUI and are intended for
Workstation machines.

An image ID is the lowercase and hyphen-separated value of its enum variant, for
example, `Rust => rust`, `JetBrainsToolbox => jetbrains-toolbox`, etc.

Images provide type-safe software models, and the System app user will need
their IDs to execute the operations the app implements for these images.

### Image Installation

The operation `Install` loads the given images from the program repository and
executes the software installation in the host OS.

*Syntax:* `system install { image_1, image_2, ..., image_n }`.

Make sure to install in the correct order if they are dependencies since the MVP
won't implement checking the OS state before operating for each ad-hoc image.

You can add one or many images, and the program will install them one after
another.

### Image Uninstallation

The operation `Uninstall` loads the given images from the program repository and
executes the software uninstallation in the host OS.

*Syntax:* `system install { image_1, image_2, ..., image_n }`.

You can add one or many images, and the program will uninstall them one after
another.

## Available Images

The list of currently supported images is next.

`Available Server Images`

- Rust
- Go
- Sdkman
- Java
- Gradle
- Nvm
- Node
- Miniconda

`Available Desktop Images`

- Zoom
- VsCode
- JetBrainsToolbox

`Available JetBrainsIde Images`

- IntelliJIdea
- WebStorm
- RustRover
- CLion
- PyCharm
- DataGrip
- Goland
- Rider
- PhpStorm
- RubyMine

## OS Compatibility

The System MVP app is designed and tested for **Ubuntu** and should work for
many other distros, although it may not work and is not necessarily intended to
work on non-Debian distros.

The MVP *will not officially support other Linux distros*; only the
engineering-grade version will.

Support for closed-source OSs like Windows is *discouraged* and *incompatible*
with the engineering-grade version. Software with engineering grade must be
thoroughly open source to comply with the "public service" engineering standard,
just like science works with open development.

## About

**System | MathSwe Ops MVP**

It automates software operations in Linux, such as installation, uninstallation,
and configuration, allowing you to set up server VMs and desktop Workstations by
running a command.

Copyright © 2024 Tobias Briones. All rights reserved.

### License

This project is licensed under the
[GNU General Public License v3.0 or later License](../LICENSE).
