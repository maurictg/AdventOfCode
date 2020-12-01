using System;
using System.Linq;
using System.IO;
using System.Threading.Tasks;

namespace Helper
{
    public static class FileReader
    {
        public async static Task<int[]> ReadLines(string file)
        {
            string[] lines = await File.ReadAllLinesAsync(file);
            return lines.Select(x => Int32.Parse(x)).ToArray();
        }
    }
}