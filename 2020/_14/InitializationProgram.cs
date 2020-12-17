using System;
using System.Collections.Generic;
using System.Linq;
using System.Text.RegularExpressions;

namespace _14
{
    public class InitializationProgram
    {
        private Queue<string> _instructions;
        private Dictionary<long, Int36> _memory = new Dictionary<long, Int36>();
        private string _mask;
        private Regex _regex = new Regex(@"mem\[(\d*)\]\s=\s(\d*)");

        public InitializationProgram(string instructions)
            => _instructions = new Queue<string>(instructions.Split(Environment.NewLine));

        public void Run(bool v2 = false)
        {
            while (_instructions.Any())
            {
                var i = _instructions.Dequeue();
                if (i.StartsWith("mask = "))
                {
                    _mask = i[7..];
                }
                else
                {
                    var m = _regex.Match(i);
                    long addr = Convert.ToInt64(m.Groups[1].Value);
                    if (!v2)
                    {
                        Int36 val = new Int36(Convert.ToInt64(m.Groups[2].Value), _mask);
                        _memory[addr] = val;
                    }
                    else
                    {
                        Int36 val = new Int36(Convert.ToInt64(m.Groups[2].Value));
                        var addresses = new Int36(addr).ApplyMaskV2(_mask);
                        foreach (var a in addresses)
                            _memory[a.Value] = val;
                    }
                }
            }
        }

        public long Sum()
            => _memory.Values.Where(x => x.Value > 0).Sum(x => x.Value);
    }
}