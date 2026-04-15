namespace ProgrammingPrac.Problems;

// 問題16: 二分探索 (難易度: ★★☆☆☆)
// 見つからなければ -1 を返す。
public static class P16BinarySearch
{
    public static int BinarySearch(int[] arr, int target)
    {
        int left = 0;
        int right = arr.Length;
        while (left<right) {
            int mid = left + (right - left) /2;
            if (arr[mid]==target) {
                return mid;
            }
            if (arr[mid]<target) {
                left = mid+1;
            }
            else {
                right = mid;
            }
        }
        return -1;
    }
}
