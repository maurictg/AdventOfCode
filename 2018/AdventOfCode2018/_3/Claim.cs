using System;
using System.Collections.Generic;
using System.Text.RegularExpressions;

namespace _3
{
    public struct Claim
    {
        private static readonly Regex Regex = new Regex(@"^#(\d+)\s@\s(\d+),(\d+):\s(\d+)x(\d+)$");
        
        public int Id { get; set; }
        public int X { get; set; }
        public int Y { get; set; }
        public int Width { get; set; }
        public int Height { get; set; }

        public Claim(string format)
        {
            var match = Regex.Match(format);
            Id = Convert.ToInt32(match.Groups[1].Value);
            X = Convert.ToInt32(match.Groups[2].Value);
            Y = Convert.ToInt32(match.Groups[3].Value);
            Width = Convert.ToInt32(match.Groups[4].Value);
            Height = Convert.ToInt32(match.Groups[5].Value);
        }

        public IEnumerable<(int x, int y)> Coordinates()
        {
            for (var y = Y; y < Y + Height; y++)
                for (var x = X; x < X + Width; x++)
                    yield return (x,y);
        }
    }
}