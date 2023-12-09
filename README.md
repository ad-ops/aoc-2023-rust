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

## Day 4
Was a pretty nice and standard task. This time building structs to handle the input.

## Day 5
Started building HashMaps, but that quickly became to hard so with my own structs it was more manageble. Still it took to long for part 2 so I added `Rayon` to use all cores. It still took almost 2 min on my MacBook M1 and 40s on my UM790 Pro.

Tried to save all processed ranges and ignore seeds that was in that range. It was a bit faster, but not significant...

## Day 6
This feels like it was the simplest task this AoC. I was worried that it would be to big to brute-force, but it took only 27ms on my UM790 Pro. The only thing I needed to change was to use `u64`.

## Day 7
For me this was the hardest task because I first implemented real poker-rules for determining the value. Everything became much simpler when I read the instructions correctly, but it meant many things were designed for a more advanced case which took some time to undo. It was nice to be able to lean on Rust's type system and deriving `Ord` to get the value of a hand.

