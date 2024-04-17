# Rufl Contributing Guide

Hi! Thank you for choosing rufl.

rufl is a efficient, reusable util function library of rust. It makes rust dev easier by taking the hassle out of working with collection, math, file, string, etc.

We are excited that you are interested in contributing to rufl. Before submitting your contribution though, please make sure to take a moment and read through the following guidelines.

## Issue Guidelines

- Issues are exclusively for bug reports, feature requests and design-related topics. Other questions may be closed directly.

- Before submitting an issue, please check if similar problems have already been issued.

- Please specify which version of rufl and rust you are using, and provide OS information.

## Pull Request Guidelines

- Fork this repository to your own account. Do not create branches here.

- Commit info should be formatted as `type(scope): info about commit`. eg. `fix(mod): [scrollbar] fix xxx bug`.

  1. type: type should be one of [chore, docs, feat, fix, refactor, release, test].

  2. scope: scope should be one of [mod, file, internal].

  3. header: header should not be longer than 72 characters.

- Rebase before creating a PR to keep commit history clear.

- Before submitting a PR, please execute the unit test command: `cargo test --verbose` to ensure that all unit test tasks should pass.

- Make sure PRs are created to `rc` branch instead of other branch.

- If your PR fixes a bug, please provide a description about the related bug.

- If the PR is for a new feature, make sure to complete the unit test doc test section.
