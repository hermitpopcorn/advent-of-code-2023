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

## Day 04

Deceptive! The first part was very similar to Day 02, leading me to base the solution on the code for that day. But then the second part rolls around with greater complexity.

I thought I had to go the recursive path that's going to exponentially increase my computation times since it has to redo the calculations over and over, but after thinking it out a bit, I can just work bottom-to-top, caching the results along the way to minimize recalculation. Is this the so-called "dynamic programming"?

## Day 05

The numbers got so big that `u32` overflowed. I went with usize instead for this day.

I started part 2 with the naive approach of "Okay, let's just loop and make a list of all the seeds in the seed ranges and then loop over all of them." The resulting seed list was so big and took so much memory that the executable just killed itself... Oh well. Start small and then tune it for sufficient speed!

Then I continued with the native approach of "Okay, let's not create big lists in memory, let's just do loops with short-lived integers in the stack and loop over all of them." The resulting run time took so much time that I almost killed myself... Oh well. I slapped Rayon on, pretended parallelism will help me, and hoped for the best. At least I got my answer? After hours of waiting...

I don't know how to optimize this. Each step of the way has a different set of range so I don't know how to reliably make a dictionary and cache part of the process without it going wrong midway.

It's only the fifth day and I'm stumped already. This does not bode well.

## Day 06

A breather day; this day's far easier than the previous one.

Still not unlike the previous days, the second part of this day brings about a very big number compared to the first part. Fortunately, there are no branching paths, so I decided on a simple solution of linear calculation. The answer didn't take much time to show. Maybe I can optimize this by going from both ends at the same time, or maybe some other way. I'll think about it at a later date.
