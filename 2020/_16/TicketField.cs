namespace _16
{
    public class TicketField
    {
        public int A, B, C, D;
        public TicketField(int a, int b, int c, int d)
        {
            A = a; B = b; C = c; D = d;
        }

        public bool IsMatch(int value)
            => (value >= A && value <= B) || (value >= C && value <= D);
    }
}