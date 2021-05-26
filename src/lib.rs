/*!
``occlib`` is The Open Cinema Collective's python library template. To build your own
documentation, simply start using it. Below we will show some example documentation with
the basic functions of this library template.

Demo
====

```rust
use occlib::cast_spell;

assert_eq!(cast_spell(), "Avada Kedavra")
```

Occlib comes with a number of pre-built quality-of-life macros for developers so they
can code more and manage less, most of which are accessed through ``make`` commands.

In addition to your lib's package folder, occlib has two other main directories:

   * ``./zdevelop`` - where maintenance scripts, and other information is stored

In addition, the following files are used:

   * ``./README.md`` - brief description and pointer to doc link for [Github](http://Githib.com)
   * ``./setup.cfg`` - when possible, settings for all tools are stored here.
   * ``./tarpaulin.toml`` - configuration for code coverage with [tarpaulin](https://github.com/xd009642/tarpaulin).
   * ``./Makefile`` - contains make commands for the development features detailed in
     this doc.
   * ``./azure_pipelines.yml`` - build process definition for [Azure Pipelines](https://dev.azure.com/peake100/Open%20Cinema%20Collective/_build).

Setting up your Library
=======================

Getting started is easy, just follow the below steps. Many of these steps include
``Make`` scripts that help you get up and running quickly. To run the ``Make`` commands,
ensure that the active directory of your terminal session is ``"occlib-py"``

1. Clone occlib-go from Github
--------------------------------

navigate to where you wish to keep your project in terminal: ::

```shell
cd /path/to/local_repository
git clone https://github.com/opencinemac/occlib-rs.git
```

once the library is cloned, move into it as your active directory:

```shell
cd occlib-rs
```

2. Pick a Name
--------------

When you have chosen a name for your new lib, update the Cargo.toml.

3. Pick a Description
---------------------

In the ``./Cargo.toml`` file, under the ``[metadata]`` header, change the ``description``
field to a brief description of your project.

4. Initialize a new git repo
----------------------------

You should delete the existing ``.git`` folder for the repository, then initialize a
clean repo by typing:

```shell
git init
```

In the future, you may wish to cherry-pick commits / updates to this template into
your own libraries. A guide for how to do that can be found here:

[Guide needs to be written]

5. Register your library
------------------------

Please reference the relevant documentation for registering your library in Github,
Azure Pipelines, etc. Links to relevant guides can be found below:

[Guides need to be written]

Writing Your Library
====================

1. Style
--------

The Open Cinema Collective's style guide is simple and straightforward:

   1. [rustfmt](https://github.com/rust-lang/rustfmt) first
   2. [clippy](https://github.com/rust-lang/rust-clippy) second
   3. When 1 & 2 contradict: see 1

This keeps things simple, but consistent.

2. Lint
-------

To check the formatting of your library, type:

```shell
make lint
```

This will run the following tools to tell you where adjustments need to be made:

   * [rustfmt](https://github.com/rust-lang/rustfmt)
   * [clippy](https://github.com/rust-lang/rust-clippy)
   * [Misspell](https://github.com/client9/misspell)

3. Re-format
------------

Strict pep8 and Black adherence, while useful in many ways to the organization, can be
annoying and distracting to individual engineers. To help with this, the occlib
template comes with tools to re-format your code for you.

To re-format your code, type:

```shell
make format
```

This will run the following tools:

   * [rustfmt](https://github.com/rust-lang/rustfmt)

With these tools, keeping your code properly formatted is minimally invasive, and as an
organization will lead to a more consistent, maintainable codebase.

4. Test
-------

To run your tests type:

```shell
make test
```

5. Document
-----------

occlib uses the built-in cargo tools to making documentation. To create docs for your library,
type:

```shell
make doc
```

Deploying Your Library
======================

1. Make Commits:
----------------
Make your commits as you work. Your commits will be made to the ``dev`` branch, changes
are pushed to master automatically once builds are passed.

2. Version:
-----------

The major / minor version of the library are set in the ``setup.cfg`` file under
``version:target``.

Patch versions are generated automatically by the build system. So if ``version:target``
is ``1.2`` and the last published build was ``1.2.4`` the next build created will
become ``1.2.5``.

When a new major / minor version bump is desired, simply change the ``version:target``
value, ie ``1.3`` or ``2.0``.

3. Push:
--------

When you are ready, push your code to github. This will set off a chain of events that
will:

   * automatically run formatting and unit tests
   * if tests are passed, build and push your library to be available to other developers
   * uploads a new version to [crates.io](http://crates.io)

4. Build:
---------

occlib uses [Azure Pipelines](https://dev.azure.com/peake100/Open%20Cinema%20Collective/_build) to
automatically run builds.

For more information on azure builds, see the
[azure build templates repo](https://github.com/opencinemac/azure-pipelines-templates).

Other Quality of Life Development Functions
===========================================

1. Scratch Folder
-----------------

The folder ``zdevelop/scratch`` is included in .gitignore, so you can store scratch work
to do quick tests in this directory without accidentally causing a commit conflict.
!*/

/// casts a spell
pub fn cast_spell() -> &'static str {
    "Avada Kedavra"
}

#[cfg(test)]
mod tests {
    use crate::cast_spell;

    #[test]
    fn test_spell() {
        assert_eq!("Avada Kedavra", cast_spell())
    }
}
