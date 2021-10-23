// Test in Go Playground
// https://play.golang.org/

package main

import "fmt"

func main() {
	var numbers = []int{1,2,3}
	var permutations = permute(numbers);
	fmt.Println(permutations)
}

func permute(nums []int) [][]int {
    var res = [][]int{}
    var permutation func(int, int)
    permutation = func(start int, end int) {
        if start == end {
            temp := make([]int, len(nums))
            copy(temp, nums)
            res = append(res, temp)
            return
        }
        for i := start; i < end; i++ {
            nums[start], nums[i] = nums[i], nums[start]
            permutation(start+1, end)
            nums[start], nums[i] = nums[i], nums[start]
        }
    }
    permutation(0, len(nums))
    return res
}
