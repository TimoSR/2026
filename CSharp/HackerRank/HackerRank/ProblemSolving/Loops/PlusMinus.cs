using System;
using System.Collections.Generic;

namespace HackerRank.ProblemSolving.Loops;

public class PlusMinus
{
    public static void plusMinus(List<int> arr)
    {
        float positive = 0;
        float negative = 0;
        float zero = 0;
        int number_of_elements = arr.Count;
        
        foreach(var number in arr) {
            if(number > 0) positive++;
            if(number < 0) negative++;
            if(number == 0) zero++;
        }
        
        float positive_ratio = positive / number_of_elements;
        float negative_ratio = negative / number_of_elements;
        float zero_ratio = zero / number_of_elements;
        
        Console.WriteLine($"{positive_ratio:F6}");
        Console.WriteLine($"{negative_ratio:F6}");
        Console.WriteLine($"{zero_ratio:F6}");
    }
}