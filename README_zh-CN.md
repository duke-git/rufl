<div align=center>
<img src="./logo.png" width="396" height="215"/>
<br/>
</div>

#### Ruf是一个Rust语言工具函数库。它提供了许多工具函数，让你的rust开发变得更简单.

_Ruf是”rust util function"的缩写。_

## [English](./README.md)

## 特性

- 💪 强大: 支持常用开发特性, string, collection, random, file...
- 💎 纯净: 保持最小范围外部依赖。
- 🛠 简洁: 结构组织良好，测试所有API函数。

## 安装
- cargo add ruf

```toml
[dependencies]
ruf = "0.1.1"
```

## 示例

这里以字符串函数`add_commas`为例，该函数的作用是每隔3位数（从右边开始）向数字值中添加逗号，并在前面添加前缀符号。为了实现这个功能，我们需要用到 `ruf::string`模块。

代码:
```rust
use ruf::string;

fn main() {
    let money_amount: String = string::add_commas("1234567", "$");
    println!("current money is {}", money_amount); // current money is $1,234,567
}
```