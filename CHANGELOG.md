# 1.0.0 (2025-08-28)


### Bug Fixes

* Remove redundant verbose flag from cargo bundle command ([f9d919e](https://github.com/arian24b/cpusage/commit/f9d919ef0f7abe7341687a1d54e69c013ba4acaf))
* Simplify semantic release step and consolidate upload process in GitHub Actions workflow ([1ac4105](https://github.com/arian24b/cpusage/commit/1ac41059edf0dbf6ff95ea6f3261a05cdf44237c))
* Update build process to include cargo-bundle and correct app packaging path ([e222448](https://github.com/arian24b/cpusage/commit/e222448ab7e12c7d0fff050d8fddeb6afea20722))


### Features

* Add application icon conversion and packaging steps in GitHub Actions workflow ([6f16d17](https://github.com/arian24b/cpusage/commit/6f16d17ff6c3c077dd7ea89728121fedec964691))
* Enhance GitHub Actions workflow for Rust app with testing and packaging steps ([74608c5](https://github.com/arian24b/cpusage/commit/74608c538caf1e8e2f9e20cbbe75b911563a818f))
* Implement automated versioning and changelog generation with semantic-release ([ca8235f](https://github.com/arian24b/cpusage/commit/ca8235f839fb95be5e617fa7cad822669e1ed360))
* Refactor GitHub Actions workflow for Rust app release process ([ca595bb](https://github.com/arian24b/cpusage/commit/ca595bb1cd0bbcde93b9b1d7a97685118f5e27a1))

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added
- Initial release of CPUsage - a macOS status bar CPU usage monitor
- Real-time CPU usage display in menu bar
- Custom app icon
- Automated build and release workflow

### Technical Details
- Built with Rust for optimal performance
- Uses macOS native APIs for status bar integration
- Optimized for minimal resource usage
