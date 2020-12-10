use std::fs;

#[derive(Clone)]
struct XmasStream {
  value: i64,
  origi: usize,
  validated: i32,
}

impl XmasStream {
  pub fn new(value: i64, origi: usize) -> Self {
    Self {
      value,
      origi,
      validated: 0,
    }
  }
}

pub fn execute_day_nine() { // 403 is too low 36399731384757 is too high
  let path = "./input/day9.txt";
  let preamble: usize = 25;
  let working = prepare_input(path);
  let weak_point = find_weakness(working, preamble);
  println!("Use {} to attack for massive damage!", weak_point);
}

fn find_weakness(working: Vec<XmasStream>, preamble: usize) -> i64{
  let mut weak_point = 0;
  let mut preamble_start: usize = 0;
  let mut preamble_end: usize = preamble;
  let mut w_copy = working;
  let mut p_work: Vec<XmasStream> = Vec::new();
  let mut target: i64 = 0;

  while weak_point == 0 {
    p_work = w_copy[preamble_start..preamble_end].to_vec().clone();
    let target_index = preamble_end;
    target = w_copy[target_index].value;

    for i in 0..p_work.len() {
      for j in 0..p_work.len() {
        if i != j && p_work[i].value + p_work[j].value == target {
            w_copy[preamble_end+1].validated +=1;
        } 
      }
    }
    if w_copy[preamble_end+1].validated == 0 {
      weak_point = target;
    }
    /*
     * Hoooh boy, ok
     * You need the preamble and only the VERY NEXT number
     * for all combinations of preamble numbers
     *  count the total times summed (if you want but you don't need to)
     *  return the first preamble+1 number that has 0 sums   
    */

    preamble_start += 1;
    preamble_end += 1;
  }

  return weak_point;
}

fn prepare_input (filepath: &str) -> Vec<XmasStream> {
  let mut ret: Vec<XmasStream> = Vec::new();
  let list = fs::read_to_string(filepath).expect("Yeah, that's not a file");

  for lin in list.lines() {
    ret.push(XmasStream::new(lin.parse::<i64>().unwrap(), ret.len()));
  }
  return ret;
}