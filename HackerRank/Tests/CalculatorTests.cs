using HackerRank;
using Xunit.Abstractions;

namespace Tests;

public class CalculatorTests(ITestOutputHelper testOutputHelper)
{
    private readonly ICalculator _calculator = new Calculator();

    [Fact]
    public void AddingTwoNumbers()
    {
        var result = _calculator.Add(2, 2);
        testOutputHelper.WriteLine(result.ToString());
        Assert.Equal(4, result);
    }
    
    [Fact]
    public void SubtractingTwoNumbers()
    {
        var result = _calculator.Subtract(2, 2);
        Assert.Equal(0, result);
    }
    
    [Fact]
    public void MultiplyingTwoNumbers()
    {
        var result = _calculator.Multiply(2, 2);
        Assert.Equal(4, result);
    }

    [Fact]
    public void DividingTwoNumbers()
    {
        var result = _calculator.Divide(2, 2);
        Assert.Equal(1, result);
    }

    [Fact]
    public void CantDivideByZero()
    {
        Assert.Throws<DivideByZeroException>(() => _calculator.Divide(0, 0));
    }
}