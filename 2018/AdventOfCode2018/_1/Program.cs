using System;
using System.Collections.Generic;
using System.Linq;

var numbers = Utils.Utils.ReadToInt("input.txt").ToArray();

//Part one
var result = numbers.Sum();
Console.WriteLine(result);

//Part two
var cache = new List<int> {0};
var freq = 0;

while (true)
{
    foreach (var n in numbers)
    {
        freq += n;
        
        if (cache.Contains(freq))
        {
            Console.WriteLine("Found: "+freq);
            return;
        }
    
        cache.Add(freq);
    }
}