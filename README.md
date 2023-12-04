# AoC 2023 - Rust
**spoilers for AoC 2023!**

Nothing fancy this year in form of util and structure.

## Day 1
Part 2 was harder than expected. Tried using regex to solve it, but the problem was overlapping words such as: `xtwoney` which could be `two` and also `one`. The problem was when it was the last number so it would match two and consume the characters.

Solved it first with a super ugly "reverse" regex and only getting the first and the reversed (last) first number.

## Day 2
Feels like I should have parsed the data into real structs, but it was a simple enough problem.

## Day 3
I started making a plan to do a lookup in a hashmap for the coordinates, but then I forgot about that and just iterated over everything. Would be interesting to see the performance differences...

