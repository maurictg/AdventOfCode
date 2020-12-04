using System;
using System.IO;
using System.Linq;

namespace _4
{
    class Program
    {
        static void Main(string[] args)
        {
            Passport[] inputs = File.ReadAllText(Path.Combine(Environment.CurrentDirectory, "input.txt"))
                .Split(Environment.NewLine + Environment.NewLine, StringSplitOptions.RemoveEmptyEntries)
                .Select(x => new Passport(x)).ToArray();
            
            //Part 1
            Console.WriteLine(inputs.Count(x => x.Valid));
            
            //Part 2
            foreach (var p in inputs)
                p.Parse(); //parse because not done in constructor for first
            
            Console.WriteLine(inputs.Count(x => x.Valid));
        }
    }
}