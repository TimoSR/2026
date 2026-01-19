using System;
using System.Collections.Generic;

namespace HackerRank.ProblemSolving.Loops;

public class MiniMaxSum
{
    public static void miniMaxSum(List<int> arr)
    { 
        long sum = 0;
        int min = arr[0];
        int max = arr[0];
        
        for(int index = 0; index < arr.Count; index++) {
            var value = arr[index];
            sum += value;
            if (value < min) min = value;
            if (value > max) max = value;
        }
        
        long minSum = sum - max;
        long maxSum = sum - min;
        
        Console.WriteLine($"{minSum} {maxSum}");
    }
}