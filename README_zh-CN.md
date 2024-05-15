<div align=center>
<img src="./logo.png" width="396" height="215"/>
<br/>
</div>

#### Rufl是一个Rust语言工具函数库。它提供了许多工具函数，让你的rust开发变得更简单.

_Rufl是”rust util function"的缩写。_


[![crates.io](https://img.shields.io/crates/v/rufl?label=latest)](https://crates.io/crates/rufl)
[![Documentation](https://docs.rs/rufl/badge.svg?version=0.1.3)](https://docs.rs/rufl/0.1.3)
![MSRV](https://img.shields.io/badge/rustc-1.71+-ab6000.svg)
[![CI](https://github.com/duke-git/rufl/actions/workflows/ci.yml/badge.svg?branch=main)](https://github.com/duke-git/rufl/actions/workflows/ci.yml)
[![codecov](https://codecov.io/gh/duke-git/rufl/branch/main/graph/badge.svg)](https://app.codecov.io/gh/duke-git/rufl/tree/main)
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
rufl = "0.1.3"
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

## 文档

### <span id="index">目录<span>

- [collection](#collection)
- [eventbus](#eventbus)
- [file](#file)
- [math](#math)
- [random](#random)
- [string](#string)


<h3 id="collection">1. Collection mod包含用于操作集合数据类型的工具函数。 <a href="#index">回到目录</a></h3>

```rust
use rufl::collection;
```

- **all_match:** 如果集合的所有元素都通过判断函数检查，则返回true。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.all_match.html)]

- **chunk:** 按照size参数均分vector。[[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.chunk.html)]

- **count:** 返回给定元素在集合中出现的次数。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.count.html)]

- **count_by:** 使用判断函数迭代集合的元素，返回所有匹配元素的数量。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.count_by.html)]

- **difference:** 比较两个集合元素，返回不相等的元素集合 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.difference.html)]

- **difference_by:** 将两个集合中的每个元素调用比较函数，并比较它们的返回值，如果不相等返回在第一个参数集合中对应的值。[[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.difference_by.html)]

- **difference_with:** 接受比较器函数，该比较器被调用以将两个集合的元素进行比较。结果值的顺序和引用由第一个集合确定。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.difference_with.html)]

- **fill:** 用初始值填充vector的元素。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.fill.html)]

- **filter:** 迭代集合的元素，返回通过判断函数的所有元素的集合。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.filter.html)]

- **filter_map:** 返回一个对给定集合应用过滤和映射的集合。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.filter_map.html)]

- **find:** 迭代集合的元素，返回符合判断函数的第一个元素及其索引。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.find.html)]

- **find_last:** 迭代集合的元素，返回传递判断函数的最后一个元素及其索引。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.find_last.html)]

- **index_of:** 返回在集合中找到第一次出现的元素的索引。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.index_of.html)]

- **insert_at:** 在vector内的位置索引处插入一个元素。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.insert_at.html)]

- **intersection:** 创建所有集合中包含的唯一元素的vector。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.intersection.html)]

- **is_ascending_order:** 检查集合中所有元素是否按升序排列。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.is_ascending_order.html)]

- **is_descending_order:** 检查集合中所有元素是否按降序排列。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.is_descending_order.html)]

- **is_sorted:** 检查集合中的所有元素是否已排序（升序或降序）。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.is_sorted.html)]

- **last_index_of:** 返回在集合中找到最后一次出现的元素的索引。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.last_index_of.html)]

- **map:** 通过iteratee迭代器函数调用集合中的每个元素来创建新的元素集合。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.map.html)]

- **max:** 返回集合元素的最大值。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.max.html)]

- **min:** 返回集合元素的最小值。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.min.html)]

- **none_match:** 如果集合中没有元素通过判断函数检查，则返回true。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.none_match.html)]

- **partition:** 通过给定判断函数的评估对集合元素进行分区。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.partition.html)]

- **reduce:** 对集合元素执行reduce操作。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.reduce.html)]

- **reduce_right:** 类似Reduce操作，迭代切片元素顺序从右至左。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.reduce_right.html)]

- **remove_all:** 删除vector内的所有特定元素。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.remove_all.html)]

- **replace_all:** 将vector中的所有旧元素替换为新元素 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.replace_all.html)]

- **replace_n:** 将vector中的前n个旧元素替换为新元素。[[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.replace_n.html)]

- **shuffle:** 返回打乱值的vector。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.shuffle.html)]

- **some_match:** 如果集合的任何一个元素通过判断函数检查，则返回true。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.some_match.html)]

- **union:** 在所有集合之间创建唯一元素的vector。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.union.html)]

- **union_by:** 在两个集合之间创建唯一元素的vector。 接受为每个集合的每个元素调用的判断函数，以生成计算唯一性的标准。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.union_by.html)]

