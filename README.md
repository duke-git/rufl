<div align=center>
<img src="./logo.png" width="439" height="200"/>
<br/>
</div>

#### Rufl is an util function library for rust. It provides a series of useful functions to make your rust development easier.

_Rufl is short for "rust util function library"._

## [简体中文](./README_zh-CN.md)

## Features

- Powerful: supports commonly used development features, string, collection, random, file... etc.
- Pure: keep external dependencies to a minimum.
- Simple: well structure, test for every function.

## Installation
- cargo add rufl

```toml
[dependencies]
rufl = "0.1.1"
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