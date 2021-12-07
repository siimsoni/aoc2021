use std::collections::HashMap;
use std::cmp::max;

pub fn solve(input: &Vec<usize>) -> usize {
   let mut weights = HashMap::new();
   let mut max_val = 0;
   for val in input {
      *weights.entry(*val).or_insert(0) += 1;
      max_val = max(*val, max_val)
   }

   let mut min_cost = usize::MAX;

   for val in 0..max_val {
      let mut cost = 0;
      for (other_val, weight) in &weights {
         if *other_val > val {
            cost += get_cost(other_val - val) * weight;
         } else if val > *other_val {
            cost += get_cost(val - other_val) * weight;
         }
      }
      if cost < min_cost {
         min_cost = cost;
      }
   }

   min_cost
}

fn get_cost(dist: usize) -> usize {
   dist * (dist + 1) / 2
}
