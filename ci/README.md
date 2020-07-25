
# Mango - continuous integration

## Principles

* As much automatic validation as possible.
* Different levels of automated testing for different actions, to balance stability and bottleneck prevention.
* Operations should run inside containers as much as possible.
* Steps should be individual files in commonly available languages (preferably Bash, possibly Python or Rust). Because:
    * It is easy to run locally.
    * Not tightly coupled to any CI tool.
    * Readable by more people.
* No hard guidelines for code coverage or performance, but both should not be sacrificed lightly.

## Pipelines

> Note: some of this is not implemented yet.

* `Basic`: The code can be manually tested.
    * It compiles without errors on the test platform.
    * Quick inspections are run, but non-blocking.
        * Build warnings.
        * Unit tests.
        * Lint (clippy).
        * Style (rustfmt).
* `Deploy`: Deploy an environment for manual testing.
    * _This is a plan for the future._
* `Stable`: The code can be merged to master.
    * The quick checks from `Basic`.
    * Run tests with miri.
    * Integration tests.
    * Fuzzing tests.
    * _Maybe? performance test._
    * Documentation builds (no warning).
    * Manual testing (if testable changes).
    * Code review (if reviewer can be found).
* `Dependency`: Non-Mango code is safe and compatible.
    * For each dependency:
        * Up-to-date.
        * No known vulnerabilities.
        * It compiles without warnings (already checked).
        * License is compatible.
    * Runs periodically; dependencies get outdated without Mango change.
* `Release`: The code can be distributed, artifacts are generated.
    * `Stable` and `Dependency` tests must have succeeded.
    * The code compiles on multiple platforms.
    * Artifacts are generated:
        * Readme, license.
        * Read-only docker image containing executable.
        * _Maybe? executable for some platforms._
        * Release notes.
        * Full documentation.
        * Performance results, flamegraph.
        * Test coverage.
        * Source code and git hash.
        * Dependency tree.
        * Binary size breakdown.
        * Semver verification.
        * Overview of code marked 'unsafe'.
    * Can be triggered manually, even if not intending to release.

