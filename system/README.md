<!-- Copyright (c) 2024 Tobias Briones. All rights reserved. -->
<!-- SPDX-License-Identifier: GPL-3.0-or-later -->
<!-- This file is part of https://github.com/mathswe-ops/mathswe-ops---mvp -->

# System

[![Project](https://mathswe-ops-services.tobiasbriones-dev.workers.dev/badge/project/mathswe-system-ops?mvp)](https://ops.math.software#system)
&nbsp;
[![GitHub Repository](https://img.shields.io/static/v1?label=GITHUB&message=REPOSITORY&labelColor=555&color=0277bd&style=for-the-badge&logo=GITHUB)](https://github.com/mathswe-ops/mathswe-ops---mvp/blob/main/system)

[![GitHub Project License](https://img.shields.io/github/license/mathswe-ops/mathswe-ops---mvp.svg?style=flat-square)](https://github.com/mathswe-ops/mathswe-ops---mvp/blob/main/LICENSE)

![GitHub Release](https://mathswe-ops-services.tobiasbriones-dev.workers.dev/badge/version/github/mathswe-ops/mathswe-ops---mvp?path=system)

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

#### Install with Config

The flag `--config` will perform a restoration after installing the image, if
the application supports the `Config` operation for that image.

*Syntax:* `system install --config { image_1, image_2, ..., image_n }`.

The installation operation allows you to execute the image configuration, if
available, after installing it in your host OS.

### Image Re-Installation

The composed operation `reinstall` will apply the procedural operations
`Uninstall` and then `Install` to reinstall the software.

*Syntax:* `system reinstall { image_1, image_2, ..., image_n }`.

You can add one or many images, and the program will reinstall them one after
another.

### Image Configuration

The operation `Config` loads the image and configuration implementation, if any,
from the program repository and executes the software restoration.

*Syntax:* `system config { image_1, image_2, ..., image_n }`.

You define the configuration to restore in the `image/` directory. For example,
`image/miniconda.config.json` for the `server::miniconda::MinicondaConfig` type.

You can add one or many images, and the program will config them one after
another.

## Serializable Image Information

While image models with sensitive values that don't change, like URL domain
names to fetch installers, are secure within Rust's type system, there are data,
like versions, that change and can be safely maintained separately from the
program boundaries.

Serializable information needs special maintenance since it changes frequently
and must not pose security gaps. It is secured thanks to Rust's type system.

For example, if a software version is `SemVer`, and this value goes to the URL
path, the program will ensure the deserialized value is a `SemVer`
type rather than a random string. Moreover, invariants such as domain name,
using the `HTTPS` protocol, etc., are engineered into the System app domain;
thus, volatile serializable data can't affect these protocols and standards.

The current serialization format is `JSON` and image information go to the
`images/` root directory of the app.

When you provide routine maintenance to the app repository, like updating
software versions, PRs will affect the `images/` directory rather than the
application source code, making it relatively scalable since its initial
release, MVP `v0.1.0`.

To find technical details about images, like data types, the modules
`image::server` and `image::desktop` contain their implementations.

The serializable part of an image is the one volatile with minimized control,
where the System app ensures type safety for this boundary. It allows us more
scalability and maintenance, while the app currently supports the format JSON in
its `images/` directory.

### Serialization Examples

Image information will be similar many times, so here are some examples to give
an idea of what they look like.

`Image Serialization of IntelliJIdea JetBrainsIdeImage`

```json
{
  "version": "2024.2.0.1",
  "hash_sha256": "293fa50d4cbae4da55526b72c19650eca26efc8ef3a7107fed2371b70b812d6f"
}
```

You can check the version type, in this case, `YearSemVer`, in the
[corresponding image module](#serializable-image-information) and the `package`
module.

Integrity checks are mandatory when the vendor provides them. Do not forget to
**update the hash value when updating the version** (you get this on the vendor
site), or the download will fail the integrity check.

`Image Serialization of ZoomImage`

```json
{
  "version": "6.1.1.443",
  "public_key_version": "5-12-6",
  "key_fingerprint": "59C8 6188 E22A BB19 BD55 4047 7B04 A1B8 DD79 B481"
}
```

Zoom is a bit peculiar since it requires `Gpg` to verify the file integrity. The
System app supports both `Sha256` hash and `Gpg` verification.

Most image information files consist of the software version and integrity data
from the vendor site to perform a secure download.

## Automated Operations with Super User Requirements

While the System app automates software operations, it's crucial to recall that
*your terminal* will ask for your `sudo` password when required. It's
unnecessary and discouraged to run the app with `sudo` privileges.

So, the only manual interaction you can expect when using the app is to enter
your `sudo` password to your terminal, if required.

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

## Building for Debian

The crate [cargo-deb](https://crates.io/crates/cargo-deb) creates the `deb`
installer from the Rust project. Install it in your system via
`cargo install cargo-deb`.

To create the executable run `cargo deb` in the project directory, and the tool
will generate it in the `target/debian` directory.

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
