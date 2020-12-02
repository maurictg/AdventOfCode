using System;
using System.IO;
using System.Linq;

namespace _2
{
    class Program
    {
        static void Main(string[] args)
        {
            //part 1
            string[] input = File.ReadAllLines(Path.Combine(Environment.CurrentDirectory, "input.txt"));
            int validPasswords = input.Count(x => new PasswordPolicy(x).IsValid());
            Console.WriteLine($"{validPasswords} are valid according to the policy");
            
            //part 2
            int tobaggonValidPassword = input.Count(x => new TobogganPasswordPolicy(x).IsValid());
            Console.WriteLine($"{tobaggonValidPassword} are valid according to the tobaggan policy");
        }
    }
}