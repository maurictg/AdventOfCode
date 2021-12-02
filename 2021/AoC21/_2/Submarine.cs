using System;

namespace _2
{
    public class Submarine
    {
        public int X { get; private set; }
        public int Depth { get; private set; }
        private int Aim { get; set; }
        private readonly bool _aimMode;

        public Submarine(int x = 0, int d = 0, bool aimMode = false)
        {
            (X, Depth) = (0, 0);
            Aim = 0;
            _aimMode = aimMode;
        }

        public void Go(params string[] instructions)
        {
            foreach (var instruction in instructions)
            {
                var instr = instruction.Split(' ', StringSplitOptions.TrimEntries);
                var code = Convert.ToInt32(instr[1]);
                switch (instr[0])
                {
                    case "forward":
                        X += code;
                        if(_aimMode) Depth += Aim * code;
                        break;
                    case "down":
                        if (_aimMode) Aim += code;
                        else Depth += code;
                        break;
                    case "up":
                        if (_aimMode) Aim -= code;
                        else Depth -= code;
                        break;
                }
            }
        }
    }
}