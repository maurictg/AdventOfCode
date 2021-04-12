namespace _17
{
    public class Cube
    {
        public int X { get; }
        public int Y { get; }
        public int Z { get; }
        public bool Active { get; set; }

        public Cube(int x, int y, int z, bool active = false)
        {
            X = x;
            Y = y;
            Z = z;
            Active = active;
        }

        public override int GetHashCode()
            => X ^ Y ^ Z;

        public override bool Equals(object? obj)
        {
            if (obj == null) return false;
            if (obj.GetType() != GetType()) return false;
            var o = (Cube) obj;
            return o.X == X && o.Y == Y && o.Z == Z;
        }

        public override string ToString()
            => $"{X}, {Y}, {Z}: {(Active ? '#' : '.')}";
    }
}