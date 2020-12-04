using System;
using System.Collections.Generic;
using System.ComponentModel.DataAnnotations;
using System.Reflection;

namespace _4
{
    [AttributeUsage(AttributeTargets.Property)]
    public class Name : Attribute
    {
        public string Value { get; }

        public Name(string name)
            => Value = name;
    }
    
    public class Passport
    {
        [Name("ecl"), Required, RegularExpression("amb|blu|brn|gry|grn|hzl|oth")]
        public string EyeColor { get; set; }
        
        [Name("hcl"), Required, RegularExpression("^#([a-fA-F0-9]{6})$")]
        public string HairColor { get; set; }
        
        [Name("pid"), Required, RegularExpression("^\\d{9}$")]
        public string PassportId { get; set; }
        
        [Name("eyr"), Required, Range(2020, 2030)]
        public int ExpirationYear { get; set; }
        
        [Name("iyr"), Required, Range(2010, 2020)]
        public int IssueYear { get; set; }
        
        [Name("byr"), Required, Range(1920, 2002)]
        public int BirthYear { get; set; }
        
        [Name("hgt"), Required, RegularExpression("^(1[5-8][0-9]|19[0-3])cm|(59|6[0-9]|7[0-6])in")]
        public string Height { get; set; }
        
        [Name("cid")]
        public string CountryId { get; set; }
        
        public Dictionary<string, string> Items;
        private readonly string[] _requiredItems = { "ecl", "pid", "eyr", "hcl", "byr", "iyr", "hgt" };
        private bool _strict;
        
        public bool Valid
        {
            get
            {
                if (Items.Count < 7) return false;
                foreach (var item in _requiredItems)
                    if (!Items.ContainsKey(item))
                        return false;

                if (_strict)
                {
                    var ctx = new ValidationContext(this);
                    var res = new List<ValidationResult>();
                    if (!Validator.TryValidateObject(this, ctx, res, true))
                    {
                        //foreach (var err in res)
                        //    Console.Error.WriteLine($"{err}");
                        return false;
                    }
                }

                return true;
            }
        }

        public Passport(string input, bool strict = false)
        {
            this._strict = strict;
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
            this._strict = true;
            Type t = typeof(Passport);
            foreach (var m in t.GetMembers())
            {
                foreach (var attr in m.GetCustomAttributes(true))
                {
                    if (attr is Name)
                    {
                        PropertyInfo prop = t.GetProperty(m.Name);
                        string key = (attr as Name).Value;
                        if (!Items.ContainsKey(key))
                            continue;
                        
                        if (Type.GetTypeCode(prop?.PropertyType) == TypeCode.Int32)
                            prop?.SetValue(this, Convert.ToInt32(Items[key]));
                        else
                            prop?.SetValue(this, Items[key]);
                    }
                }
            }
        }
    }
}