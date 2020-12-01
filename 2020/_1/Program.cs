using System;
using System.IO;
using System.Threading.Tasks;
using Helper;

namespace _1
{
    class Program
    {
        static async Task Main(string[] args)
        {
            int[] input = await FileReader.ReadLines(Path.Combine(Environment.CurrentDirectory, "input.txt"));
            Console.WriteLine(From2(input));
            Console.WriteLine(From3(input));
        }

        static int From2(int[] input) {
            for (int i = 0; i < input.Length; i++)
            {
                for (int j = i; j < input.Length; j++)
                {
                    int sum = input[i] + input[j];
                    if(sum == 2020) {
                        return input[i] * input[j];
                    }
                }
            }
            return -1;
        }

        static int From3(int[] input) {
            for (int i = 0; i < input.Length; i++)
            {
                for (int j = i; j < input.Length; j++)
                {
                    for (int k = 0; k < input.Length; k++)
                    {
                        int sum = input[i] + input[j] + input[k];
                        if(sum == 2020) {
                            return input[i] * input[j] * input[k];
                        }
                    }
                }
            }
            return -1;
        }
    }
}