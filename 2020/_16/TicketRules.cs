using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace _16
{
    public class TicketRules
    {
        public Dictionary<string, TicketField> Fields;
        private Regex _field = new Regex(@"(\w*\s?\w*?):\s(\d*)-(\d*)\sor\s(\d*)-(\d*)");

        public TicketRules(string input)
        {
            Fields = new Dictionary<string, TicketField>();
            foreach (var item in input.Split(Environment.NewLine))
            {
                var m = _field.Match(item);
                Fields.Add(m.Groups[1].Value, new TicketField(
                    Convert.ToInt32(m.Groups[2].Value),
                    Convert.ToInt32(m.Groups[3].Value),
                    Convert.ToInt32(m.Groups[4].Value),
                    Convert.ToInt32(m.Groups[5].Value)));
            }
        }

        public bool AnyValid(int value)
            => Fields.Select(x => x.Value)
                .Any(x => x.IsMatch(value));

        //TODO: get valid fields for part two
    }
}