using System;
using System.Collections.Generic;
using System.Linq;
using System.Text;

namespace _17
{
    public class ConwayCubes
    {
        //If it doesnt contain a cube it must be inactive, mmm?
        private HashSet<Cube> _map;
        private Queue<(Cube, bool)> _updates;
        public int Length { get; private set; }
        public int Width { get; private set; }
        public int Height { get; private set; }
        
        public ConwayCubes(string[] input2d)
        {
            _map = new HashSet<Cube>();
            _updates = new Queue<(Cube, bool)>();
            Length = input2d.Length;
            Width = input2d[0].Length;
            Height = 1;
            //init map
            for (int y = 0; y < Length; y++)
                for (int x = 0; x < Width; x++)
                    if (input2d[y][x] == '#')
                        _map.Add(new Cube(x, y, 0, true));
        }

        //Get layer (virtual, because I only store active cubes)
        public bool[,] GetLayer(int z = 0)
        {
            var blks = _map.Where(x => x.Z == z).ToHashSet();
            bool[,] cubes = new bool[Length,Width];
            for (int y = 0; y < Length; y++)
                for (int x = 0; x < Width; x++)
                {
                    var c = new Cube(x, y, z);
                    if (blks.Contains(c))
                        cubes[y, x] = blks.First(cb => cb.Equals(c)).Active;
                    else
                        cubes[y, x] = false;
                    
                }
            return cubes;
        }

        public string PrintLayer(int z = 0)
        {
            var l = GetLayer(z);
            StringBuilder sb = new StringBuilder();
            for (int y = 0; y < Length; y++)
            {
                for (int x = 0; x < Width; x++)
                    sb.Append(l[y, x] ? '#' : '.');
                sb.Append('\n');
            }

            return sb.ToString();
        }

        public IEnumerable<Cube> Neighbors(Cube loc, int d = 1)
        {
            for (int x = -d; x <= d; x++)
                for (int y = -d; y <= d; y++)
                    for (int z = -d; z <= d; z++)
                    {
                        var nLoc = new Cube(loc.X + x, loc.Y + y, loc.Z + z);
                        if (!loc.Equals(nLoc))
                        {
                            if (_map.Contains(nLoc))
                                yield return nLoc;
                            else
                                yield return new Cube(x, y, z);
                        }
                    }
        }

        public void ExecuteAll()
        {
            for (int z = 0; z < Height; z++)
                for (int y = 0; y < Length; y++)
                    for (int x = 0; x < Width; x++)
                        Execute(new Cube(x,y,z));
        }

        public void Execute(Cube cube)
        {
            int cnt = Neighbors(cube).Count(x => x.Active);
            Console.WriteLine(cnt);
            if (cube.Active)
            {
                if(cnt == 2 || cnt == 3)
                    _updates.Enqueue((cube, true));
                else
                    _updates.Enqueue((cube, false));
            }
            else
            {
                if(cnt == 3)
                    _updates.Enqueue((cube, true));
            }
        }

        public void Update()
        {
            Console.WriteLine(_map.Count(x => x.Active));
            while (_updates.Any())
            {
                var c = _updates.Dequeue();
                c.Item1.Active = c.Item2;
                if (_map.Contains(c.Item1))
                    _map.Remove(c.Item1);
                
                if(c.Item2)
                    _map.Add(c.Item1);
            }
            Console.WriteLine(_map.Count(x => x.Active));
        }
    }
}