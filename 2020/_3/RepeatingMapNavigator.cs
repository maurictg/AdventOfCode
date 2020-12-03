using System;

namespace _3
{
    public class RepeatingMapNavigator
    {
        private string[] pattern;
        private string[] map;
        private (int x, int y) location;

        private int Width;
        private int Height;

        public RepeatingMapNavigator(string[] pattern) : this(pattern, (0, 0)) {}
        public RepeatingMapNavigator(string[] pattern, (int, int) startingPoint)
        {
            this.pattern = pattern;
            map = pattern;
            location = startingPoint;
            Width = map[0].Length;
            Height = map.Length;
        }

        public void Reset()
        {
            location = (0, 0);
            map = pattern;
            Width = map[0].Length;
            Height = map.Length;
        }

        public char At() => At(location);
        public char At((int x, int y) point) => map[point.y][point.x];
        public bool Exists((int x, int y) point) => point.y < Height && point.x < Width;

        public void Extend()
        {
            for (int i = 0; i < Height; i++)
                map[i] += pattern[i];
            this.Width *= 2;
        }

        public bool NavigateTo((int x, int y) point)
        {
            if (Exists(point))
                location = point;
            return Exists(point);
        }
        
        public bool Navigate(int x, int y)
        {
            var nextPoint = (location.x + x, location.y + y);
            //Console.WriteLine(nextPoint);
            
            if (!Exists(nextPoint))
            {
                if (nextPoint.Item2 < Height)
                {
                    Extend();
                    return Navigate(x, y);
                }

                return false;
            }
            
            location = nextPoint;
            return true;
        }
        
    }
}