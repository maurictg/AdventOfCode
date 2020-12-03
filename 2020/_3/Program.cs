using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace _3
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] data = File.ReadAllLines(Path.Combine(Environment.CurrentDirectory, "input.txt"));

            PartOne(new RepeatingMapNavigator(data));
            PartTwo(new RepeatingMapNavigator(data));
        }

        static void PartOne(RepeatingMapNavigator map)
        {
            int cnt = CountSlope(map, (3, 1));
            Console.WriteLine(cnt);
        }

        static int CountSlope(RepeatingMapNavigator map, (int, int) slope)
        {
            //map.NavigateTo((0, 0));
            int cnt = 0;
            do
            {
                //Console.WriteLine(map.At());
                if (map.At() == '#')
                    cnt++;
            } while (map.Navigate(slope.Item1, slope.Item2));

            //Console.WriteLine(cnt);
            return cnt;
        }

        static void PartTwo(RepeatingMapNavigator map)
        {
            var slopes = new List<(int, int)>()
            {
                (1,1), (3,1), (5,1), (7,1), (1,2)
            };
            
            var results = new List<long>();

            foreach (var slope in slopes)
            {
                results.Add(CountSlope(map, slope));
                map.Reset();
            }
            
            Console.WriteLine(results.Aggregate((a, x) => a * x));
        }
    }
}