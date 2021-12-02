using System;
using System.IO;
using _2;

var instructions = File.ReadAllLines("input.txt");

//Part 1
var sub1 = new Submarine();
sub1.Go(instructions);

Console.WriteLine(sub1.X * sub1.Depth);

//Part 2
var sub2 = new Submarine(aimMode: true);
sub2.Go(instructions);

Console.WriteLine(sub2.X * sub2.Depth);