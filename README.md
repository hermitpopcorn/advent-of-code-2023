# Advent of Code 2023

This repo houses my code solutions for [Advent of Code](https://adventofcode.com/) 2023.

I'll write my findings and impressions for each day on this README file.

## Day 01

My current solution uses a straight for loop (the one for part two even does two passes for each input). I tried implementing sliding window for both parts but for some reason it increases the execution time instead of reducing it. I used `std::time::Instant` to track the execution time, though--maybe the more proper `cargo bench` command will turn up a different result? Or maybe that's not it. Maybe the sliding window I implemented was just wack af and I'm actually just dumber than I thought I could possibly be.

## Day 02

I ditched the idea of making it fast and just wrote it to be readable. The functions are many, but short. The function names are bad, though; I made "game sets" up.

## Day 03

I think this one is far trickier than the last.

This time, I ditched the thought of optimization out of the window and just wrote in a way that I deem as "good structure." As Fowler said, "write tunable software first and then tune it for sufficient speed."

I don't think I'll be tuning this anytime soon, though.
