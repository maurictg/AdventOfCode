using System;
using System.Collections.Generic;
using System.IO;
using System.Linq;

namespace Helper
{
    public static class Utils
    {
        public static int[] ReadLinesAsInt(string file)
            => File.ReadAllLines(file).Select(x => Convert.ToInt32(x)).ToArray();

        //LINQ extension to zip multiple collections
        public static IEnumerable<TRes> ZipN<T,TRes>(this IEnumerable<T> coll, Func<IEnumerable<T>,TRes> selector, params IEnumerable<T>[] collections)
        {
            var enumerators = collections.Select(x => x.GetEnumerator()).ToList();
            enumerators.Add(coll.GetEnumerator());
            while (enumerators.All(x => x.MoveNext()))
                yield return selector(enumerators.Select(x => x.Current));
        }
    }
}

