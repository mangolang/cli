
# Mango - continuous integration

## Principles

* 
* Operations should run inside containers as much as possible.
* Steps should be individual files in commonly available languages (preferably Bash, possibly Python or Rust).
* Code 

## Pipelines

> Note: some of this is not implemented yet.

* `Basic`: The code can be manually tested.
    * It compiles without errors on the test platform.
    * Quick inspections are run, but non-blocking.
        * Build warnings.
        * Unit tests.
        * Lint (clippy).
        * Style (rustfmt).
* `Stable`: The code can be merged to master.
    * The quick checks from `Basic`.
    * Integration tests.
    * _Maybe? benchmarks._
    * Documentation builds (no warning).
    * Manual testing (if testable changes).
    * Code review (if reviewer can be found).
* `Dependency`:
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
        * Docker image containing executable.
        * _Maybe? executable for some platforms._
        * Full documentation.
        * Performance results, flamegraph.
        * Test coverage.
        * Source code and git hash.
        * Dependency tree.
        * Binary size breakdown.
        * Semver verification.
        * Overview of code marked 'unsafe'.

