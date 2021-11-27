using System;
using System.Collections.Generic;
using System.Globalization;
using System.Linq;
using System.Text.RegularExpressions;

namespace _4
{
    public struct Record
    {
        private static readonly Regex Regex = new(@"#(\d+)");
        public DateTime Timestamp { get; set; }
        public int? GuardId { get; set; }
        public bool Asleep { get; set; }

        public Record(string str)
        {
            var parts = str.TrimStart('[').Split(']');
            Timestamp = DateTime.ParseExact(parts[0], "yyyy-MM-dd HH:mm", CultureInfo.InvariantCulture);
            if (Regex.IsMatch(parts[1]))
            {
                Asleep = false;
                GuardId = Convert.ToInt32(Regex.Match(parts[1]).Groups[1].Value);
            }
            else
            {
                if (parts[1].Contains("sleep")) Asleep = true;
                else Asleep = false;
                GuardId = null;
            }
        }

        public static Record[] OrderAndSet(IEnumerable<Record> records)
        {
            var rec = records.OrderBy(x => x.Timestamp).ToArray();
            int? curr = null;
            for (var i = 0; i < rec.Length; i++)
            {
                if (rec[i].GuardId != null)
                    curr = rec[i].GuardId;
                else
                    rec[i].GuardId ??= curr;
            }

            return rec;
        }
    }
}