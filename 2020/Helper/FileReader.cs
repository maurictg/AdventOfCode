using System;
using System.Linq;
using System.IO;
using System.Threading.Tasks;

namespace Helper
{
    public static class FileReader
    {
        public static int[] ToIntArray(this string[] arr) 
            => arr.Select(x => Int32.Parse(x)).ToArray();
        
        public static long[] ToLongArray(this string[] arr) 
            => arr.Select(x => Int64.Parse(x)).ToArray();

        public async static Task<int[]> ReadLines(string file)
            => (await File.ReadAllLinesAsync(file)).ToIntArray();

        public async static Task<int[]> ReadSeperated(string file, char separator = ',')
            => (await File.ReadAllTextAsync(file)).Split(separator).ToIntArray();
    }
}