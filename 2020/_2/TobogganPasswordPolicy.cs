using System;

namespace _2
{
    public class TobogganPasswordPolicy : PasswordPolicy
    {
        public TobogganPasswordPolicy(string input) : base(input) {}
        public override bool IsValid()
        {
            try
            {
                return Password[Min - 1] == Character && Password[Max - 1] != Character
                       || Password[Min - 1] != Character && Password[Max - 1] == Character;
            }
            catch (IndexOutOfRangeException)
            {
                return false;
            }
        }
    }
}