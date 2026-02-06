using System;
using System.Collections.Generic;

public class GradingStudents()
{
    private static readonly Func<int, bool> IsFailing = grade => grade < 38;
    private static readonly Func<int, bool> IsMultipleOfFive = grade => grade % 5 == 0;

    public static List<int> GradingStudentsMethodWithDelegates(List<int> grades)
    {
        if (grades == null)
        {
            throw new ArgumentNullException(nameof(grades));
        }

        var roundedGrades = new List<int>(grades.Count);

        foreach (var grade in grades)
        {
            if (IsFailing(grade))
            {
                roundedGrades.Add(grade);
                continue;
            }

            if (!IsMultipleOfFive(grade))
            {
                var remainder = grade % 5;
                var delta = 5 - remainder;

                if (delta <= 2)
                {
                    roundedGrades.Add(grade + delta);
                    continue;
                }
            }

            roundedGrades.Add(grade);
        }

        return roundedGrades;
    }

    public static List<int> GradingStudentsMethodWithLamdas(List<int> grades)
    {
        if (grades == null)
        {
            throw new ArgumentNullException(nameof(grades));
        }

        var isFailing = (int grade) => grade < 38;
        var isMultipleOfFive = (int grade) => grade % 5 == 0;

        var roundedGrades = new List<int>(grades.Count);

        foreach (var grade in grades)
        {
            if (isFailing(grade))
            {
                roundedGrades.Add(grade);
                continue;
            }

            if (!isMultipleOfFive(grade))
            {
                var remainder = grade % 5;
                var delta = 5 - remainder;

                if (delta <= 2)
                {
                    roundedGrades.Add(grade + delta);
                    continue;
                }
            }

            roundedGrades.Add(grade);
        }

        return roundedGrades;
    }
}