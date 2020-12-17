using System;
using System.Collections.Generic;
using System.Linq;

namespace _13
{
    public class BusStop
    {
        public int EarliestTimestamp { get; }
        private int[] _buses;
        private int _timestamp;
        
        public BusStop(string[] input, bool useEarliestTimestamp = true)
        {
            EarliestTimestamp = useEarliestTimestamp ? Convert.ToInt32(input[0]) : 1;
            _timestamp = EarliestTimestamp;
            _buses = input[1].Replace("x", "1").Split(',')
                .Select(x => Convert.ToInt32(x)).ToArray();
        }

        public (int time, int id) Next()
        {
            while (true)
            {
                var bus = _buses.FirstOrDefault(x => x != 1 && _timestamp % x == 0);
                _timestamp++;
                if (bus > 0)
                    return (_timestamp - 1, bus);
            }
        }

        /*
        //This is for sure too heavy
        public IEnumerable<long> MaskMatches(long offset = 1)
        {
            long inc = offset;
            while (true)
            {
                var expr = _buses.All(x => inc++ % x == 0);
                if (expr)
                    yield return inc - _buses.Length;
            }
        }*/
    }
}