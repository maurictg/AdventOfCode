using System;
using System.IO;
using System.Linq;

namespace _5
{
    class Program
    {
        static void Main(string[] args)
        {
            string[] passes = File.ReadAllLines(Path.Combine(Environment.CurrentDirectory, "input.txt"));
            
            //Part 1
            Console.WriteLine(passes.Max(x => GetId(x)));
            
            //Part 2
            int[] ids = passes.Select(x => GetId(x))
                .OrderBy(x => x).ToArray();
            
            //Zip returns pairs like: ([660 661], [661, 662], [662, 663]
            //So the first pair that differs 2 (like: [670, 672]) miss one step: the missing seat
            //So return the lowest one + 1 or the greatest one - 1
            Console.WriteLine(ids
                .Zip(ids.Skip(1), (left, right) => (left,right))
                .First(p => p.Item2 - p.Item1 > 1).Item1 + 1);
        }

        static int GetIdSmart(string pass)
        {
            int row = Convert.ToInt32(pass[..7].Replace('F', '0').Replace('B', '1'), 2);
            int col = Convert.ToInt32(pass[7..].Replace('L', '0').Replace('R', '1'), 2);
            return (row * 8) + col;
        }

        static int GetId(string pass)
        {
            int row = GetPos(pass.Substring(0, 7));
            int col = GetPos(pass.Substring(7, 3), 0, 7);
            return (row * 8) + col;
        }

        static int GetPos(string instruction, int lo = 0, int hi = 127)
        {
            for (int j = 0; j < instruction.Length; j++)
            {
                int diff = Convert.ToInt32(Math.Floor((float)hi - lo) / 2);
                if (instruction[j] == 'F' || instruction[j] == 'L') hi -= diff;
                else lo += diff;
                //Console.WriteLine($"lo: {lo}, hi: {hi}");
            }

            return (instruction[^1] == 'F' || instruction[^1] == 'L') ? lo : hi;
        }
    }
}