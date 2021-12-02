using System;
using System.Linq;
using Helper;

//Part 1
var input = Utils.ReadLinesAsInt("input.txt");
var part1 = input.Zip(input.Skip(1), (i1, i2) => i2 - i1).Count(x => x > 0);
Console.WriteLine(part1); //1233

//Part 2: linq doesnt support zip with multiple, so I created my own LINQ extension (see utils.cs)
var sums = input.ZipN(i => i.Sum(), input.Skip(1), input.Skip(2)).ToArray();
var part2 = sums.Zip(sums.Skip(1), (i1, i2) => i2 - i1).Count(x => x > 0);
Console.WriteLine(part2); //1275