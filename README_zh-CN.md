<div align=center>
<img src="./logo.png" width="396" height="215"/>
<br/>
</div>

#### Rufl是一个Rust语言工具函数库。它提供了许多工具函数，让你的rust开发变得更简单.

_Rufl是”rust util function"的缩写。_


[![crates.io](https://img.shields.io/crates/v/rufl?label=latest)](https://crates.io/crates/rufl)
[![Documentation](https://docs.rs/rufl/badge.svg?version=0.1.2)](https://docs.rs/rufl/0.1.2)
![MSRV](https://img.shields.io/badge/rustc-1.71+-ab6000.svg)
[![CI](https://github.com/duke-git/rufl/actions/workflows/coverage.yml/badge.svg?branch=main)](https://github.com/duke-git/rufl/actions/workflows/coverage.yml)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](https://github.com/duke-git/rulf/blob/main/LICENSE)

## [English](./README.md)

## 特性

- 💪 强大: 支持常用开发特性, string, collection, random, file...
- 💎 纯净: 保持最小范围外部依赖。
- 🛠 简洁: 结构组织良好，测试所有API函数。

## 安装
- cargo add rufl

```toml
[dependencies]
rufl = "0.1.2"
```

## 示例

这里以字符串函数`add_commas`为例，该函数的作用是每隔3位数（从右边开始）向数字值中添加逗号，并在前面添加前缀符号。为了实现这个功能，我们需要用到 `rufl::string`模块。

代码:
```rust
use rufl::string;

fn main() {
    let money_amount: String = string::add_commas("1234567", "$");
    println!("current money is {}", money_amount); // current money is $1,234,567
}
```