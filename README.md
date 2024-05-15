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
[![codecov](https://codecov.io/gh/duke-git/rufl/branch/main/graph/badge.svg)](https://app.codecov.io/gh/duke-git/rufl/tree/main)
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

## Documentation

### <span id="index">Index<span>

- [collection](#collection)
- [eventbus](#eventbus)
- [file](#file)
- [math](#math)
- [random](#random)
- [string](#string)

<h3 id="collection">1. Collection mod contains several utility functions to manipulate collection data type. <a href="#index">index</a></h3>

```rust
use rufl::collection;
```

- **all_match:** Returns true if all elements of the collection pass the predicate function check. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.all_match.html)]

- **chunk:** Returns a vector of elements split into groups the length of size. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.chunk.html)]

- **count:** Returns the number of occurrences of the given element in the collection. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.count.html)]

- **count_by:** Iterates over elements of collection with predicate function, returns the number of all matched elements. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.count_by.html)]

- **difference:** Creates a vector of values not included in the given collections using equality comparisons. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.difference.html)]

- **difference_by:** Like difference except that it accepts iteratee which is invoked for each element of collection and values to generate the criterion by which they’re compared. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.difference_by.html)]

- **difference_with:** Like difference except that it accepts comparator which is invoked to compare elements of collection to values. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.difference_with.html)]

- **fill:** Fills elements of vector with initial value. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.fill.html)]

- **filter:** Iterates over elements of collection, returning a collection of all elements pass the predicate function. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.filter.html)]

- **filter_map:** Returns a collection which apply both filtering and mapping to the given collection. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.filter_map.html)]

- **find:** Iterates over elements of collection, returning the first one and its index that pass predicate function. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.find.html)]

- **find_last:** Iterates over elements of collection, returning the last one and its index that pass predicate function. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.find_last.html)]

- **index_of:** Returns the index at which the first occurrence of a element is found in the collection. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.index_of.html)]

- **insert_at:** Inserts an element at position index within the vector. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.insert_at.html)]

- **intersection:** Creates a vector of unique elements that included by the all collections. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.intersection.html)]

- **is_ascending_order:** Checks if all elements are in ascending order within collection. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.is_ascending_order.html)]

- **is_descending_order:** Checks if all elements are in descending order within collection. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.is_descending_order.html)]

- **is_sorted:** Checks if all elements are sorted(ascending or descending order) within collection. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.is_sorted.html)]

- **last_index_of:** Returns the index at which the last occurrence of a element is found in the collection. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.last_index_of.html)]

- **map:** Creates new collection of element by running each element in collection thru iteratee. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.map.html)]

- **max:** Returns the maximum value of a collection. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.max.html)]

- **min:** Returns the minimum value of a collection. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.min.html)]

- **none_match:** Returns true if there is no element of the collection pass the predicate function check. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.none_match.html)]

- **partition:** Partition collection elements with the evaluation of the given predicate function. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.partition.html)]

- **reduce:** Reduces collection to a value which is the accumulated result of running each element in collection thru iteratee [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.reduce.html)]

- **reduce_right:** Reduce right like reduce except that it iterates over elements of collection from right to left. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.reduce_right.html)]

- **remove_all:** Remove all specific elements within the vector. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.remove_all.html)]

- **replace_all:** Replace all old items with new items within the vector. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.replace_all.html)]

- **replace_n:** Replace the first count n old elements with new elements in the vector. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.replace_n.html)]

- **shuffle:** Returns a vector of shuffled values. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.shuffle.html)]

- **some_match:** Returns true if any element of the collection pass the predicate function check. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.some_match.html)]

- **union:** Creates a vector of unique elements between all collections. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.union.html)]

- **union_by:** Creates a vector of unique elements between two collections. it accepts iteratee which is invoked for each element of each collection to generate the criterion by which uniqueness is computed. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.union_by.html)]

- **unique:** Remove duplicate elements in collection(array, vector), use PartialEq equality comparisons. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.unique.html)]

- **unique_by:** Calls a provided custom comparator with element of collection, returns a vector of unique element. [[doc](https://docs.rs/rufl/0.1.2/rufl/collection/fn.unique_by.html)]


<h3 id="eventbus">2. Eventbus implements a simple pub/sub event lib.<a href="#index">index</a></h3>

```rust
use rufl::eventbus;
```

- **Event:** An event is a struct that can hold any data type. It is then published to the event bus. Once published, the event is then passed to each subscriber when the event bus runs. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/struct.Event.html)]

