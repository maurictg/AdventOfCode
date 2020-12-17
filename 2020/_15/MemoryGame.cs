using System;
using System.Collections.Generic;
using System.Linq;

namespace _15
{
    public class MemoryGame
    {
        private int[] _startingNumbers;
        private Dictionary<int, int> _memory;
        
        public MemoryGame(string numbers)
        {
            _startingNumbers = numbers.Split(',')
                .Select(x => Convert.ToInt32(x)).ToArray();
            _memory = new Dictionary<int, int>();
        }
        
        public IEnumerable<int> Next()
        {
            int turn = 1;
            foreach (var s in _startingNumbers[..^1])
            {
                yield return s;
                _memory[s] = turn++;
            }

            var last = _startingNumbers.Last();
            yield return last;

            while (true)
            {
                var next = _memory.ContainsKey(last) ? turn - _memory[last] : 0;
                _memory[last] = turn++;
                last = next;
                yield return next;
            }
        }
    }
}