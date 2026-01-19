using System.Collections.Generic;

namespace HackerRank.ProblemSolving.Loops;

public class SimpleArraySum
{
    public static int simpleArraySum(List<int> ar)
    {
        var sum = 0;
        
        foreach(var value in ar) sum += value;
        
        return sum;
    }
}