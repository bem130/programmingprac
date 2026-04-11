import { greet } from "../src/problems/p01_hello";
import { fizzbuzz } from "../src/problems/p02_fizzbuzz";
import { sum } from "../src/problems/p03_sum";
import { reverse } from "../src/problems/p04_reverse";
import { isPalindrome } from "../src/problems/p05_palindrome";
import { fibonacci } from "../src/problems/p06_fibonacci";
import { countChars } from "../src/problems/p07_countChars";
import { unique } from "../src/problems/p08_unique";
import { median } from "../src/problems/p09_median";
import { evalRpn } from "../src/problems/p10_rpn";

describe("p01: greet", () => {
  test("World", () => expect(greet("World")).toBe("Hello, World!"));
  test("Rust", () => expect(greet("Rust")).toBe("Hello, Rust!"));
  test("empty", () => expect(greet("")).toBe("Hello, !"));
});

describe("p02: fizzbuzz", () => {
  test("3 → Fizz", () => expect(fizzbuzz(3)).toBe("Fizz"));
  test("5 → Buzz", () => expect(fizzbuzz(5)).toBe("Buzz"));
  test("15 → FizzBuzz", () => expect(fizzbuzz(15)).toBe("FizzBuzz"));
  test("7 → 7", () => expect(fizzbuzz(7)).toBe("7"));
});

describe("p03: sum", () => {
  test("basic", () => expect(sum([1, 2, 3])).toBe(6));
  test("empty", () => expect(sum([])).toBe(0));
  test("negative", () => expect(sum([-1, 1, -2, 2])).toBe(0));
});

describe("p04: reverse", () => {
  test("hello", () => expect(reverse("hello")).toBe("olleh"));
  test("empty", () => expect(reverse("")).toBe(""));
  test("single", () => expect(reverse("a")).toBe("a"));
});

describe("p05: palindrome", () => {
  test("racecar", () => expect(isPalindrome("racecar")).toBe(true));
  test("Madam", () => expect(isPalindrome("Madam")).toBe(true));
  test("hello", () => expect(isPalindrome("hello")).toBe(false));
  test("empty", () => expect(isPalindrome("")).toBe(true));
});

describe("p06: fibonacci", () => {
  test("0", () => expect(fibonacci(0)).toBe(0));
  test("1", () => expect(fibonacci(1)).toBe(1));
  test("10", () => expect(fibonacci(10)).toBe(55));
  test("20", () => expect(fibonacci(20)).toBe(6765));
});

describe("p07: countChars", () => {
  test("Hello", () => {
    const r = countChars("Hello");
    expect(r.get("h")).toBe(1);
    expect(r.get("l")).toBe(2);
    expect(r.get("o")).toBe(1);
  });
  test("spaces ignored", () => {
    const r = countChars("A a");
    expect(r.get("a")).toBe(2);
    expect(r.size).toBe(1);
  });
  test("empty", () => expect(countChars("").size).toBe(0));
});

describe("p08: unique", () => {
  test("basic", () => expect(unique([1, 2, 3, 2, 1])).toEqual([1, 2, 3]));
  test("all same", () => expect(unique([5, 5, 5])).toEqual([5]));
  test("empty", () => expect(unique([])).toEqual([]));
});

describe("p09: median", () => {
  test("odd", () => expect(median([3, 1, 2])).toBe(2));
  test("even", () => expect(median([4, 1, 3, 2])).toBe(2.5));
  test("empty", () => expect(median([])).toBe(0));
  test("single", () => expect(median([42])).toBe(42));
});

describe("p10: rpn", () => {
  test("add", () => expect(evalRpn("3 4 +")).toBe(7));
  test("complex", () => expect(evalRpn("5 1 2 + 4 * + 3 -")).toBe(14));
  test("divide", () => expect(evalRpn("4 2 /")).toBe(2));
  test("empty", () => expect(evalRpn("")).toBeNull());
  test("invalid", () => expect(evalRpn("1 +")).toBeNull());
});
