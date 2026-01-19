using System;

namespace HackerRank.ProblemSolving.Strings;

public class StairCase
{
    public static void staircase(int n)
    {
        for(int block_count = 1; block_count <= n ; block_count++)
        {
            var space = n - block_count;
            var stair_step_empty = new string(' ', space);
            var stair_step_block = new string('#', block_count);
            var stair_step = stair_step_empty + stair_step_block;
            Console.WriteLine(stair_step);
        }
    }
}