using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;
using System.Linq;

var input = File.ReadAllLines("input.txt")
    .Select(x => new BitArray(x.Select(b => b == '1').ToArray())).ToArray();

string ToString(BitArray arr)
{
    var bl = new bool[arr.Length];
    arr.CopyTo(bl, 0);
    return string.Join("", bl.Select(x => x ? '1' : '0'));
}

bool MostCommonBit(IEnumerable<BitArray> arr, int pos, bool bitCriteria = true)
{
    var a = arr.ToArray();
    var cnt = Enumerable.Range(0, a.Length).ToArray();
    var (i, o) = (cnt.Count(y => a[y][pos]), cnt.Count(y => !a[y][pos]));
    return i == o ? bitCriteria : bitCriteria ? i > o : i < o;
}

int ToDecimal(BitArray b) => Convert.ToUInt16(ToString(b), 2);

//Part 1
var gamma = new BitArray(input[0].Length);
for (var x = 0; x < input[0].Length; x++)
    gamma.Set(x, MostCommonBit(input, x));

var epsilon = ((BitArray)gamma.Clone()).Not();

var γ = ToDecimal(gamma);
var ε = ToDecimal(epsilon);

Console.WriteLine(γ * ε);

//Part 2
BitArray Find(bool bitCriteria)
{
    var items = input.ToList();
    for (var x = 0; x < items[0].Length; x++)
    {
        var mcb = MostCommonBit(items, x, bitCriteria);
        var filtered = items.Where(i => i[x] == mcb).ToList();

        if (filtered.Count == 1)
            return filtered.First();
        
        items = filtered;
    }

    throw new ArgumentException("Error: array not found. Check your input!");
}

var oxygen = ToDecimal(Find(true));
var scrubber = ToDecimal(Find(false));

Console.WriteLine(oxygen * scrubber);