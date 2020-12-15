use std::fs;
use itertools::Itertools;


pub fn execute_dayten() {
  let path = "./input/day10.txt";
  let working: Vec<i32> = prepare_input(path);
  let (one, three) = gap_count(working.clone());
  println!("Should be {} * {} = {}", one, three, one * three );

  let perms = get_permutations(working);
  println!("There's {} permutations", perms);
}

fn get_permutations(full_list: Vec<i32>) -> i64{
  let mut perms = 0;
  let mut distances: Vec<i32> = Vec::new();



  perms
}

/*
  Each number can be itself or up to -3 
  Find the item on the list with the highest number and +3
  Start at 0
  At each level, check for all numbers within +3 (filter / drain)
    for each option

  So, in terms of permutations, you need to know the total number of permutations
  And then you need to know the minimum allowed sequence of adaptors
  So, maybe you could do an array of the gaps and get the permutations of all the gaps/potential gaps
  0..x
  Need to know if the neighbors can be removed before considering each number's individual permutation
  Worst case scenario and every gap is 1, the range needs to be 3 in either direction, if exists
  0 and <create last number> cannot be removed
 */

 fn gap_count(worklist: Vec<i32>) -> (i32, i32){ //2170 too low 6300 too high
   let mut workan: Vec<i32> = worklist;
   workan.sort_unstable();
   workan.push(workan.last().unwrap()+3); //The laptop
   let mut one = 0;
   let mut three = 0;

   for (e, &x) in workan.iter().enumerate() {
    let searcher: Vec<i32>  = workan.clone();
     let i_target: usize = e+1;
     if i_target >= workan.len() {
       break;
     }
     let target: i32 = searcher[i_target];
     if target - x == 1 {
       one +=1;
     } else if target - x == 2 {
      println!("Sommin");
     }
     else if target - x == 3{
      three +=1;
     } else {
       println!("Oh man, not sure how we got here");
     }
   }

   (one as i32, three as i32)
 }

//Thanks, fasterthanlime!
fn prepare_input (filepath: &str) -> Vec<i32> {
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");
  let ret = list
                      .as_str()
                      .split('\n')
                      .map(str::parse::<i32>)
                      .map(Result::unwrap)
                      .collect();
  ret
}