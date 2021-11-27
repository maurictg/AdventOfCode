using System.Text;
using Utils;

namespace _5
{
    public class Polymer
    {
        private StringBuilder _str;
        
        public Polymer(string polymer)
        {
            _str = new StringBuilder(polymer);
        }

        public int Length => _str.Length;

        public Polymer Without(char letter)
        {
            var low = char.ToLower(letter);
            var up = char.ToUpper(letter);

            _str = _str
                .Replace(low.ToString(), null)
                .Replace(up.ToString(), null);
            return this;
        }

        public Polymer React()
        {
            var i = 0;
            while (true)
            {
                if (i < _str.Length - 1)
                {
                    if (_str[i] == _str[i + 1].Toggle())
                    {
                        _str.Remove(i, 2);
                        if (i > 0) i--;
                    }
                    else
                        i++;
                }
                else
                    break;
            }

            return this;
        }
    }
}