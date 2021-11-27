using System;
using System.IO;
using System.Linq;
using _4;

var records = Record.OrderAndSet(File.ReadAllLines("input.txt").Select(x => new Record(x)));
var guards = records.GroupBy(x => x.GuardId).Select(x => new Guard(x)).ToArray();

var sleepyJoe = guards.OrderByDescending(x => x.SleepyMinutes.Sum()).First();
Console.WriteLine($"Sleeps the most: #{sleepyJoe.Id}");

//Part 1
var sleepyMinute = sleepyJoe.SleepyMinutes.Select((n, min) => (min, n)).OrderByDescending(x => x.n).First();
Console.WriteLine($"Sleeps the most at {sleepyMinute}");

Console.WriteLine($"Answer: {sleepyJoe.Id * sleepyMinute.min}");

//Part 2
var freqSleeper = guards.OrderByDescending(x => x.SleepyMinutes.Max()).First();
var sleepyMoment = freqSleeper.SleepyMinutes.Select((n, min) => (min, n)).OrderByDescending(x => x.n).First();

Console.WriteLine($"Sleeps most frequent: #{freqSleeper.Id}");
Console.WriteLine($"Sleeps most frequent at: {sleepyMoment}");

Console.WriteLine($"Answer: {freqSleeper.Id * sleepyMoment.min}");