- **EventBus:** The event bus is a central hub for all events. It is responsible for managing all subscribers and publishing events related to the event bus. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/struct.EventBus.html)]


<h3 id="file">3. File mod contains several utility functions for handling file operation.<a href="#index">index</a></h3>

```rust
use rufl::file;
```
- **clear:** Clear file content. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/fn.clear.html)]

- **copy_dirs:** Copys all directories in src path to dst path. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/fn.copy_dirs.html)]

- **create:** Creates a file in path and returns it. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/fn.create.html)]

- **file_names:** Returns all file names of specific directory path. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/fn.file_names.html)]

- **get_md5:** Gets the md5 value of file. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/fn.get_md5.html)]

- **is_symlink:** Checks if file is symbol link file. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/fn.is_symlink.html)]

- **read_to_buffer:** Reads file to buffer byte array. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/fn.read_to_buffer.html)]

- **read_to_lines:** Reads file and returns lines string vector. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/fn.read_to_lines.html)]

- **read_to_string:** Reads file to string. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/fn.read_to_string.html)]

- **write_to:** Write data to file, if file isn’t exist, create it. [[doc](https://docs.rs/rufl/0.1.2/rufl/eventbus/fn.write_to.html)]


<h3 id="math">4. Math mod contains several utility functions for handling mathematical calculations.<a href="#index">index</a></h3>

```rust
use rufl::math;
```
- **abs:** Returns the absolute value of number n. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.abs.html)]

- **average:** Calculats the average value of number vector. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.average.html)]

- **factorial:** Calculats the factorial value of number n. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.factorial.html)]

- **fib_nth:** Calculates the nth value of fibonacci number sequence. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.fib_nth.html)]

- **fib_seq:** Returns fibonacci number sequence. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.fib_seq.html)]

- **fib_sum:** Calculates the sum value of fibonacci number sequence. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.fib_sum.html)]

- **gcd:** Returns greatest common divisor (GCD) of integers. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.gcd.html)]

- **harmonic:** Calculates harmonic value number n. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.harmonic.html)]

- **is_prime:** Checks if number is prime or not. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.is_prime.html)]

- **lcm:** Return least common multiple (lcm) of integers. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.lcm.html)]

- **percent:** Calculates percentage. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.percent.html)]

- **round:** Round off n decimal places to number. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.round.html)]

- **round_down:** Round down and truncate off n decimal places to number. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.round_down.html)]

- **round_up:** Round up and truncate off n decimal places to number. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.round_up.html)]

- **sqrt:** Calculates square root of float number n. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.sqrt.html)]

- **sum:** Calculats the sum of number vector. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.sum.html)]

- **to_angle:** Converts radian value to angle value. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.to_angle.html)]

- **to_radian:** Converts angle value to radian value. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.to_radian.html)]

- **truncate:** Truncate number to n decimal places after decimal point. [[doc](https://docs.rs/rufl/0.1.2/rufl/math/fn.truncate.html)]



<h3 id="random">5. Random mod contains several utility functions for generating random number or string.<a href="#index">index</a></h3>

```rust
use rufl::random;
```

- **alpha_number:** Generate random alphabetic or numberic string. [[doc](https://docs.rs/rufl/0.1.2/rufl/random/fn.alpha_number.html)]

- **alphabet:** Generate random alphabetic string. [[doc](https://docs.rs/rufl/0.1.2/rufl/random/fn.alphabet.html)]

- **lower:** Generate random lower case English letter string. [[doc](https://docs.rs/rufl/0.1.2/rufl/random/fn.lower.html)]

- **numberic:** Generate random numberic string. [[doc](https://docs.rs/rufl/0.1.2/rufl/random/fn.numberic.html)]

- **symbol:** Generate random string which only contains special chars (!@#$%^&*()_+-=[]{}|;’:",./<>?). [[doc](https://docs.rs/rufl/0.1.2/rufl/random/fn.symbol.html)]

- **upper:** Generate random upper case English letter string. [[doc](https://docs.rs/rufl/0.1.2/rufl/random/fn.upper.html)]



<h3 id="string">6. String mod contains several utility functions for handling string.<a href="#index">index</a></h3>

```rust
use rufl::string;
```

- **add_commas:** Add comma to a number value by every 3 numbers from right. Ahead by prefix symbol. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.add_commas.html)]

- **after:** Returns the substring after the first occurrence of a specified substr in the source string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.after.html)]

- **after_last:** Returns the substring after the last occurrence of a specified substr in the source string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.after_last.html)]

- **before:** Returns the substring before the first occurrence of a specified substr in the source string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.before.html)]

- **before_last:** Returns the substring before the last occurrence of a specified substr in the source string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.before_last.html)]

- **camel_case:** Converts string to camel case.[[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.camel_case.html)]

- **capitalize:** Converts the first character of string to upper case and the remaining to lower case.[[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.capitalize.html)]

- **count_by:** Counts the characters in target string with predicate function, returns the number of all matched characters. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.count_by.html)]

- **count_chars:** Returns the characters count in target string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.count_chars.html)]

