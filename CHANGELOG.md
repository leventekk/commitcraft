# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v0.9.6] - 2024-08-19
### :sparkles: New Features
- [`14539a8`](https://github.com/leventekk/commitcraft/commit/14539a870c566bbe85cd24af026c4bc750736bb8) - **instruction**: ignore some files to get more accurate commit message *(commit by [@leventekk](https://github.com/leventekk))*
- [`a14df0f`](https://github.com/leventekk/commitcraft/commit/a14df0f6cf7b842c7692fca9464e557fdd6c51ae) - **package**: update version to 0.9.6 *(commit by [@leventekk](https://github.com/leventekk))*

### :bug: Bug Fixes
- [`180474a`](https://github.com/leventekk/commitcraft/commit/180474ab43afc95e1c5b890baf6d7d9504589040) - **generator**: update model to Gpt4 and improve request handling *(PR [#1](https://github.com/leventekk/commitcraft/pull/1) by [@leventekk](https://github.com/leventekk))*

### :wrench: Chores
- [`e0d99e6`](https://github.com/leventekk/commitcraft/commit/e0d99e6aea67d9fc48a40132fd16cf09cd8cabe8) - **tests**: add some basic test cases for executor *(commit by [@leventekk](https://github.com/leventekk))*
- [`9e9f2a8`](https://github.com/leventekk/commitcraft/commit/9e9f2a86fa0d54078729c9d57a62dade804edb6a) - update code style *(commit by [@leventekk](https://github.com/leventekk))*


## [v0.9.5] - 2024-05-16
### :sparkles: New Features
- [`ac1973c`](https://github.com/leventekk/commitcraft/commit/ac1973cf1e1ec27d26340c5c82bcc7c5a338e5c2) - **config**: remove add_description field and related functionality from AppConfig and ConfigOptions *(commit by [@leventekk](https://github.com/leventekk))*
- [`57013cd`](https://github.com/leventekk/commitcraft/commit/57013cdda3281d733b3583aff276e767afebfcf0) - **readme**: add CommitCraft logo and update description *(commit by [@leventekk](https://github.com/leventekk))*
- [`50f1853`](https://github.com/leventekk/commitcraft/commit/50f185357979eeeeafa99da1a66f11a28551731c) - **commitcraft**: update version to 0.9.5 in Cargo.toml and Cargo.lock *(commit by [@leventekk](https://github.com/leventekk))*

### :bug: Bug Fixes
- [`9dc7963`](https://github.com/leventekk/commitcraft/commit/9dc796398c2be6ba0589bc9e599cfe9c887feddc) - **generator**: handle error response from API when generating message *(commit by [@leventekk](https://github.com/leventekk))*


## [v0.9.4] - 2024-05-09
### :sparkles: New Features
- [`260ac70`](https://github.com/leventekk/commitcraft/commit/260ac70fc2eb273e3dffbf4b63ce71fa55693628) - **actions**: add backup functionality to recover and destroy commit messages *(commit by [@leventekk](https://github.com/leventekk))*
- [`98d07fc`](https://github.com/leventekk/commitcraft/commit/98d07fc708cc060997d2370adfe0186cc3b07c15) - **package**: update commitcraft to version 0.9.4 *(commit by [@leventekk](https://github.com/leventekk))*


## [v0.9.3] - 2024-05-03
### :sparkles: New Features
- [`e28a00d`](https://github.com/leventekk/commitcraft/commit/e28a00d111623f66a7e481929c6693229092a031) - **actions**: add the ability to track changes in the configuration *(commit by [@leventekk](https://github.com/leventekk))*
- [`c377b3f`](https://github.com/leventekk/commitcraft/commit/c377b3fc6c63d23bd895362cb71fecdd418e3601) - **actions**: update GenerateMessageCommand to handle different commit message formats *(commit by [@leventekk](https://github.com/leventekk))*
- [`a6f9133`](https://github.com/leventekk/commitcraft/commit/a6f9133d42603220a9b07d0e2c21e58e1e05b0f1) - **actions/generate_message**: improve message generation process *(commit by [@leventekk](https://github.com/leventekk))*
- [`b1737bd`](https://github.com/leventekk/commitcraft/commit/b1737bdcfbe7734ae7b897325ebb763756339102) - **actions**: improve handling of empty stashed files *(commit by [@leventekk](https://github.com/leventekk))*
- [`c09dcf6`](https://github.com/leventekk/commitcraft/commit/c09dcf6a1a949386143d5e72d2d93d953377d72c) - **instructions**: improve commit generation prompt and guidance *(commit by [@leventekk](https://github.com/leventekk))*
- [`0734963`](https://github.com/leventekk/commitcraft/commit/07349631da1a33aa426dbb814010bc8d794d61c6) - **actions**: improve error handling in commit_changes *(commit by [@leventekk](https://github.com/leventekk))*

### :bug: Bug Fixes
- [`a8f93db`](https://github.com/leventekk/commitcraft/commit/a8f93db70e4ae0cb06838b497e977d242529f8ba) - **Cargo.toml**: update package version to 0.9.3 *(commit by [@leventekk](https://github.com/leventekk))*


## [v0.9.2] - 2024-04-30
### :sparkles: New Features
- [`edfdf64`](https://github.com/leventekk/commitcraft/commit/edfdf640d55b0c395362e533c8b4c89944b54651) - **workspace**: add pre-build step for aarch64-unknown-linux-gnu target *(commit by [@leventekk](https://github.com/leventekk))*

### :wrench: Chores
- [`a4025e6`](https://github.com/leventekk/commitcraft/commit/a4025e6cac2fb7be47e84a1dfb66d3aae741dc50) - update commitcraft to version 0.9.2 *(commit by [@leventekk](https://github.com/leventekk))*


## [v0.9.1] - 2024-04-30
### :wrench: Chores
- [`52fd988`](https://github.com/leventekk/commitcraft/commit/52fd988d4da1e9f937e07030df07ba1ebfa7f46d) - update version to 0.9.1 *(commit by [@leventekk](https://github.com/leventekk))*

[v0.9.1]: https://github.com/leventekk/commitcraft/compare/v0.9.0...v0.9.1
[v0.9.2]: https://github.com/leventekk/commitcraft/compare/v0.9.1...v0.9.2
[v0.9.3]: https://github.com/leventekk/commitcraft/compare/v0.9.2...v0.9.3
[v0.9.4]: https://github.com/leventekk/commitcraft/compare/v0.9.3...v0.9.4
[v0.9.5]: https://github.com/leventekk/commitcraft/compare/v0.9.4...v0.9.5
[v0.9.6]: https://github.com/leventekk/commitcraft/compare/v0.9.5...v0.9.6
