// Instructions 1: Armstrong
// Write a function that checks if a number is an armstrong number
// An Armstrong number is a number that is the sum of its own digits each raised to the power of the number of digits.
pub fn is_armstrong_number(num: u32) -> bool {
    // todo!("true if {num} is an armstrong number")
    let strNum = num.to_string();
    let size = strNum.len();
    let mut sum :u32= 0;

    for digit in strNum.chars(){
            let n: u32 = digit.to_digit(10).unwrap();                    
               sum =  sum.checked_add( n.pow(size as u32)).unwrap_or_default();          
            }
    sum == num as u32
}

// Instructions 2: Grains of wheat
// Calculate the number of grains of wheat on a chessboard given that the number on each square doubles.

pub fn square(s: u32) -> u64 {
    // todo!("grains of rice on square {s}");
        if s > 64 || s == 0 {panic!("Square must be between 1 and 64")};
    let mut successive = 0;
    for i in 1..(s + 1){
        if i == 1{
       successive +=1;    
        }
        else{
        successive += successive; 
        }
    }
    successive
}
pub fn total() -> u64 {
    // todo!();
    let mut total = 0;
    for i in 1..65{       
       total +=  square(i);
    }
    total
}
//Instruction 3: Gigaseconds
//  determine the date and time of one gigasecond after a certain date.
use time::PrimitiveDateTime as DateTime;
use time::{Duration, macros::{date, time}};
// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    // todo!("What time is a gigasecond later than {start}");
  
    // Define one gigasecond in seconds
    const GIGASECOND: i64 = 1_000_000_000;
    // Add one gigasecond to the current date and time
    let future_date_time = start + Duration::seconds(GIGASECOND);
    
    future_date_time
}

//Instruction 4: proverb
// For want of a horseshoe nail, a kingdom was lost, or so the saying goes.
// Given a list of inputs, generate the relevant proverb. 
pub fn build_proverb(list: &[&str]) -> String {
    // todo!("build a proverb from this list of items: {list:?}")
    let mut out_put = String::new();
    let num = list.len();
    for i in 0..num{
        if i != num - 1{
        out_put.push_str("For want of a ");
        out_put.push_str(list[i]);
        out_put.push_str( " the " );
        out_put.push_str(list[i + 1]);
        out_put.push_str(" was lost.\n");
       
        }else{
            
                out_put.push_str("And all for the want of a ");
                out_put.push_str(list[0]);
                out_put.push_str(".");
            
            
        }
    }
    out_put
}
// Instruction 5: Difference of square
// Find the difference between the square of the sum and the sum of the squares of the first N natural numbers.
// Instruction 6:
// Reversing strings (reading them from right to left, rather than from left to right) is a surprisingly common task in programming.

use unicode_reverse::reverse_grapheme_clusters_in_place;
pub fn reverse(input: &str) -> String {
    // todo!("Write a function to reverse {input}");
        input.chars().rev().collect()
}
pub fn square_of_sum(n: u32) -> u32 {
    let mut square = 0;
    for i in 1..(n + 1){
        square += i;
    }
    square * square
}
pub fn sum_of_squares(n: u32) -> u32 {
    let mut square = 0;
    for i in 1..(n + 1){
        square += i * i;
    }
    square
}
pub fn difference(n: u32) -> u32 {
    square_of_sum(n) - sum_of_squares(n)
}

//Instruction 7:
//  write the code that calculates the energy points that get awarded to players when they complete a level.
use std::collections::HashMap;
pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    // todo!("Sum the multiples of all of {factors:?} which are less than {limit}")
    let mut v:HashMap<u32, bool> = HashMap::new();
let mut sum = 0;
  for i in factors.iter(){
   for j in 1..limit{
    let x = i * j;
    if x >= limit{
        break;
    }
    match v.get(&x).copied(){
        None =>  {v.insert(x, true);
        sum += &x}
        _ => ()
    }   
   }
  }
  sum
}
// Instruction 8: RainDrop
//  convert a number into its corresponding raindrop sounds. 
pub fn raindrops(n: u32) -> String {
    // todo!("what sound does Raindrop #{n} make?")
      let factor: [u32;3] = [3,5,7];
    let mut out_put = String::new();
    for i in factor{
        if n % i == 0{
           if i == 3{
            out_put.push_str("Pling");
           }
           else if  i == 5 {
            out_put.push_str("Plang");      
           }
           else{
            out_put.push_str("Plong");
           }
        }                              
    }
    if out_put.len() == 0{
        let stri = n.to_string();
            out_put = stri;
        
    }
    out_put
}
// Instruction 9: Leap Year.
//  determine whether a given year is a leap year.
pub fn is_leap_year(year: u64) -> bool {
    // todo!("true if {year} is a leap year")
    if year % 100 == 0{
        year % 400 == 0
    }
else{
    year % 4 == 0
}
}
// Instruction 10: Nth Pirme
// Given a number n, determine what the nth prime is.
fn is_prime(num: u32)  -> bool{
        let mut div = 0;
        for i in 1..(num + 1){
            if num % i == 0 {
                div += 1;           
            }
        }
        if div == 2{
            true 
        }
        else{
            false 
        }
    
    }
    fn nth(n: u32) -> u32 {
        let mut start = 2;
        let mut index = 0;
       loop{
        if is_prime(start) == true{
            if index == n{
               break; 
            } 
            index += 1;      
        }
        start += 1;      
       }
       start
       }

fn main() {
    //1) Armstrong
    println!("1) Armstrongs Results");
    let status = is_armstrong_number(9);
    println!("status: {}", status);
    let status2 = is_armstrong_number(10);
    println!("status2: {}", status2);
    let status3 = is_armstrong_number(4_106_098_957);
    println!("status3: {}", status3);

    //2) Grains of wheat
    println!("2) Grains of wheat Results");
    let check = square(20);
    let all = total();
    println!("check 5: {check}, all: {all}");

    //3) gigaseconds
    println!("3) One gigaseconds Results");
    let d =  DateTime::new(date!(2019 - 01 - 01), time!(0:00));
    let future = after(d);
    println!("One gigaseconds of {d} is {future}");

    //4) proverbs
    println!("4) Proverbs result");
    let res = build_proverb(&["nail", "shoe", "horse", "rider", "message", "battle", "kingdom"]);
        println!("{:?}", res);

    //5) Difference of square
    println!("5) Square difference result");
    let diff = difference(10);
    println!("{diff}");

    // 6) Reverse Strings
    println!("5) Reverse Strings result");
    let  result = reverse("input");
    println!("{result}");
    let mut x = "man\u{0303}ana".to_string();

    println!("{}", x); // prints "mañana"

    reverse_grapheme_clusters_in_place(&mut x);
    println!("{}", x); // prints "anañam"

    // 7) Sum of multiples
    println!("7) Sum of multiples");
    let x = sum_of_multiples(20, &[3,5]);
    println!("{x}");
    // 8) RainDrop
    println!("8) Rain Drop");
    let res = raindrops(28);
    println!("{:?}",res);
    let res = raindrops(34);
    println!("{:?}",res);
    let res = raindrops(30);
    println!("{:?}",res);
    //9) Leap Year
    println!("9) Raindrop Result");
    let l = is_leap_year(2000);
    let m = is_leap_year(1900);
    let n = is_leap_year(1997);
    println!("{l}, {m}, {n}");
    // 100 Nth prime:
    println!("10) Nth prime");
    let i = 10_000;
    let value = nth(i);
    println!("{i}: {value}")
    
}


