<div align=center>
<img src="./logo.png" width="439" height="200"/>
<br/>
</div>

#### Rufl is an util function library for rust. It provides a series of useful functions to make your rust development easier.

_Rufl is short for "rust util function library"._

[![crates.io](https://img.shields.io/crates/v/rufl?label=latest)](https://crates.io/crates/rufl)
[![Documentation](https://docs.rs/rufl/badge.svg?version=0.1.2)](https://docs.rs/rufl/0.1.2)
![MSRV](https://img.shields.io/badge/rustc-1.71+-ab6000.svg)
[![CI](https://github.com/duke-git/rufl/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/duke-git/rufl/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/duke-git/rufl/branch/master/graph/badge.svg)](https://app.codecov.io/gh/duke-git/rufl/tree/main)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/duke-git/rulf/blob/main/LICENSE)

## [简体中文](./README_zh-CN.md)

## Features

- Powerful: supports commonly used development features, string, collection, random, file... etc.
- Pure: keep external dependencies to a minimum.
- Simple: well structure, test for every function.

## Installation
- cargo add rufl

```toml
[dependencies]
rufl = "0.1.2"
```

## Example

Here takes the string function `add_commas` (Add comma to a number value by every 3 numbers from right. Ahead by prefix symbol.) as an example, and the `rufl::string` mod needs to be used.

Code:
```rust
use rufl::string;

fn main() {
    let money_amount: String = string::add_commas("1234567", "$");
    println!("current money is {}", money_amount); // current money is $1,234,567
}
```

## How to Contribute

#### [Contributing Guide](./CONTRIBUTING.md)