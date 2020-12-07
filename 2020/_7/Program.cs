using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace _7
{
    class Program
    {
        static void Main(string[] args)
        {
            List<Bag> input = File.ReadAllLines(Path.Combine(Environment.CurrentDirectory, "input.txt"))
                .Select(x => new Bag(x)).ToList();

            //Part 1
            Console.WriteLine(FindCount(input, "shiny gold", new List<Bag>()));
            
            //Part 2
            var shinyGoldBag = input.First(x => x.Color == "shiny gold");
            Console.WriteLine(CountChildren(input, shinyGoldBag));
        }

        //The power of recursion!
        static int FindCount(List<Bag> bags, string searchTerm, List<Bag> alreadyFound)
        {
            var res = bags
                .Where(b => !alreadyFound.Contains(b))
                .Where(x => x.Contains(searchTerm)).ToList();
            
            if (res.Count == 0)
                return 0;

            alreadyFound.AddRange(res);
            return res.Count + res.Sum(x => FindCount(bags, x.Color, alreadyFound));
        }

        static int CountChildren(List<Bag> bags, Bag b)
        {
            if (b.Bags.Count == 0) return 0;

            return b.Bags.Sum(x => x.Value)
                   + b.Bags.Sum(x => CountChildren(bags, bags.First(y => y.Color == x.Key)) * x.Value);
        }
    }
}