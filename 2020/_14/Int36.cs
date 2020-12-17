using System;
using System.Collections;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace _14
{
    public class Int36
    {
        private BitArray _bits;
        public long Value
        {
            get
            {
                byte[] b = new byte[_bits.Length / 8];
                _bits.CopyTo(b, 0);
                return BitConverter.ToInt64(b);
            }
        }

        public Int36(string bits) : this(Convert.ToInt64(bits, 2)) {}
        public Int36(long value)
            => _bits = new BitArray(BitConverter.GetBytes(value));
        public Int36(long value, string mask) : this(value)
            => ApplyMask(mask);

        public Int36(BitArray bits)
            => _bits = bits;

        public Int36 Clone()
            => new Int36(Value);

        public void SetBit(int idx, bool value)
            => _bits[idx] = value;
        
        public void ApplyMask(string maskString)
        {
            char[] mask = maskString.Reverse().ToArray();
            for (int i = 0; i < mask.Length; i++)
            {
                if (mask[i] == '0')
                    _bits[i] = false;
                else if (mask[i] == '1')
                    _bits[i] = true;
            }
        }

        public Int36[] ApplyMaskV2(string maskString)
        {
            char[] mask = maskString.Reverse().ToArray();
            List<int> flucBits = new List<int>();
            for (int i = 0; i < mask.Length; i++)
            {
                if (mask[i] == '1')
                    _bits[i] = true;
                else if(mask[i] == 'X')
                    flucBits.Add(i);
            }

            List<Int36> possibilities = new List<Int36>();
            int p = 0;
            for(int i = 0; i < Math.Pow(2, flucBits.Count); i++)
            {
                //create binary string from a counter int to make all binary combinations (e.g. [0,0], [0,1], [1,0], [1,1])
                string bits = Convert.ToString(p++, 2).PadLeft(flucBits.Count, '0');
                Int36 nr = Clone();
                for (int j = 0; j < flucBits.Count; j++)
                    nr.SetBit(flucBits[j], bits[j] == '1');
                possibilities.Add(nr);
            }

            return possibilities.ToArray();
        }

        //custom (casting) operators, great C# feature
        public static explicit operator Int36(long val) => new Int36(val);

        public override string ToString()
            => Convert.ToString(Value, 2).PadLeft(36, '0');
    }
}