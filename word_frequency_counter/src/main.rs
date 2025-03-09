use std::{io , collections::HashMap};
use word_frequency_counter::file_reader;
fn main(){
  println!("Please enter the path to the file you want to read");
  let mut path = String::new();
  io::stdin().read_line(&mut path).expect("Failed to read line");
  let ans : HashMap<String , u32> = file_reader::count_words(path);
  println!("{:?}", ans);
}