- **unique:** 删除集合（array、vector）中的重复元素，使用 PartialEq 相等比较。 [[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.unique.html)]

- **unique_by:** 使用集合元素调用提供的自定义比较函数，返回唯一元素的vector。[[doc](https://docs.rs/rufl/0.1.3/rufl/collection/fn.unique_by.html)]


<h3 id="eventbus">2. Eventbus实现了一个简单的发布/订阅事件库。<a href="#index">回到目录</a></h3>

```rust
use rufl::eventbus;
```

- **Event:** 事件是一个可以保存任何数据类型的结构。然后将其发布到事件总线。一旦发布，事件就会在事件总线运行时传递给每个订阅者。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/struct.Event.html)]

- **EventBus:** Eventbus是所有事件的中心枢纽。 它负责管理所有订阅者并发布与事件总线相关的事件。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/struct.EventBus.html)]


<h3 id="file">3. File mod包括文件操作相关工具函数。<a href="#index">回到目录</a></h3>

```rust
use rufl::file;
```
- **clear:** 清空文件内容。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/fn.clear.html)]

- **copy_dirs:** 将src路径中的所有目录复制到dest路径。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/fn.copy_dirs.html)]

- **create:** 在指定路径中创建一个文件并返回它。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/fn.create.html)]

- **file_names:** 返回特定目录路径的所有文件名。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/fn.file_names.html)]

- **get_md5:** 获取文件md5值。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/fn.get_md5.html)]

- **is_symlink:** 检查文件是否是符号链接文件。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/fn.is_symlink.html)]

- **read_to_buffer:** 读取文件到缓冲区字节数组。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/fn.read_to_buffer.html)]

- **read_to_lines:** 读取文件并返回行字符串vector。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/fn.read_to_lines.html)]

- **read_to_string:** 将文件读取为字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/fn.read_to_string.html)]

- **write_to:** 将数据写入文件，如果文件不存在，则创建它。 [[doc](https://docs.rs/rufl/0.1.3/rufl/eventbus/fn.write_to.html)]


<h3 id="math">4. Math mod包括一些数学计算函数。<a href="#index">回到目录</a></h3>

```rust
use rufl::math;
```
- **abs:** 计算绝对值。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.abs.html)]

- **average:** 计算平均数。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.average.html)]

- **factorial:** 计算阶乘。[[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.factorial.html)]

- **fib_nth:** 计算斐波那契数列的第n个值。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.fib_nth.html)]

- **fib_seq:** 返回斐波那契数列。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.fib_seq.html)]

- **fib_sum:** 计算斐波那契数列的总和值。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.fib_sum.html)]

- **gcd:** 计算最大公约数。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.gcd.html)]

- **harmonic:** 计算谐波值。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.harmonic.html)]

- **is_prime:** 判断素数。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.is_prime.html)]

- **lcm:** 计算最小公倍数。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.lcm.html)]

- **percent:** 计算百分比。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.percent.html)]

- **round:** 将小数点后n位四舍五入为数字。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.round.html)]

- **round_down:** 向下四舍五入并截去n位小数。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.round_down.html)]

- **round_up:** 对数字进行四舍五入并截去n位小数。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.round_up.html)]

- **sqrt:** 计算开平方。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.sqrt.html)]

- **sum:** 计算和值。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.sum.html)]

- **to_angle:** 弧度转角度。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.to_angle.html)]

- **to_radian:** 角度转弧度。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.to_radian.html)]

- **truncate:** 将数字截断至小数点后n位。 [[doc](https://docs.rs/rufl/0.1.3/rufl/math/fn.truncate.html)]



<h3 id="random">5. Random mod用于生成随机字符串和数字。<a href="#index">回到目录</a></h3>

```rust
use rufl::random;
```

- **alpha_number:** 生成随机字母或数字字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/random/fn.alpha_number.html)]

- **alphabet:** 生成随机英文字母字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/random/fn.alphabet.html)]

- **lower:** 生成随机小写英文字母字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/random/fn.lower.html)]

- **numberic:** 生成随机数字字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/random/fn.numberic.html)]

- **symbol:** 生成随机特殊字符字符串。 (!@#$%^&*()_+-=[]{}|;’:",./<>?). [[doc](https://docs.rs/rufl/0.1.3/rufl/random/fn.symbol.html)]

- **string:** 生成随机字符串(包括字母, 数字, 符号)。 [[doc](https://docs.rs/rufl/0.1.3/rufl/random/fn.string.html)]

- **upper:** 生成随机大写英文字母字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/random/fn.upper.html)]



<h3 id="string">6. String mod包括操作字符串(String&str)的工具函数<a href="#index">回到目录</a></h3>

```rust
use rufl::string;
```

- **add_commas:** 从右侧每3个数字向数字值添加逗号。支持前缀符号。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.add_commas.html)]

