using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Text;

var ids = File.ReadAllLines("input.txt");
var alphabet = "abcdefghijklmnopqrstuvwxyz".ToCharArray();

//Part 1
var twos = 0;
var threes = 0;

foreach (var id in ids)
{
    var containsTwo = alphabet.Any(x => id.Count(y => y == x) == 2);
    var containsThree = alphabet.Any(x => id.Count(y => y == x) == 3);
    twos += containsTwo?1:0;
    threes += containsThree?1:0;
}

Console.WriteLine(twos*threes);

//Part 2
var items = new Queue<string>(ids);

while (items.Count > 0)
{
    var str = items.Dequeue();
    string? diff = null;
    
    var match = items.FirstOrDefault(x => Compare(str, x, out diff) == str.Length - 1);
    if (match != null)
    {
        Console.WriteLine(diff);
    }
}

static int Compare(string one, string other, out string? same)
{
    var score = 0;
    var chars = new StringBuilder();
    for (var i = 0; i < Math.Min(one.Length, other.Length); i++)
    {
        if (one[i] == other[i])
        {
            chars.Append(one[i]);
            score++;
        }
    }

    same = chars.ToString();
    return score;
}