# Advent of Code 2022

## By Olle Wreede

---

## Advent of Code

 * Daily programming puzzles.
 * https://adventofcode.com/2022

---

## Solutions

 * Solutions in the Rust programming language.
 * Execute the solutions by hitting `Enter`.
 * This only works when running locally.

---

## Day 1 part 1

```rust
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut gnomes = vec![];
    let mut calories = 0;
    let input = fs::read_to_string("assets/aoc2022/input1.txt")?;
    for line in input.lines() {
        if line.is_empty() {
            gnomes.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<i32>()?;
        }
    }
    gnomes.push(calories);
    gnomes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("Gnome with most calories: {}", gnomes.first().unwrap());
    Ok(())
}
```

---

## Day 1 part 2

```rust
use std::{error::Error, fs};

fn main() -> Result<(), Box<dyn Error>> {
    let mut gnomes = vec![];
    let mut calories = 0;
    let input = fs::read_to_string("assets/aoc2022/input1.txt")?;
    for line in input.lines() {
        if line.is_empty() {
            gnomes.push(calories);
            calories = 0;
        } else {
            calories += line.parse::<i32>()?;
        }
    }
    gnomes.push(calories);
    gnomes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    let top_three: i32 = gnomes.iter().take(3).sum();
    println!("Calories from top three gnomes: {}", top_three);
    Ok(())
}
```
