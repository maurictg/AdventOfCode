using System;
using System.Linq;

namespace _15
{
    class Program
    {
        static void Main(string[] args)
        {
            //Part 1
            MemoryGame mg = new MemoryGame("0,1,5,10,3,12,19");
            Console.WriteLine(mg.Next().Skip(2019).First());

            //Part 2
            mg = new MemoryGame("0,1,5,10,3,12,19");
            Console.WriteLine(mg.Next().Skip(30000000 - 1).First());
        }
    }
}