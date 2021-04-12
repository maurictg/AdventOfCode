using System;
using System.IO;
using System.Linq;

namespace _17
{
    class Program
    {
        //Using the same strategy as day #11, when I was using an update system to apply changes
        //at the same time.
        static void Main(string[] args)
        {
            string[] input = File.ReadAllLines("input.txt");
            var cc = new ConwayCubes(input);
            Console.WriteLine(cc.PrintLayer(0));
            cc.ExecuteAll();
            cc.Update();
            Console.WriteLine(cc.PrintLayer(0));
        }
    }
}