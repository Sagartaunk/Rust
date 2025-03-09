pub mod file_reader{
    use std::{io , collections::HashMap ,fs::File , io::BufRead};
    pub fn count_words(path: String) -> HashMap<String , u32>{
      let mut words : HashMap<String , u32> = HashMap::new();
      let file1 = path.clone();
      let file = match File::open(path){
        Ok(file) => file,
        Err(error) => match error.kind(){
          io::ErrorKind::NotFound => {
            let file2 = file1.clone();
            File::create(file1).unwrap();
            println!("File not found, creating file");
            File::open(file2).unwrap()},
          _ => panic!("Unable to create file : {}", error),
          
        }
      };
      let reader = io::BufReader::new(file);
      for li in reader.lines(){
        if let Ok(line) = li{        
          for wor in line.split_whitespace(){
            let word = wor.to_lowercase();
            *words.entry(word).or_insert(0) += 1;                
          }
        }
        
      };
      words
          
    }
  }