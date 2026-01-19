using System.Collections.Generic;

namespace HackerRank.ProblemSolving.LargeNumbers;

public class AVeryBigSum
{
    public static long aVeryBigSum(List<long> ar)
    {
        long sum = 0;
        
        foreach(var number in ar) sum += number;
        
        return sum;
    }
}