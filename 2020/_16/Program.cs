using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace _16
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] inputs = File.ReadAllText("input.txt")
                .Split(Environment.NewLine + Environment.NewLine, StringSplitOptions.RemoveEmptyEntries);

            string options = inputs[0];

            int[] myTicket = inputs[1].Split(Environment.NewLine)[1].Split(',')
                .Select(x => Convert.ToInt32(x)).ToArray();

            List<int[]> otherTickets = inputs[2].Split(Environment.NewLine).Skip(1)
                .Select(x => x.Split(',').Select(y => Convert.ToInt32(y)).ToArray()).ToList();
            
            var rules = new TicketRules(options);

            var invalidValues = new List<int>();
            foreach (var t in otherTickets)
            {
                var s = t.Where(f => !rules.AnyValid(f));
                invalidValues.AddRange(s);
            }

            //Part 1
            Console.WriteLine(invalidValues.Sum());
            
            //Part 2
            var validTickets = otherTickets.Where(x => x.All(f => rules.AnyValid(f))).ToList();
            validTickets.Add(myTicket);

            //TODO: part 2
        }
    }
}