using System;
using System.Linq;

namespace _2
{
    public class PasswordPolicy
    {
        public readonly int Min;
        public readonly int Max;
        public readonly char Character;
        public readonly string Password;


        public PasswordPolicy(string input)
        {
            string[] items = input.Split(':');
            Password = items[1].TrimStart();
            items = items[0].Split(' ');
            Character = items[1][0];
            Min = Int32.Parse(items[0].Split('-')[0]);
            Max = Int32.Parse(items[0].Split('-')[1]);
        }

        public virtual bool IsValid()
        {
            int cnt = Password.Count(x => x == Character);
            return cnt <= Max && cnt >= Min;
        }
    }
}