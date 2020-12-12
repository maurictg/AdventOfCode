using System;
using System.IO;

namespace _12
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] input = File.ReadAllLines("input.txt");
            
            //Part 1
            Ship s = new Ship(input);
            s.SetSail();
            Console.WriteLine(s.Distance());

            //Part 2
            Ship s2 = new Ship(input);
            s2.UseWaypoint();
            s2.SetSail();
            Console.WriteLine(s2.Distance());
        }
    }
}