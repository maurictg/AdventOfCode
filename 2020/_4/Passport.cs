using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;

namespace _4
{
    public class Passport
    {
        [Required, RegularExpression("amb|blu|brn|gry|grn|hzl|oth")]
        public string EyeColor { get; private set; }
        
        [Required, RegularExpression("^#([a-fA-F0-9]{6})$")]
        public string HairColor { get; private set; }
        
        [Required, RegularExpression("^\\d{9}$")]
        public string PassportId { get; private set; }
        
        [Required, Range(2020, 2030)]
        public int ExpirationYear { get; private set; }
        
        [Required, Range(2010, 2020)]
        public int IssueYear { get; private set; }
        
        [Required, Range(1920, 2002)]
        public int BirthYear { get; private set; }
        
        [Required, RegularExpression("^(1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in")]
        public string Height { get; private set; }
        public string CountryId { get; private set; }
        
        public Dictionary<string, string> Items;
        private readonly string[] requiredItems = { "ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt" };
        private bool strict;
        
        public bool Valid
        {
            get
            {
                if (Items.Count < 7) return false;
                foreach (var item in requiredItems)
                    if (!Items.ContainsKey(item))
                        return false;

                if (strict)
                {
                    var ctx = new ValidationContext(this);
                    var res = new List<ValidationResult>();
                    if (!Validator.TryValidateObject(this, ctx, res, true))
                    {
                        foreach (var err in res)
                            Console.Error.WriteLine($"{err}");
                        return false;
                    }
                }

                return true;
            }
        }

        public Passport(string input, bool strict = false)
        {
            this.strict = strict;
            Items = new Dictionary<string, string>();
            string[] values = input.Replace(Environment.NewLine, " ")
                .Split(" ");

            foreach (var v in values)
            {
                int idx = v.IndexOf(':');
                string key = v.Substring(0, idx);
                string value = v.Substring(idx + 1);
                Items.Add(key, value);
            }
            
            if(strict)
                Parse();
        }

        public void Parse()
        {
            this.strict = true;
            foreach (var item in Items)
            {
                string key = item.Key;
                string value = item.Value;
                switch (key)
                {
                    case "byr":
                        BirthYear = Convert.ToInt32(value);
                        break;
                    case "iyr":
                        IssueYear = Convert.ToInt32(value);
                        break;
                    case "eyr":
                        ExpirationYear = Convert.ToInt32(value);
                        break;
                    case "hgt":
                        Height = value;
                        break;
                    case "hcl":
                        HairColor = value;
                        break;
                    case "ecl":
                        EyeColor = value;
                        break;
                    case "pid":
                        PassportId = value;
                        break;
                    case "cid":
                        CountryId = value;
                        break;
                    default:
                        Console.WriteLine("Unknown key: " + key);
                        break;
                }
            }
        }
    }
}