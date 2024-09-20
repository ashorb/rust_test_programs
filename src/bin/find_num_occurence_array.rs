fn find_num_occurrence_array (n : u8, arr : [u8; 10]) -> usize {
    // TODO: find and return the number of occurrences of "n" in array "arr".
    let mut count = 0;
    for &i in &arr {
      if i == n {
        count += 1;
      };
    }
    count
  }

  fn find_num_occurrence_array_ref (n : u8, arr_ref : &[u8; 10]) -> usize {
    // TODO: find and return the number of occurrences of "n" in array referenced by "arr_ref".
    let mut count = 0;
    for &i in arr_ref {
      if i == n {
        count += 1;
      };
    }
    count
  }

  fn find_num_occurrence_slice (n : u8, slice : &[u8]) -> usize {
    // TODO: find and return the number of occurrences of "n" in slice reference "slice".
    let mut count = 0;
    for &i in slice {
      if i == n {
        count += 1;
      }
    };
    count
  }
  
  fn main () {
    let array = [4,5,6,7,8,9,5,5,6,10];
    // TODO: call find_num_occurrence_array in a loop (with every from 0 to 9 inclusive).
    let res1 = find_num_occurrence_array(5, array);
    let res2 = find_num_occurrence_array_ref(5, &array);
    let res3 = find_num_occurrence_slice(5,  &array[..]);
    
    println!("1: {}", res1);
    println!("2: {}", res2);
    println!("3: {}", res3);
  }