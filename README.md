<!-- SPDX-License-Identifier: MIT OR Apache-2.0 -->
<!-- SPDX-FileCopyrightText: Ferrous Systems GmbH -->

<br>
<p align="center">
    <img height="75" src="https://github.com/ferrocene/ferrocene/raw/main/ferrocene/logo.svg" alt="Ferrocene Logo: A circle between two planes, representing the chemical compound of the name Ferrocene, with the name written next to it.">
</p>
<br>

Ferrocene is a toolchain to enable the use of the Rust programming language in
safety-critical environments. It is a proper downstream of the main Rust
compiler - rustc, maintained by the Rust project on [`rust-lang/rust`].

The mission of Ferrocene is to bring open source practices to safety-critical
industries and improve the Rust open source ecosystem through safety-critical
practices.

Ferrocene is maintained and supported by the world-renowed experts at Ferrous
Systems. Both standard and long-term support are available. Check [our
website][website] for details.

## Current status

Ferrocene is qualified for ISO 26262 (ASIL D) and IEC 61508 (SIL 4).
Qualification for other standards and areas, such as railway and aerospace,
are planned.

## Installation

Prebuilt Ferrocene binaries are available for customers and partners. You can
visit [releases.ferrocene.dev] to download the release archives after logging
in with your Ferrocene account.

## Documentation and Procedures

The documentation of the Ferrocene toolchain and its source can be found
in the [`ferrocene/doc` directory]. The documentation contains the projects
procedures, its quality management measures and current testing coverage.

Rendered versions of the documentation are also available:

* [docs.ferrocene.dev]: available to customers and partners, contains the
  documentation for all release channels.
* [public-docs.ferrocene.dev/main]: publicly available, contains the
  documentation for the latest commit merged on the `main` branch.

## Support

Multiple levels of support are available for paying customers, provided by
[Ferrous Systems]. You can log into [customers.ferrocene.dev] to learn about
your support plan, and how to send support requests.

## Contribution policy

As a downstream of the Rust project, Ferrocene prefers to keep the compiler
unmodified. This means that general, contributions to the compiler or its tools
and discussions should be kept to [`rust-lang/rust`]. However, Ferrocene does
serve as a community of peers to propose and produce changes useful in
safety-critical changes towards the project.

Contributions to qualification activities and manuals are welcome, but
generally gated. Contribution is open to industry and academic partners,
customers and project employees.

You can use the [Ferrocene issue tracker][issues] to file an issue for the
materials provided by the Ferrocene developers. Please note that the issue
tracker is not a support channel.

Please note that Ferrocene is governed under the Apache-2.0 license and
contribution policies apply to the issue tracker as well as the codebase
itself.

## Additional services

[Ferrous Systems] provides services built and tailored around Ferrocene:

* **Trainings:** trainings on Rust and Ferrocene, particularly for teams, are
  available. Trainings can be custom tailored to your needs. [Check out our
  training offerings.][trainings]

* **Inclusion in SDKs:** Ferrocene is available to be integrated in your
  toolchain! Please [get in touch][contact] to learn more.

* **Tailoring, enabling and integration within your system**: We're more than
  happy to enable Rust support in your operating system or tool, including
  porting the Rust compiler to the targets you need and qualifying them in
  Ferrocene. [Get in touch][contact] to learn more.

* **Infrastructure support**: Ferrocene is built for a DevOps world. Rust
  for your builds in the cloud is a first-class citizen for us, and we can
  provide support tailored to you. [Get in touch][contact] for more
  information.

## Security

Please follow [Ferrocene's security policy][security] if you discover a
security vulnerability affecting Ferrocene.

## License and trademark

The contents of the repository are primarily licensed under either the MIT or
Apache 2.0 license: users can choose either license, and contributors must
license their changes under both licenses. Note that the repository contains
files authored by third parties and published under different licenses, see the
annotations next to those files.

Ferrocene is a registered trademark of Critical Section GmbH, a subsidiary of
Ferrous Systems. See our [trademark policy][trademark] for the guidelines on
the use of the trademark.

[issues]: https://github.com/ferrocene/ferrocene/issues
[trainings]: https://ferrous-systems.com/training
[contact]: https://ferrous-systems.com/contact#ferrocene
[trademark]: ./TRADEMARK.md
[security]: https://github.com/ferrocene/ferrocene/security/policy
[website]: https://ferrous-systems.com/ferrocene

[Ferrous Systems]: https://ferrous-systems.com
[releases.ferrocene.dev]: https://releases.ferrocene.dev
[docs.ferrocene.dev]: https://docs.ferrocene.dev
[public-docs.ferrocene.dev/main]: https://public-docs.ferrocene.dev/main
[customers.ferrocene.dev]: https://customers.ferrocene.dev

[`rust-lang/rust`]: https://github.com/rust-lang/rust
[`ferrocene/doc` directory]: ferrocene/doc/
