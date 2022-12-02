# Advent of Code 2022

## By Olle Wreede

---

### Day 1

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
    gnomes.sort_by(|a, b| b.partial_cmp(a).unwrap());
    println!("Gnome with most calories: {}", gnomes.first().unwrap());
    Ok(())
}
```

---


