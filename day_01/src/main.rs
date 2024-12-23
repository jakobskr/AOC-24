use std::{fs};
use std::iter::zip;
use std::collections::HashMap;


fn main() {
    println!("Hello, world!");

    let rel_path = "/inputs/test_data";

    let mut input : Vec<String> = Vec::new();



    for line in fs::read_to_string("/Users/jakob/Documents/kode/AOC-24/day_01/inputs/data").unwrap().lines() {
        // println!("{}" ,line);

        input.push(line.to_string());
    }

    println!("part_1: {}", part1(&input));
    println!("part_2: {}", part2(&input));

}


fn part1(problem: &Vec<String>) -> i32{


    let mut left_list : Vec<i32> = Vec::new();
    let mut right_list : Vec<i32> = Vec::new();


    for line in problem {

        let tmp : Vec<&str> = line.split_whitespace().collect();
        left_list.push(tmp[0].parse().unwrap());
        right_list.push(tmp[1].parse().unwrap());
    }

    left_list.sort();
    right_list.sort();

    let zipped = zip(left_list, right_list);



    // println!("{:?}", zipped);

    return zipped.fold(0, |acc, x| i32::abs(x.0 - x.1) + acc);
}

fn part2(problem : &Vec<String>) -> u32 {


    let mut left_list : Vec<u32> = Vec::new();
    let mut right_list : Vec<u32> = Vec::new();

    for line in problem {

        let tmp : Vec<&str> = line.split_whitespace().collect();

        // println!("{:?}", tmp);

        left_list.push(tmp[0].parse().unwrap());
        right_list.push(tmp[1].parse().unwrap());
    }

    let mut right_map : HashMap<u32, u32> = HashMap::new();

    for item in right_list {
        *right_map.entry(item).or_insert(0) += 1;
    }

    // println!("{:?}", right_map);
    // println!("{:?}", left_list);


    return left_list.iter().fold(0, |acc, x| *right_map.entry(*x).or_insert(0) * x + acc);
}