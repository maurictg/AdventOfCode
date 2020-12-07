using System;
using System.IO;
using System.Linq;

namespace _6
{
    class Program
    {
        static void Main(string[] args)
        {
            string input = File.ReadAllText(Path.Combine(Environment.CurrentDirectory, "input.txt"));
            string[] groups = input.Split(Environment.NewLine + Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);
           
            //Part 1
            Console.WriteLine(groups.Sum(x => Questions(x).Length));
            
            //Part 2
            Console.WriteLine(groups.Sum(CountAnswers));
        }

        static char[] Questions(string q)
            => q.Replace("\n", "").ToCharArray().Distinct().ToArray();

        static int CountAnswers(string q)
        {
            var qs = Questions(q);
            string[] persons = q.Split('\n');
            return qs.Count(x => persons.All(f => f.Contains(x)));
        }
        
        
    }
}