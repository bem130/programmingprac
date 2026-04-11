using ProgrammingPrac.Problems;

namespace ProgrammingPrac.Tests;

public class P01_P10Tests
{
    // p01
    [Fact] public void P01_World() => Assert.Equal("Hello, World!", P01Hello.Greet("World"));
    [Fact] public void P01_Rust() => Assert.Equal("Hello, Rust!", P01Hello.Greet("Rust"));
    [Fact] public void P01_Empty() => Assert.Equal("Hello, !", P01Hello.Greet(""));

    // p02
    [Fact] public void P02_Fizz() => Assert.Equal("Fizz", P02Fizzbuzz.Fizzbuzz(3));
    [Fact] public void P02_Buzz() => Assert.Equal("Buzz", P02Fizzbuzz.Fizzbuzz(5));
    [Fact] public void P02_FizzBuzz() => Assert.Equal("FizzBuzz", P02Fizzbuzz.Fizzbuzz(15));
    [Fact] public void P02_Number() => Assert.Equal("7", P02Fizzbuzz.Fizzbuzz(7));

    // p03
    [Fact] public void P03_Basic() => Assert.Equal(6, P03Sum.Sum([1, 2, 3]));
    [Fact] public void P03_Empty() => Assert.Equal(0, P03Sum.Sum([]));
    [Fact] public void P03_Negative() => Assert.Equal(0, P03Sum.Sum([-1, 1, -2, 2]));

    // p04
    [Fact] public void P04_Hello() => Assert.Equal("olleh", P04Reverse.Reverse("hello"));
    [Fact] public void P04_Empty() => Assert.Equal("", P04Reverse.Reverse(""));
    [Fact] public void P04_Single() => Assert.Equal("a", P04Reverse.Reverse("a"));

    // p05
    [Fact] public void P05_Racecar() => Assert.True(P05Palindrome.IsPalindrome("racecar"));
    [Fact] public void P05_Madam() => Assert.True(P05Palindrome.IsPalindrome("Madam"));
    [Fact] public void P05_Hello() => Assert.False(P05Palindrome.IsPalindrome("hello"));
    [Fact] public void P05_Empty() => Assert.True(P05Palindrome.IsPalindrome(""));

    // p06
    [Fact] public void P06_Zero() => Assert.Equal(0L, P06Fibonacci.Fibonacci(0));
    [Fact] public void P06_One() => Assert.Equal(1L, P06Fibonacci.Fibonacci(1));
    [Fact] public void P06_Ten() => Assert.Equal(55L, P06Fibonacci.Fibonacci(10));
    [Fact] public void P06_Twenty() => Assert.Equal(6765L, P06Fibonacci.Fibonacci(20));

    // p07
    [Fact]
    public void P07_Hello()
    {
        var r = P07CountChars.CountChars("Hello");
        Assert.Equal(1, r['h']);
        Assert.Equal(2, r['l']);
        Assert.Equal(1, r['o']);
    }

    [Fact]
    public void P07_Spaces()
    {
        var r = P07CountChars.CountChars("A a");
        Assert.Equal(2, r['a']);
        Assert.Single(r);
    }

    [Fact] public void P07_Empty() => Assert.Empty(P07CountChars.CountChars(""));

    // p08
    [Fact] public void P08_Basic() => Assert.Equal(new[] { 1, 2, 3 }, P08Unique.Unique([1, 2, 3, 2, 1]));
    [Fact] public void P08_AllSame() => Assert.Equal(new[] { 5 }, P08Unique.Unique([5, 5, 5]));
    [Fact] public void P08_Empty() => Assert.Equal(Array.Empty<int>(), P08Unique.Unique([]));

    // p09
    [Fact] public void P09_Odd() => Assert.Equal(2.0, P09Median.Median([3, 1, 2]));
    [Fact] public void P09_Even() => Assert.Equal(2.5, P09Median.Median([4, 1, 3, 2]));
    [Fact] public void P09_Empty() => Assert.Equal(0.0, P09Median.Median([]));
    [Fact] public void P09_Single() => Assert.Equal(42.0, P09Median.Median([42]));

    // p10
    [Fact] public void P10_Add() => Assert.Equal(7L, P10Rpn.EvalRpn("3 4 +"));
    [Fact] public void P10_Complex() => Assert.Equal(14L, P10Rpn.EvalRpn("5 1 2 + 4 * + 3 -"));
    [Fact] public void P10_Divide() => Assert.Equal(2L, P10Rpn.EvalRpn("4 2 /"));
    [Fact] public void P10_Empty() => Assert.Null(P10Rpn.EvalRpn(""));
    [Fact] public void P10_Invalid() => Assert.Null(P10Rpn.EvalRpn("1 +"));
}
