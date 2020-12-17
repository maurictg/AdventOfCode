using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace _13
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] input = File.ReadAllLines("input.txt");
            
            //Part 1
            BusStop b = new BusStop(input);
            var bus = b.Next();
            var waitingTime = bus.time - b.EarliestTimestamp;
            Console.WriteLine(bus.id * waitingTime);

            //Part 2. very, very heavy brute force. Maybe the answer will be found in the upcoming year
            /*BusStop b2 = new BusStop(input, false);
            var match = b2.MaskMatches(100000000000000).First();
            Console.WriteLine(match);*/
        }
    }
}