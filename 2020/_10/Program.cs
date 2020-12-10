using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Helper;

namespace _10
{
    class Program
    {
        static async Task Main(string[] args)
        {
            int[] input = (await FileReader.ReadLines("input.txt"))
                .OrderBy(x => x).ToArray();

            Console.WriteLine(PartOne(input));
            
            var inp = new List<int>(input.Length + 2);
            inp.Add(0); //add outlet
            inp.AddRange(input); //add adapters
            inp.Add(input.Max() + 3); //add target joltage
            
            Console.WriteLine(PartTwo(inp.ToArray()));
        }

        static long PartTwo(int[] inp)
        {
            long[] possibilities = new long[inp.Max() + 1];
            possibilities[0] = 1;
            
            for (int i = 1; i < inp.Max() + 1; i++)
            {
                //try possibilities 1, 2 and 3
                for (int j = 1; j < 4; j++)
                    if (inp.Contains(i - j))
                        possibilities[i] += possibilities[i - j]; //shift possibilities
            }

            return possibilities[^1];
        }

        static int PartOne(int[] inp)
        {
            int outlet = 0, j1diff = 0, j3diff = 0;
            int closest(int min) => inp.Where(x => x - min <= 3 && x > min).Min();
            
            for (int i = 0; i < inp.Length; i++)
            {
                int prev = outlet;
                outlet = closest(outlet);
                if (outlet - prev == 3) j3diff++;
                if (outlet - prev == 1) j1diff++;
            }

            return j1diff * (j3diff + 1);
        }
    }
}