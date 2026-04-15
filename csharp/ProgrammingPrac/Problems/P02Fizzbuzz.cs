namespace ProgrammingPrac.Problems;

// 問題2: FizzBuzz (難易度: ★☆☆☆☆)
// public static class P02Fizzbuzz
// {
//     public static string Fizzbuzz(int n)
//     {
//         if (n%15==0) {
//             return "FizzBuzz";
//         }
//         if (n%3==0) {
//             return "Fizz";
//         }
//         if (n%5==0) {
//             return "Buzz";
//         }
//         return n.ToString();
//     }
// }

public static class P02Fizzbuzz {
    public static string Fizzbuzz(int n) {
        return (n%3,n%5) switch {
            (0,0)=>"FizzBuzz",
            (0,_)=>"Fizz",
            (_,0)=>"Buzz",
            _=>n.ToString(),
        };
    }
}