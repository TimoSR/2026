using System;
using System.Collections.Generic;

public class GradingStudents()
{
    private static readonly Func<int, bool> IsFailing = grade => grade < 38;
    private static readonly Func<int, bool> IsNotFailing = grade => !IsFailing(grade);
    private static readonly Func<int, bool> IsMultipleOfFive = grade => grade % 5 == 0;
    private static readonly Func<int, bool> IsNotMultipleOfFive = grade => !IsMultipleOfFive(grade);

    public static int Add(int a, int b) => a + b;

    public static List<int> GradingStudentsMethodWithDelegates(List<int> grades)
    {
        var roundedGrades = new List<int>(grades.Count);

        foreach (var grade in grades)
        {   
            var result = grade;

            if(IsNotFailing(result) && IsNotMultipleOfFive(result))
            {
                var remainder = result % 5;

                var delta = 5 - remainder;

                if (delta < 3)
                {
                    result += delta;
                }
            }
            
            roundedGrades.Add(result);
        }

        return roundedGrades;
    }

    public static List<int> GradingStudentsMethodWithLamdas(List<int> grades)
    {
        var isFailing = (int grade) => grade < 38;
        var isNotFailing = (int grade) => !isFailing(grade);
        var isMultipleOfFive = (int grade) => grade % 5 == 0;
        var isNotMultipleOfFive = (int grade) => !isMultipleOfFive(grade);

        var roundedGrades = new List<int>(grades.Count);

        foreach (var grade in grades)
        {   
            var result = grade;

            if(isNotFailing(result) && isNotMultipleOfFive(result))
            {
                var remainder = result % 5;

                var delta = 5 - remainder;

                if (delta < 3)
                {
                    result += delta;
                }
            }
            
            roundedGrades.Add(result);
        }

        return roundedGrades;
    }
}