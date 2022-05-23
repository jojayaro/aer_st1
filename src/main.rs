use std::env;
use std::fs;
use std::fs::File;
use std::io::{BufReader, BufRead};

fn main() {
    //Collect arguments when running the program
    let args: Vec<String> = env::args().collect();

    let query = &args[1];
    let filename = &args[2];

    println!("Searching for {}", query);
    println!("In file {}", filename);

    //Open File
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);

    //Search for indices where query is found
    let v1: Vec<_> = contents.match_indices(query).collect();

    println!("{:?}", v1);

    //Insert file lines contents into a vector
    let file = File::open(filename).expect("File not found");
    let content = BufReader::new(file);

    let lines: Vec<String> = content
        .lines()
        .map(|line| line.expect("Something went wrong"))
        .collect();

    //Iterate over lines to index the position where the query is found
    let lines_iter = lines.iter().enumerate();

    let mut lines_index: Vec<usize> = Vec::new();
    
    for (pos, e) in lines_iter {
        if e.contains(query) {
            lines_index.push(pos);
            println!("Element at position {}: {:?}", pos, e);

        }
    };

    //Slice Lines vector to include only licences
    let mut licences_vec: Vec<&str> = Vec::new();
    
    for i in lines_index[1]..lines_index[2]-2 {
        licences_vec.push(&lines[i].trim());
    }

    //Find empty lines in the licences vector
    let mut lines_index_empty: Vec<usize> = Vec::new();
    
    for (pos, e) in licences_vec.iter().enumerate() {
        if e.len() == 0 {
            lines_index_empty.push(pos);
            println!("Element at position {}: {:?}", pos, e);

        }
        //lines_index.push(pos as u32);
    };
    
    //Remove empty lines from the licences vector
    let licences_vec_clean = licences_vec.into_iter().filter(|&i| i.len() != 0).collect::<Vec<_>>();

    //Iterate over licences_vec_clean vector and split every element by whitespace and push results into a new vector
    let mut licence_vec_clean_split: Vec<&str> = Vec::new();

    for i in licences_vec_clean {
        let mut split_vec: Vec<&str> = i.split("  ").collect();
        licence_vec_clean_split.append(&mut split_vec);
    }
    
    let licence_vec_clean_split = licence_vec_clean_split.into_iter().filter(|&i| i.len() != 0).collect::<Vec<_>>();


    //Print to check
    println!("{:?}", licence_vec_clean_split);

    println!("{:?}", lines_index);

    println!("{:?}", lines_index_empty);
   
    //println!("{:?}", lines);
}