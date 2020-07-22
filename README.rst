
.. image:: https://github.com/mangolang/cli/workflows/Check%20Mango%20CLI/badge.svg?branch=master
    :target: https://github.com/mangolang/cli/actions

.. image:: https://deps.rs/repo/github/mangolang/cli/status.svg
    :target: https://deps.rs/repo/github/mangolang/cli

.. image:: https://readthedocs.org/projects/mangolang/badge/?version=latest
    :target: https://docs.mangocode.org/en/latest/

.. image:: https://img.shields.io/badge/License-Apache%202.0-blue.svg
    :target: https://opensource.org/licenses/Apache-2.0


Mango CLI
===============================

This is the command-line interface for Mango, a programming language to help you make large software projects reliable, for browsers and servers.

This tool is the main entrypoint for interacting with Mango, even though the actual code is largely divided over different repositories, like frontend_, IR_, to be compiled_, to WebAssembly or interpreted_.

https://mangocode.org/

Status
-------------------------------

This project is still in early development stage. It is not ready to use, not even experimentally.

How to use
-------------------------------

There are two main ways to use the CLI [note: not finished], Docker and self-compiled. Because Mango is in early development, there are no pre-compiled binaries yet.

Docker
...............................

To use the Mango Docker image, you will need Docker installed.

TODO: this image is not available yet

We can then start Mango in a Docker container, mounting your code directory so it can be compiled::

    docker run --rm -it --name mango --mount type=bind,src=/YOUR/CODE/PATH,dst=/code mangocode/mango:latest -- mango --help

Or a short version without name, auto-delete and version::

    docker run -it -v"/YOUR/CODE/PATH":/code mangocode/mango mango --help

Self-compiled
...............................

Compiling and running the code should be easy.

* `Install Rust`_.
* Download and compile Mango:

    cargo install mango

* Run the compiler at `$HOME/.cargo/bin/mango --help`.

If you want to access the executable more easily, either move `mango` to e.g. `/usr/local/bin` (for all users), or add `$HOME/.cargo/bin` to your `PATH` (just for you).

Links
-------------------------------

* `Official website`_
* `Documentation`_
* `Code of conduct and contributing`_

.. _Official website: https://mangocode.org/
.. _`Documentation`: https://docs.mangocode.org/
.. _`Code of conduct and contributing`: https://github.com/mangolang/mango
.. _frontend: https://github.com/mangolang/compiler
.. _IR: https://github.com/mangolang/mango_ir
.. _compiled: https://github.com/mangolang/wasm
.. _interpreted: https://github.com/mangolang/interpreter
.. _Install Rust: https://rustup.rs/
