using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace _11
{
    public class SeatMap
    {
        /* == Directions ==
        * 0.0  0.1  0.2
        * 1.0  1.1  1.2
        * 2.0  2.1  2.2
        */
        private readonly char[,] _map;
        private readonly int _width;
        private readonly int _height;
        private Queue<(int, int, char)> _updates;
        private readonly (int,int)[] _directions = 
            { (-1, -1), (0, -1), (1, -1), (1, 1), (-1, 0), (1, 0), (-1, 1), (0, 1)};

        public char At(int x, int y) =>
            (x < _width && y < _height && x >= 0 && y >= 0) ? _map[y, x] : '_';

        private bool Is(int x, int y, char val, bool ignoreFloor = true, bool ignoreNonExistent = true)
            => At(x, y) == val || (ignoreNonExistent && At(x,y) == '_') || (ignoreFloor && At(x,y) == '.');

        public SeatMap(string[] input)
        {
            _updates = new Queue<(int, int, char)>();
            _height = input.Length;
            _width = input.Max(x => x.Length);
            _map = new char[_height, _width];
            for (int i = 0; i < _height; i++)
                for (int j = 0; j < _width; j++)
                    _map[i, j] = input[i][j];
        }

        /* Checks
         * x-,y-| y- |x+,y-
         * x-   | 0  |  x+
         * x-,y+| y+ |x+,y+
         */
        private void ApplyRule(int x, int y)
        {
            char s = At(x, y); if(s == '_' || s == '.') return;
            if (s == 'L' && _directions.All(i => Is(x + i.Item1, y + i.Item2, 'L')))
                _updates.Enqueue((x,y,'#'));
            else if (s == '#' && _directions.Count(i => Is(x + i.Item1, y + i.Item2, '#', false, false)) >= 4)
                _updates.Enqueue((x,y,'L'));
        }

        private void ApplyNewRule(int x, int y)
        {
            char s = At(x, y); if(s == '_' || s == '.') return;
            int seenCount = 0;
            foreach (var d in _directions)
            {
                int xt = x;
                int yt = y;
                while (true)
                {
                    xt += d.Item1;
                    yt += d.Item2;
                    if (At(xt, yt) == '#') {
                        seenCount++;
                        break;
                    }
                    
                    if(At(xt, yt) == '_' || At(xt, yt) == 'L')
                        break;
                }
            }
            
            if(seenCount == 0)
                _updates.Enqueue((x,y,'#'));
            else if(seenCount >= 5)
                _updates.Enqueue((x,y,'L'));
        }

        public void ApplyRules(bool useNewRules = false)
        {
            for (int i = 0; i < _height; i++)
                for (int j = 0; j < _width; j++)
                    if(useNewRules) ApplyNewRule(j,i);
                    else ApplyRule(j,i);
        }

        public int Update()
        {
            int changes = 0;
            while (_updates.Any())
            {
                var u = _updates.Dequeue();
                changes += _map[u.Item2, u.Item1] == u.Item3 ? 0 : 1;
                _map[u.Item2, u.Item1] = u.Item3;
            }
            return changes;
        }

        public int Count(char item)
        {
            int cnt = 0;
            for (int i = 0; i < _height; i++)
                for (int j = 0; j < _width; j++)
                    cnt += Is(j, i, item, false, false) ? 1 : 0;
            return cnt;
        }

        public override string ToString()
        {
            StringBuilder sb = new StringBuilder();
            for (int i = 0; i < _height; i++)
            {
                for (int j = 0; j < _width; j++)
                    sb.Append(_map[i, j]);
                sb.Append('\n');
            }

            return sb.ToString();
        }
    }
}