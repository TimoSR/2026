using System;
using System.Collections.Generic;

namespace HackerRank.ProblemSolving.Loops;

public class DiagonalDifference
{
    public static int diagonalDifference(List<List<int>> arr)
    {
        var sum_diagonal_down = 0;
        var sum_diagonal_up = 0;
        var max_index = arr.Count - 1;
        
        for(var index = 0; index < arr.Count; index++) {
            sum_diagonal_down += arr[index][index];
            sum_diagonal_up += arr[index][max_index - index];
        }
        
        var absolute_dif = Math.Abs(sum_diagonal_down - sum_diagonal_up);
        
        return absolute_dif;
    }
}