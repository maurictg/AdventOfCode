using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;
using System.Threading.Tasks;
using Helper;

namespace _9
{
    class Program
    {
        static void Main(string[] args)
        {
            long[] input = File.ReadAllLines(Path.Combine(Environment.CurrentDirectory, "input.txt")).ToLongArray();
            
            //Part 1
            var i = new XMAS(input, 25);
            long answer = i.GetInvalidNumbers().FirstOrDefault();
            Console.WriteLine(answer);
            
            //Part 2
            var x = i.FindWeakness(answer);
            Console.WriteLine(x.First() + x.Last());
        }
    }
}