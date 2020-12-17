using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;

namespace _14
{
    class Program
    {
        static void Main(string[] args)
        {
            string input = File.ReadAllText("input.txt");
            //Part 1
            var p = new InitializationProgram(input);
            p.Run();
            Console.WriteLine(p.Sum());
            
            //Part 2
            var p2 = new InitializationProgram(input);
            p2.Run(true);
            Console.WriteLine(p2.Sum());
        }
    }
}