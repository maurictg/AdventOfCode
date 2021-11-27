// See https://aka.ms/new-console-template for more information

using System;
using System.IO;
using System.Linq;
using _5;

var str = File.ReadAllText("input.txt");

//Part 1
Console.WriteLine(new Polymer(str).React().Length);

//Part 2
var alphabet = "abcdefghijklmnopqrstuvwxyx";
var shortest = alphabet.Select(x => new Polymer(str).Without(x)).Min(x => x.React().Length);
Console.WriteLine(shortest);