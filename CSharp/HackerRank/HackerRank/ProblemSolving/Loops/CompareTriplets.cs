using System.Collections.Generic;

namespace HackerRank.ProblemSolving.Loops;

public class CompareTriplets
{
    public static List<int> compareTriplets(List<int> a, List<int> b)
    {
        var alice_points = 0;
        var bob_points = 0;
        
        for(var index = 0; index < a.Count; index++) {
            var value_alice = a[index];
            var value_bob = b[index];
            
            if(value_alice > value_bob) alice_points++;
            if(value_bob > value_alice) bob_points++;
        }
        
        return [alice_points, bob_points];
    }
}