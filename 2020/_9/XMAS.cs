using System;
using System.Collections.Generic;
using System.Linq;

namespace _9
{
    public class XMAS
    {
        private long[] _data;
        private int _preambleSize;

        public XMAS(long[] data, int preambleSize = 25)
        {
            _data = data;
            _preambleSize = preambleSize;
        }

        public IEnumerable<long> GetInvalidNumbers()
        {
            for (int i = _preambleSize; i < _data.Length; i++)
            {
                var preamble = 
                    _data.Skip(i - _preambleSize).Take(_preambleSize).ToList();

                long nr = _data[i];
                if (preamble.All(x => 
                    !preamble.Any(y => x + y == nr && x != y && x != nr && y != nr)))
                    yield return nr;
            }
        }

        public long[] FindWeakness(long invalidNumber)
        {
            for (int i = 0; i < _data.Length; i++)
            {
                long sum = 0;
                for (int j = i; j < _data.Length - i; j++)
                {
                    sum += _data[j];
                    if (sum == invalidNumber)
                        return _data[i..(j+1)];
                    
                    if(sum > invalidNumber) 
                        break;
                }
            }
            
            return null;
        }
    }
}