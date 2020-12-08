using System;
using System.Collections.Generic;
using System.Linq;

namespace _8
{
    public class Interpreter
    {
        public int Accumulator { get; private set; }
        private string[] instructions;
        private bool[] state;

        public Interpreter(string[] instructions)
        {
            this.instructions = instructions;
            state = new bool[instructions.Length];
        }

        public void Reverse(int idx)
        {
            instructions[idx] = instructions[idx].Contains("jmp")
                ? instructions[idx].Replace("jmp", "nop")
                : instructions[idx].Replace("nop", "jmp");
        }
        
        public (int, bool) Execute(bool loop = false)
        {
            Accumulator = 0;
            int idx = 0;
            while (idx < instructions.Length + 1)
            {
                if (idx == instructions.Length)
                {
                    idx--; 
                    continue;
                }

                if (state[idx])
                {
                    //If not looping until error found return place of error
                    if(!loop) return (Accumulator, idx != instructions.Length - 1);

                    Stack<int> changeable = new Stack<int>(instructions
                        .Where(x => x.Contains("nop") || x.Contains("jmp"))
                        .Select(x => Array.IndexOf(instructions, x)));

                    while (changeable.Any())
                    {
                        int c = changeable.Pop();
                        Reverse(c);
                        var res = new Interpreter(instructions).Execute();
                        if (res.Item2) Reverse(c); //reverse it back
                        else return (res.Item1, false); //return result
                    }
                }

                state[idx] = true;
                int amount = Convert.ToInt32(instructions[idx][4..].Replace("+", ""));
                
                switch (instructions[idx][..3])
                {
                    case "nop": idx++; break;
                    case "acc":
                        Accumulator += amount;
                        idx++;
                        break;
                    case "jmp": idx += amount; break;
                }
            }

            return (Accumulator, false);
        }
    }
}