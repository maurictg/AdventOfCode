var elves = File.ReadAllText("input.txt").Split(Environment.NewLine + Environment.NewLine);

// Part 1
var max = elves.Max(e => e.Split(Environment.NewLine).Select(s => Convert.ToInt64(s)).Sum(x => x));
Console.WriteLine(max);

// Part 2
var topElves = elves.Select(e => e.Split(Environment.NewLine).Select(s => Convert.ToInt64(s)).Sum(x => x))
    .OrderByDescending(x => x);
    
var top3 = topElves.Take(3).Sum();
Console.WriteLine(top3);