- **after:** 返回源字符串中第一次出现指定子字符串之后的子字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.after.html)]

- **after_last:** 返回源字符串中最后一次出现指定子字符串之后的子字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.after_last.html)]

- **before:** 返回源字符串中第一次出现指定子字符串之前的子字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.before.html)]

- **before_last:** 返回源字符串中最后一次出现指定子字符串之前的子字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.before_last.html)]

- **camel_case:** 将字符串转换为驼峰式。[[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.camel_case.html)]

- **capitalize:** 将字符串的第一个字符转换为大写，其余字符转换为小写。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.capitalize.html)]

- **count_by:** 使用谓词函数计算目标字符串中的字符数，返回所有匹配字符的数量。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.count_by.html)]

- **count_chars:** 返回目标字符串中的字符数。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.count_chars.html)]

- **count_graphemes:** 返回目标字符串中的字素计数。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.count_graphemes.html)]

- **count_words:** 返回目标字符串中的单词字数。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.count_words.html)]

- **cut:** 在源字符串中搜索子字符串“sep”，并在子字符串“sep”第一次出现时将源字符串分成两部分：之前和之后。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.cut.html)]

- **hide:** 隐藏源字符串中的一些字符并替换为特定的子字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.hide.html)]

- **index:** 搜索字符串并返回指定搜索子字符串第一次出现的索引。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.index.html)]

- **index_all:** 搜索字符串并返回指定搜索子字符串出现的所有索引。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.index_all.html)]

- **is_alpha:** 验证字符串是否是只包含字母字符。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_alpha.html)]

- **is_alphanumberic:** 验证字符串是否是只包含字母和数字字符。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_alphanumberic.html)]

- **is_digit:** 验证字符串是否是指包含数字(0-9)。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_digit.html)]

- **is_dns:** 验证字符串是否是有效的dns。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_dns.html)]

- **is_email:** 验证字符串是否是有效的email地址。[[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_email.html)]

- **is_ipv4:** 验证字符串是否是有效的ipv4地址。[[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_ipv4.html)]

- **is_ipv6:** 验证字符串是否是有效的ipv6地址。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_ipv6.html)]

- **is_lowercase:** 验证字符串是否只包含小写字符。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_lowercase.html)]

- **is_numberic:** 验证字符串是否可以转成有效数字。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_numberic.html)]

- **is_strong_password:** 验证字符串是否是有效密码（强密码）。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_strong_password.html)]

- **is_uppercase:** 验证字符串是否只包含大写字符。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_uppercase.html)]

- **is_url:** 验证字符串是否是有效的url. [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.is_url.html)]

- **kebab_case:** 将字符串转换为短横线大小写。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.kebab_case.html)]

- **last_index:** 搜索字符串并返回指定搜索子字符串最后一次出现的索引。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.last_index.html)]

- **lower_first:** 将字符串的第一个字符转换为小写字符。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.lower_first.html)]

- **pad:** 如果字符串长度短于size，则在左右两侧填充字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.pad.html)]

- **pad_end:** 	如果字符串长度短于size，则在右侧填充字符串。[[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.pad_end.html)]

- **pad_start:** 如果字符串长度短于size，则在左侧填充字符串。[[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.pad_start.html)]

- **pascal_case:** 将字符串转换为帕斯卡大小写。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.pascal_case.html)]

- **remove_all:** 删除源字符串中出现的所有指定子字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.remove_all.html)]

- **remove_first:** 删除源字符串中第一次出现的指定子字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.remove_first.html)]

- **remove_last:** 删除源字符串中最后一次出现的指定子字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.remove_last.html)]

- **removen:** 删除源字符串中的前n个匹配子字符串。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.removen.html)]

- **snake_case:** 将字符串转换为蛇形大小写。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.snake_case.html)]

- **split_chars:** 	将输入字符串拆分为字符vector。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.split_chars.html)]

- **split_graphemes:** 将目标字符串拆分为字素str的vector。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.split_graphemes.html)]

- **split_words:** 将输入字符串拆分为其单词vector（基于大写字母或数字的位置）。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.split_words.html)]

- **starts_with_offset:** 检查字符串是否在偏移位置以指定前缀开头。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.starts_with_offset.html)]

- **substring:** 返回目标字符串中从起始索引到结束索引（不包括结束索引）的部分。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.substring.html)]

- **title_case:** 将字符串转换为标题大小写。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.title_case.html)]

- **upper_first:** 将字符串的第一个字符转换为大写字符。 [[doc](https://docs.rs/rufl/0.1.3/rufl/string/fn.upper_first.html)]



## 如何贡献代码

#### [贡献代码指南](./CONTRIBUTING.zh-CN.md)