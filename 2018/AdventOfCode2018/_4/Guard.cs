using System.Collections.Generic;
using System.Linq;

namespace _4
{
    public struct Guard
    {
        public int Id { get; set; }
        public readonly int[] SleepyMinutes;

        public Guard(IEnumerable<Record> records)
        {
            SleepyMinutes = new int[60];
            var rec = records.OrderBy(x => x.Timestamp).ToArray();
            Id = rec.First().GuardId ?? -1;

            int? from = null;
            foreach (var r in rec.Where(x => x.Timestamp.Hour == 0))
            {
                if (r.Asleep)
                    from = r.Timestamp.Minute;
                else
                {
                    if (from != null)
                    {
                        var f = from.Value;
                        var t = r.Timestamp.Minute;
                        for (var i = f; i < t + 1; i++) SleepyMinutes[i]++;
                    }
                }
            }
        }
    }
}