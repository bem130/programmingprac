using ProgrammingPrac.Problems;

namespace ProgrammingPrac.Tests;

public class P16_P25Tests
{
    // p16
    [Fact] public void P16_Found() => Assert.Equal(2, P16BinarySearch.BinarySearch([1, 3, 5, 7, 9], 5));
    [Fact] public void P16_NotFound() => Assert.Equal(-1, P16BinarySearch.BinarySearch([1, 3, 5, 7, 9], 4));
    [Fact] public void P16_Empty() => Assert.Equal(-1, P16BinarySearch.BinarySearch([], 1));
    [Fact] public void P16_First() => Assert.Equal(0, P16BinarySearch.BinarySearch([1, 3, 5, 7, 9], 1));
    [Fact] public void P16_Last() => Assert.Equal(4, P16BinarySearch.BinarySearch([1, 3, 5, 7, 9], 9));

    // p17
    [Fact] public void P17_Basic() => Assert.Equal(new[] { 1, 2, 3, 4, 5 }, P17BubbleSort.BubbleSort([5, 3, 1, 4, 2]));
    [Fact] public void P17_Sorted() => Assert.Equal(new[] { 1, 2, 3 }, P17BubbleSort.BubbleSort([1, 2, 3]));
    [Fact] public void P17_Reverse() => Assert.Equal(new[] { 1, 2, 3 }, P17BubbleSort.BubbleSort([3, 2, 1]));
    [Fact] public void P17_Empty() => Assert.Equal(Array.Empty<int>(), P17BubbleSort.BubbleSort([]));
    [Fact] public void P17_Duplicates() => Assert.Equal(new[] { 1, 1, 3, 3 }, P17BubbleSort.BubbleSort([3, 1, 3, 1]));

    // p18
    [Fact] public void P18_12_8() => Assert.Equal(4L, P18Gcd.Gcd(12, 8));
    [Fact] public void P18_Coprime() => Assert.Equal(1L, P18Gcd.Gcd(7, 13));
    [Fact] public void P18_100_75() => Assert.Equal(25L, P18Gcd.Gcd(100, 75));
    [Fact] public void P18_Same() => Assert.Equal(5L, P18Gcd.Gcd(5, 5));

    // p19
    [Fact] public void P19_10() => Assert.Equal(new[] { 2, 3, 5, 7 }, P19PrimeSieve.Primes(10));
    [Fact] public void P19_1() => Assert.Equal(Array.Empty<int>(), P19PrimeSieve.Primes(1));
    [Fact] public void P19_2() => Assert.Equal(new[] { 2 }, P19PrimeSieve.Primes(2));
    [Fact] public void P19_30() => Assert.Equal(new[] { 2, 3, 5, 7, 11, 13, 17, 19, 23, 29 }, P19PrimeSieve.Primes(30));

    // p20
    [Fact]
    public void P20_2x2()
    {
        long[][] a = [[1, 2], [3, 4]];
        long[][] b = [[5, 6], [7, 8]];
        long[][] expected = [[19, 22], [43, 50]];
        Assert.Equal(expected, P20Matrix.MatMul(a, b));
    }

    [Fact]
    public void P20_Identity()
    {
        long[][] a = [[1, 0], [0, 1]];
        long[][] b = [[3, 4], [5, 6]];
        Assert.Equal(b, P20Matrix.MatMul(a, b));
    }

    // p21
    [Fact] public void P21_Simple() => Assert.True(P21Bracket.IsValid("()"));
    [Fact] public void P21_Multiple() => Assert.True(P21Bracket.IsValid("()[]{}"));
    [Fact] public void P21_Nested() => Assert.True(P21Bracket.IsValid("([{}])"));
    [Fact] public void P21_Mismatch() => Assert.False(P21Bracket.IsValid("(]"));
    [Fact] public void P21_Unclosed() => Assert.False(P21Bracket.IsValid("(()"));

    // p22
    [Fact]
    public void P22_Encode()
    {
        var expected = new List<(char, int)> { ('a', 3), ('b', 2), ('c', 1) };
        Assert.Equal(expected, P22RunLength.Encode("aaabbc"));
    }

    [Fact] public void P22_EncodeEmpty() => Assert.Empty(P22RunLength.Encode(""));

    [Fact]
    public void P22_Decode()
    {
        var input = new List<(char Ch, int Count)> { ('a', 3), ('b', 2), ('c', 1) };
        Assert.Equal("aaabbc", P22RunLength.Decode(input));
    }

    [Fact] public void P22_DecodeEmpty() => Assert.Equal("", P22RunLength.Decode([]));

    // p23
    [Fact] public void P23_Yes() => Assert.True(P23Anagram.IsAnagram("listen", "silent"));
    [Fact] public void P23_Case() => Assert.True(P23Anagram.IsAnagram("Hello", "holle"));
    [Fact] public void P23_No() => Assert.False(P23Anagram.IsAnagram("abc", "abd"));
    [Fact] public void P23_Spaces() => Assert.True(P23Anagram.IsAnagram("Astronomer", "Moon starer"));

    // p24
    [Fact] public void P24_Basic() => Assert.Equal(3, P24LongestCommon.Lcs("abcde", "ace"));
    [Fact] public void P24_Same() => Assert.Equal(3, P24LongestCommon.Lcs("abc", "abc"));
    [Fact] public void P24_None() => Assert.Equal(0, P24LongestCommon.Lcs("abc", "def"));
    [Fact] public void P24_Complex() => Assert.Equal(3, P24LongestCommon.Lcs("ABCD", "ACBAD"));
    [Fact] public void P24_Empty() => Assert.Equal(0, P24LongestCommon.Lcs("", "abc"));

    // p25
    [Fact] public void P25_Basic() => Assert.Equal(90, P25Knapsack.Knapsack(10, [(5, 10), (4, 40), (6, 30), (3, 50)]));
    [Fact] public void P25_Zero() => Assert.Equal(0, P25Knapsack.Knapsack(0, [(1, 1)]));
    [Fact] public void P25_Exact() => Assert.Equal(100, P25Knapsack.Knapsack(5, [(5, 100)]));
    [Fact] public void P25_NoItems() => Assert.Equal(0, P25Knapsack.Knapsack(10, []));
}
