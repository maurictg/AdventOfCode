using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Utils
{
    public static class Utils
    {
        public static IEnumerable<int> ReadToInt(string file)
        {
            return File.ReadAllLines(file)
                .Select(x => x.TrimStart('+'))
                .Select(x => Convert.ToInt32(x));
        }

        public static char Toggle(this char inp)
            => char.IsUpper(inp) ? char.ToLower(inp) : char.ToUpper(inp);
    }
}