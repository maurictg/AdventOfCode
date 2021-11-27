using System;
using System.IO;
using System.Linq;
using _3;

var fabric = new byte[1000,1000];
var claims = File.ReadAllLines("input.txt").Select(x => new Claim(x)).ToArray();

//Part 1
foreach (var claim in claims)
{
    foreach (var c in claim.Coordinates())
        fabric[c.x, c.y]++;
}

var cnt = 0;

for (var x = 0; x < 1000; x++)
    for (var y = 0; y < 1000; y++)
        cnt += fabric[x,y] >=2 ? 1 : 0;
        
Console.WriteLine(cnt);

//Part 2
var untouched = claims.Single(claim => claim.Coordinates().All(c => fabric[c.x, c.y] == 1));
Console.WriteLine($"#{untouched.Id}");