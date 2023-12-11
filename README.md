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

## Day 07

I think the main point of this day's puzzle is sorting. Since I'm using Rust's built-in sort function, the language takes care of the sorting algorithm for me. The only thing I need to do is specify how to order the items. The first part is easy enough, and the simplicity of the requirement caused me to write the rules with multiple if-else blocks ala YandereDev. When I started on the second part and I had to change the existing rules, I realized just how messy this looks. It does work, though, but if a need arises where we need to add another special rule into the mix in the future, it'd be best to scrap it and think up something smarter.

## Day 08

I made a binary tree on this day. This Advent of Code thing starts to feel like LeetCode now because of it!

I've built a binary tree before but not in Rust. It's trickier here since each node references another node and you know referencing memory gets weird in Rust because of their borrow checker. I know I have to store them in the heap somewhere so I'll either have to use Box or Rc. I chose with Rc because of gut feeling, but I probably could've accomplished it with Box too. Because the nodes can loop into each other, I built all the node labels into structs first and then assigned the branches afterwards. But then I couldn't set the left and right branches because Rc is immutable... (maybe I should've gone with Box...?) so after Googling it I'm supposed to use RefCell for "interior mutability" or whatever it is. Now it works! And part 1 was completed in no time (lies; took me an hour or two to figure this out).

Then part 2 begins and boy, I fell hard on a pitfall trap on this one.

Like any straight-minded monkey, I made a loop that starts all nodes start together and then keep looping until they all land on their finished nodes together. They NEVER do. I ran the program for three hours. Then I rebuilt the executable using `--release` profile and it got a MASSIVE boost in speed. But it still never ends!

After hours and hours of waiting for the numbers to pop out, which never did, I realized that I'm going at this in a fundamentally wrong way. Fortunately, my math knowledge was able to think it up: the answer was LCM (least common denominator; or more familiarly known as "KPK / Kelipatan Persekutuan terKecil" in Indonesian). So I reran the part 1 solution, punching in the starting nodes manually, and then recording the results. Then I used an online LCM calculator. I got the final answer there. I would've been stuck if it was something I have no idea about such as calculus; glad it was just grade school math.

So, yeah. I got the second gold star without finishing the part 2 solution. I did come back and code in how to calculate LCM in the part 2 solution, though. Turns out it involves recursion and modulo (thanks GeeksForGeeks). I made use of my experience of using modulo operator in Rust: don't use `%`, use `rem_euclid()`. Though I don't remember exactly why.

Thanks to my first attempt at part 2 solution I also found out that the binary compiled with the debug profile and release profile has a very significant speed difference. Without using the release profile, my program took 10~20 seconds to get to where two nodes to finish together (on the 670079th step) but using the release profile, it only took ~2 seconds.

## Day 09

Another breather day, I think.

Thankfully, how to solve the difference is spelled out in the problem. As simple as it is, I might have not thought of doing this "difference matrix" to calculate the next or the pervious value had they not mentioned it. I know that this is a basic, school-taught way of doing arithmetic/geometric sequence, but my dumbass would've struggled to remember how to do it anyway. There's always Google, though.