- **count_graphemes:** Returns the graphemes count in target string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.count_graphemes.html)]

- **count_words:** Returns the word count in target string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.count_words.html)]

- **cut:** Cut searches for the substring ‘sep’ in the source string, and splits the source string into two parts at the first occurrence of the substring ‘sep’: before and after. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.cut.html)]

- **hide:** Hides some chars in source string and replace with speicfic substring. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.hide.html)]

- **index:** Searches a string and returns the index of the first occurrence of the specified searched substring. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.index.html)]

- **index_all:** Searches a string and returns all the indexs of the occurrence of the specified searched substring. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.index_all.html)]

- **is_alpha:** Checks if the string contains only alphabetic characters. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_alpha.html)]

- **is_alphanumberic:** Checks if the string contains only alphabetic or numeric characters. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_alphanumberic.html)]

- **is_digit:** Checks if the string contains only digit characters.(0-9) [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_digit.html)]

- **is_dns:** Checks if the string is a valid domain name. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_dns.html)]

- **is_email:** Checks if the string is a valid email address. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_email.html)]

- **is_ipv4:** Checks if the string is a valid ipv4 address. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_ipv4.html)]

- **is_ipv6:** Checks if the string is a valid ipv6 address. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_ipv6.html)]

- **is_lowercase:** Checks if the string contains only lowercase unicode characters. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_lowercase.html)]

- **is_numberic:** Checks if the string numeric (can be parsed to number). [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_numberic.html)]

- **is_strong_password:** Checks if the string is a strong password. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_strong_password.html)]

- **is_uppercase:** Checks if the string contains only uppercase unicode characters. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_uppercase.html)]

- **is_url:** Checks if the string is a valid url. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.is_url.html)]

- **kebab_case:** Converts string to kebab case. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.kebab_case.html)]

- **last_index:** Searches a string and returns the index of the last occurrence of the specified searched substring. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.last_index.html)]

- **lower_first:** Converts the first character of string to lower case. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.lower_first.html)]

- **pad:** Pads string on the left and right sides if it’s shorter than length. Padding characters are truncated if they can’t be evenly divided by length. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.pad.html)]

- **pad_end:** 	Pads string on the right side if it’s shorter than length. Padding str are truncated if they exceed length. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.pad_end.html)]

- **pad_start:** Pads string on the left side if it’s shorter than length. Padding str are truncated if they exceed length. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.pad_start.html)]

- **pascal_case:** Converts string to pascal case. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.pascal_case.html)]

- **remove_all:** Removes all the specified substring which occurrence in the source string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.remove_all.html)]

- **remove_first:** Removes the specified substring which first occurrence in the source string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.remove_first.html)]

- **remove_last:** Removes the specified substring which last occurrence in the source string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.remove_last.html)]

- **removen:** Removes first n matches substr in the source string. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.removen.html)]

- **snake_case:** Converts string to snake case. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.snake_case.html)]

- **split_chars:** 	Split the input string into a vector of characters. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.split_chars.html)]

- **split_graphemes:** Split the target string into a vector of graphemes str. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.split_graphemes.html)]

- **split_words:** Split the input string into a vector of its words (based on the position of capital letters or numbers). [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.split_words.html)]

- **starts_with_offset:** Checks whether a string starts with the specified prefix at offset position. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.starts_with_offset.html)]

- **substring:** Returns the part of target string from the start index up to and excluding the end index. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.substring.html)]

- **title_case:** Converts string to kebab case. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.title_case.html)]

- **upper_first:** Converts the first character of string to upper case. [[doc](https://docs.rs/rufl/0.1.2/rufl/string/fn.upper_first.html)]



## How to Contribute

#### [Contributing Guide](./CONTRIBUTING.md)