const INPUT: &str = include_str!("../inputs/input");

fn main() {
   println!("part 1: {}", part_1());
   println!("part 2: {}", part_2());
}

struct Cpu {
   ic: usize,
   x: i32,
   str_sum: i32,
}

impl Cpu {
   fn new() -> Cpu {
      Cpu {
         ic: 1,
         x: 1,
         str_sum: 0,
      }
   }
   fn tick(&mut self) {
      if self.ic % 40 == 20 {
         let ic_:i32 = self.ic.try_into().unwrap();
         self.str_sum += self.x * ic_;
      }
      self.ic += 1;
   }
}

fn part_1() -> i32 {
   //let mut res = 0;
   let mut cpu = Cpu::new();
   
   for line in INPUT.split('\n') {
      if line == "noop" {
         cpu.tick();
      }
      else {
         cpu.tick();
         cpu.tick();
         cpu.x += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();     
      }
      if cpu.ic > 220 {
         break;
      }
   }
   
   cpu.str_sum
}

fn crt_draw(cpu: &Cpu) -> bool {
   if cpu.x < 0 {
      return ((cpu.ic - 1) % 40) == 0;
   }
   let ic:i32 = ((cpu.ic - 1) % 40).try_into().unwrap();
   vec![cpu.x-1,cpu.x, cpu.x+1].contains(&ic)
}

fn part_2() -> String {
   let mut cpu = Cpu::new();
   let mut crt = Vec::with_capacity(240);
   
   for line in INPUT.split('\n') {
      if line == "noop" {
         crt.push(crt_draw(&cpu));
         cpu.tick();
      }
      else {
         crt.push(crt_draw(&cpu));
         cpu.tick();
         crt.push(crt_draw(&cpu));
         cpu.tick();
         cpu.x += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();    
      }
   }
   let mut res:Vec<char> = Vec::new();
   for (i, o) in crt.into_iter().enumerate() {
      if i % 40 == 0 {
         res.push('\n');
      }
      match o {
         true => res.push('#'),
         false => res.push(' ')
      }
   }
   res.into_iter().collect()
}

#[cfg(test)]
mod test {
   use crate::*;
   
   #[test]
   fn test_part_1() {
      let mut cpu = Cpu::new();
      
      for line in include_str!("../inputs/demo").split('\n') {
         println!("start of cycle {}, {line}", cpu.ic);
         if line == "noop" {
            cpu.tick();
         }
         else {
            cpu.tick();
            println!("start of cycle {}, {line}", cpu.ic);
            cpu.tick();
            cpu.x += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();   
            println!("end of cycle {}, X now has value: {}", cpu.ic - 1, cpu.x);  
         }
         if cpu.ic == 220 {
            break;
         }
      }
      assert_eq!(cpu.str_sum, 13140);
   }
   
   #[test]
   fn test_part_2() {
      let mut cpu = Cpu::new();
      let mut crt = Vec::with_capacity(240);
      
      for line in include_str!("../inputs/demo").split('\n') {
         println!("start of cycle {}: {}", cpu.ic, line);
         if line == "noop" {
            println!("CRT draws pixel in position {}",crt.len());
            crt.push(crt_draw(&cpu));
            cpu.tick();
         }
         else {
            println!("CRT draws pixel in position {}",crt.len());
            crt.push(crt_draw(&cpu));
            cpu.tick();
            println!("start of cycle {}: {}", cpu.ic, line);
            println!("CRT draws pixel in position {}",crt.len());
            crt.push(crt_draw(&cpu));
            cpu.tick();
            cpu.x += line.strip_prefix("addx ").unwrap().parse::<i32>().unwrap();    
            println!("End of cycle {}: finish executing {} (Register X is now {})",cpu.ic, line, cpu.x);
         }
      }
      let mut res:Vec<char> = Vec::new();
      for (i, o) in crt.into_iter().enumerate() {
         if i % 40 == 0 && i != 0 {
            res.push('\n');
         }
         match o {
            true => res.push('#'),
            false => res.push('.')
         }
      }
      
      let res_str: String = res.into_iter().collect();
      
      println!("{res_str}");
      assert_eq!(res_str,String::from(include_str!("../inputs/correct_image")));
   }
   
}