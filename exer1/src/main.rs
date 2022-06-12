fn main() {
    let org_arr = [1, 2, 3, 5, 6, 8, 10, 11];
    let sub_arr = [6, 8, 10];
  
    compare(&org_arr, &sub_arr);
  }
  
  fn compare(org_arr: &[i32], sub_arr: &[i32]) {
    if sub_arr.len() > org_arr.len() {
      println!("This isn't a sub array")
    }else {
      let temp = org_arr.len() - sub_arr.len();
      let mut is_sub = false;
      
      for (i, x) in org_arr[0..temp].iter().enumerate() {
        if x == &sub_arr[0] {
          let mut check = true;
          let ind = i + sub_arr.len();
          let start = i;

          for j in start..ind {
            if &org_arr[j] != &sub_arr[j - i] {
              check = false;
              break;
            }
          }

          if check {
            is_sub = true;
          }
        }
      }

      if is_sub {
        println!("Is subarray");
      } else {
        println!("Not a subarray");
      }
    }
  }
  