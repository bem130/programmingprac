import { binarySearch } from "../src/problems/p16_binarySearch";
import { bubbleSort } from "../src/problems/p17_bubbleSort";
import { gcd } from "../src/problems/p18_gcd";
import { primes } from "../src/problems/p19_primeSieve";
import { matMul } from "../src/problems/p20_matrix";
import { isValid } from "../src/problems/p21_bracket";
import { encode, decode } from "../src/problems/p22_runLength";
import { isAnagram } from "../src/problems/p23_anagram";
import { lcs } from "../src/problems/p24_longestCommon";
import { knapsack } from "../src/problems/p25_knapsack";

describe("p16: binarySearch", () => {
  test("found", () => expect(binarySearch([1, 3, 5, 7, 9], 5)).toBe(2));
  test("not found", () => expect(binarySearch([1, 3, 5, 7, 9], 4)).toBe(-1));
  test("empty", () => expect(binarySearch([], 1)).toBe(-1));
  test("first", () => expect(binarySearch([1, 3, 5, 7, 9], 1)).toBe(0));
  test("last", () => expect(binarySearch([1, 3, 5, 7, 9], 9)).toBe(4));
});

describe("p17: bubbleSort", () => {
  test("basic", () => expect(bubbleSort([5, 3, 1, 4, 2])).toEqual([1, 2, 3, 4, 5]));
  test("sorted", () => expect(bubbleSort([1, 2, 3])).toEqual([1, 2, 3]));
  test("reverse", () => expect(bubbleSort([3, 2, 1])).toEqual([1, 2, 3]));
  test("empty", () => expect(bubbleSort([])).toEqual([]));
  test("duplicates", () => expect(bubbleSort([3, 1, 3, 1])).toEqual([1, 1, 3, 3]));
});

describe("p18: gcd", () => {
  test("12, 8", () => expect(gcd(12, 8)).toBe(4));
  test("coprime", () => expect(gcd(7, 13)).toBe(1));
  test("100, 75", () => expect(gcd(100, 75)).toBe(25));
  test("same", () => expect(gcd(5, 5)).toBe(5));
});

describe("p19: primes", () => {
  test("10", () => expect(primes(10)).toEqual([2, 3, 5, 7]));
  test("1", () => expect(primes(1)).toEqual([]));
  test("2", () => expect(primes(2)).toEqual([2]));
  test("30", () => expect(primes(30)).toEqual([2, 3, 5, 7, 11, 13, 17, 19, 23, 29]));
});

describe("p20: matMul", () => {
  test("2x2", () => {
    expect(matMul([[1, 2], [3, 4]], [[5, 6], [7, 8]])).toEqual([[19, 22], [43, 50]]);
  });
  test("identity", () => {
    expect(matMul([[1, 0], [0, 1]], [[3, 4], [5, 6]])).toEqual([[3, 4], [5, 6]]);
  });
  test("rect", () => {
    expect(matMul([[1, 2, 3]], [[4], [5], [6]])).toEqual([[32]]);
  });
});

describe("p21: bracket", () => {
  test("simple", () => expect(isValid("()")).toBe(true));
  test("multiple", () => expect(isValid("()[]{}")).toBe(true));
  test("nested", () => expect(isValid("([{}])")).toBe(true));
  test("mismatch", () => expect(isValid("(]")).toBe(false));
  test("unclosed", () => expect(isValid("(()")).toBe(false));
});

describe("p22: runLength", () => {
  test("encode", () => expect(encode("aaabbc")).toEqual([["a", 3], ["b", 2], ["c", 1]]));
  test("encode empty", () => expect(encode("")).toEqual([]));
  test("decode", () => expect(decode([["a", 3], ["b", 2], ["c", 1]])).toBe("aaabbc"));
  test("decode empty", () => expect(decode([])).toBe(""));
  test("roundtrip", () => expect(decode(encode("aaabbbccccdd"))).toBe("aaabbbccccdd"));
});

describe("p23: anagram", () => {
  test("yes", () => expect(isAnagram("listen", "silent")).toBe(true));
  test("case", () => expect(isAnagram("Hello", "holle")).toBe(true));
  test("no", () => expect(isAnagram("abc", "abd")).toBe(false));
  test("spaces", () => expect(isAnagram("Astronomer", "Moon starer")).toBe(true));
});

describe("p24: lcs", () => {
  test("basic", () => expect(lcs("abcde", "ace")).toBe(3));
  test("same", () => expect(lcs("abc", "abc")).toBe(3));
  test("none", () => expect(lcs("abc", "def")).toBe(0));
  test("complex", () => expect(lcs("ABCD", "ACBAD")).toBe(3));
  test("empty", () => expect(lcs("", "abc")).toBe(0));
});

describe("p25: knapsack", () => {
  test("basic", () => expect(knapsack(10, [[5, 10], [4, 40], [6, 30], [3, 50]])).toBe(90));
  test("zero", () => expect(knapsack(0, [[1, 1]])).toBe(0));
  test("exact", () => expect(knapsack(5, [[5, 100]])).toBe(100));
  test("no items", () => expect(knapsack(10, [])).toBe(0));
});
