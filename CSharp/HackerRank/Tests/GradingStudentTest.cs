using System.Collections.Generic;
using Xunit;

public class GradingStudentsTests
{
    [Fact]
    public void Grades_Are_Rounded_According_To_Rules_With_Delegates()
    {
        // Arrange
        var input = new List<int> { 73, 67, 38, 33 };

        // Act
        var result = GradingStudents.GradingStudentsMethodWithDelegates(input);

        // Assert
        Assert.Equal(
            new List<int> { 75, 67, 40, 33 },
            result
        );
    }

    [Fact]
    public void Grades_Are_Rounded_According_To_Rules_With_Lamdas()
    {
        // Arrange
        var input = new List<int> { 73, 67, 38, 33 };

        // Act
        var result = GradingStudents.GradingStudentsMethodWitHLamdas(input);

        // Assert
        Assert.Equal(
            new List<int> { 75, 67, 40, 33 },
            result
        );
    }
}
