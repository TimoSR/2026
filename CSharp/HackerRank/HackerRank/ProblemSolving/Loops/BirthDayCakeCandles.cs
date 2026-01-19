using System;
using System.Collections.Generic;

namespace HackerRank.ProblemSolving.Loops;

public class BirthDayCakeCandles
{
    //  O(N) // "O(2N) / O(N + N)": Running time is 2N, but will be accepted as 2N
    public static int birthdayCakeCandles(List<int> candles)
    {
        var tallest_candle = candles[0];
        var max_height_candles = 0;
        
        foreach (var candle in candles)
        {
            if (candle > tallest_candle) 
                tallest_candle = candle;
        }
        foreach (var candle in candles)
        {
            if (candle == tallest_candle)
                max_height_candles++;
        }
            
        return max_height_candles;
    }

    // GPT: O(N)
    public static int GPTBirthdayCakeCandle(List<int> candles)
    {
        if (candles is null) throw new ArgumentNullException(nameof(candles));
        if (candles.Count == 0) return 0;

        var max = int.MinValue;
        var count = 0;

        for (var i = 0; i < candles.Count; i++)
        {
            var height = candles[i];

            if (height > max)
            {
                max = height;
                count = 1;
            }
            else if (height == max)
            {
                count++;
            }
        }

        return count;
    }
}