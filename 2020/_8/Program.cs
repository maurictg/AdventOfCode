using System;
using System.Collections.Generic;
using System.IO;

namespace _8
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] input = File.ReadAllLines(Path.Combine(Environment.CurrentDirectory, "input.txt"));
            
            //Part 1
            Interpreter i = new Interpreter(input);
            Console.WriteLine(i.Execute());
            
            //Part 2
            i = new Interpreter(input);
            Console.WriteLine(i.Execute(true));
        }
    }
}