pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
    // calculate prefix vec
    let mut prefixes: Vec<i32> = Vec::new();
    let mut prefix = 1;
    for n in nums.iter() {
        prefix *= n;
        prefixes.push(prefix);
    }

    // calculate postfix vec by reversing original nums
    let mut postfixes: Vec<i32> = Vec::new();
    let mut postfix = 1;
    for n in nums.iter().rev() {
        postfix *= n;
        postfixes.push(postfix);
    }

    // for nums[i] we want to multiple prefixes[i-1] * postfixes[i+1]
    // to make this simpler and easier to follow reverse the postfixes
    postfixes.reverse();

    // multiple prefixes[i-1] * postfixes[i+1]
    // handle cases for first and last nums[i]
    let mut ans: Vec<i32> = Vec::new();
    for i in 0..nums.len() {
        if i == 0 {
            ans.push(postfixes[i + 1]);
        } else if i == nums.len() - 1 {
            ans.push(prefixes[i - 1]);
        } else {
            ans.push(prefixes[i - 1] * postfixes[i + 1]);
        }
    }
    return ans;
}

// pub fn product_except_self(nums: Vec<i32>) -> Vec<i32> {
//   let mut prefixes: Vec<i32> = Vec::from([1]);

//   let mut prefix = 1;
//   for n in nums.iter() {
//       prefix *= n;
//       prefixes.push(prefix)
//   }

//   let mut postfixes: Vec<i32> = Vec::from([1]);

//   let mut postfix = 1;
//   for n in nums.iter().rev() {
//       postfix *= n;
//       postfixes.push(postfix)
//   }
//   postfixes.reverse();

//   let mut ans: Vec<i32> = Vec::new();
//   for i in 0..nums.len() {
//       ans.push(prefixes[i] * postfixes[i + 1]);
//   }

//   println!("{:?}", prefixes);
//   println!("{:?}", postfixes);
//   println!("{:?}", ans);
//   return ans;
// }
