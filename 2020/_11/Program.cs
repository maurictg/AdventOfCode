using System;
using System.IO;

namespace _11
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] input = File.ReadAllLines("input.txt");
            SeatMap m = new SeatMap(input);
            
            //Part 1
            do m.ApplyRules();
            while (m.Update() > 0);
            Console.WriteLine(m.Count('#'));
            
            //Part 2
            SeatMap m2 = new SeatMap(input);
            do m2.ApplyRules(true);
            while (m2.Update() > 0);
            Console.WriteLine(m2.Count('#'));
        }
    }
}