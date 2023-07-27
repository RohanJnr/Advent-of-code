use std::collections::HashSet;
use std::cmp::max;

fn with_set(nums: Vec<i32>) -> i32 {
    let set: HashSet<i32> = nums.clone().into_iter().collect();
    dbg!(&set);

    let mut longest = 0;

    for val in nums {
        let new_val = val - 1;
        if !(set.contains(&new_val)) {
            let mut length = 0;


            while set.contains(&(val + length)) {
                length += 1;
            }

            longest = max(longest, length);

        }
    }
    longest

}

fn with_sort(mut nums: Vec<i32>) -> i32 {

    if nums.is_empty() {
        return 0;
    }

    nums.sort_unstable();
    dbg!(&nums);

    let mut start = nums[0];

    let mut longest: i32 = 0;
    let mut local_longest: i32 = 1;

    let mut iterator = nums.iter();
    iterator.next();

    for num in iterator {
        if *num == start {
            continue;
        }
        else if *num == start + 1 {
            local_longest += 1;
        } else {
            longest = max(longest, local_longest);
            local_longest = 1;
        }
        start = *num;
    }
    longest = max(longest, local_longest);
    longest

}

fn main() {
    let input = vec![9,1,4,7,3,-1,0,5,8,-1,6];

    // let result = with_set(input);
    let result = with_sort(input);

    println!("{}", result);
}

