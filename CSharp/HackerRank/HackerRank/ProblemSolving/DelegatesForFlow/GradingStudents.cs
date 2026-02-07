using System;
using System.Collections.Generic;

public class GradingStudents()
{
    private static readonly Func<int, bool> IsFailing = grade => grade < 38;
    private static readonly Func<int, bool> IsPassing = grade => !IsFailing(grade);
    private static readonly Func<int, bool> IsMultipleOfFive = grade => grade % 5 == 0;
    private static readonly Func<int, bool> IsNotMultipleOfFive = grade => !IsMultipleOfFive(grade);
    private static readonly Func<int, bool> GradeShouldBeRounded = grade => IsPassing(grade) && IsNotMultipleOfFive(grade);

    public static int Add(int a, int b) => a + b;

    public Func<int, int, int> AddDelegate = (a, b) =>
    {
        var result = a + b;
        return result;
    };

    public static List<int> GradingStudentsMethodWithDelegates(List<int> grades)
    {
        var roundedGrades = new List<int>(grades.Count);

        foreach (var grade in grades)
        {   
            var result = grade;

            if (GradeShouldBeRounded(result))
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
        var isPassing = (int grade) => !isFailing(grade);
        var isMultipleOfFive = (int grade) => grade % 5 == 0;
        var isNotMultipleOfFive = (int grade) => !isMultipleOfFive(grade);
        var gradeShouldBeRounded = (int grade) => isPassing(grade) && isNotMultipleOfFive(grade);

        Func<int, bool> isFailingDelegateExample = grade =>
        {
            if (grade < 0)
                throw new ArgumentOutOfRangeException(nameof(grade));

            return grade < 38;
        };

        var roundedGrades = new List<int>(grades.Count);

        foreach (var grade in grades)
        {   
            var result = grade;

            if (gradeShouldBeRounded(result))
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