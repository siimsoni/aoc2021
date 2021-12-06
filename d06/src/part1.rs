use std::collections::HashMap;

type Glowfish = (u8, u8);

pub fn solve(input: &Vec<u8>) -> usize {
   let mut result = 0;
   let mut cache = HashMap::new();
   for fish in input {
      result += count((*fish, 80), &mut cache);
   }
   result
}

fn count(data: Glowfish, cache: &mut HashMap<Glowfish,usize>) -> usize {
   if let Some(val) = cache.get(&data) {
      return *val;
   }
   let mut result = 1;
   if let Some(children) = populate(data) {
      for child in children {
         result += count(child, cache);
      }
   }
   cache.insert(data, result);
   result
}

fn populate(data: Glowfish) -> Option<Vec<Glowfish>> {
   let (days_to_spawn, days_to_end) = data;
   if days_to_end < days_to_spawn + 1 {
      return None;
   }
   let mut result = Vec::new();
   let mut days_to_end = days_to_end - days_to_spawn - 1;
   result.push((8, days_to_end));
   while days_to_end >= 7 {
      days_to_end -= 7;
      result.push((8, days_to_end));
   }
   Some(result)
}
