# Advent of Code 2023

This repo houses my code solutions for [Advent of Code](https://adventofcode.com/) 2023.

I'll write my findings and impressions for each day on this README file.

## Day 01

My current solution uses a straight for loop (the one for part two even does two passes for each input). I tried implementing sliding window for both parts but for some reason it increases the execution time instead of reducing it. I used `std::time::Instant` to track the execution time, though--maybe the more proper `cargo bench` command will turn up a different result? Or maybe that's not it. Maybe the sliding window I implemented was just wack af and I'm actually just dumber than I thought I could possibly be.
