using System;
using System.Collections.Generic;

namespace _7
{
    public class Bag
    {
        public string Color { get; set; }
        public Dictionary<string, int> Bags { get; set; }

        public bool Contains(string color)
            => Bags.ContainsKey(color);

        public Bag(string b)
        {
            Bags = new Dictionary<string, int>();
            int i = b.IndexOf("bags contain", StringComparison.Ordinal);
            Color = b[..i].TrimEnd();
            if(b.Contains("no other bags"))
                return;
            
            foreach (var x in b[(i + 12)..].TrimEnd('.').Split(','))
            {
               var item = x[..x.IndexOf("bag", StringComparison.Ordinal)].Trim();
               int id = item.IndexOf(' ');
               Bags.Add(item[(id+1)..], Convert.ToInt32(item[..id]));
            }
        }

        public override string ToString()
            => Color;
    }
}