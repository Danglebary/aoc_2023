#!/bin/bash

# Get the day number from the command line arguments
day_number=$1

# If no day number is provided, print an error message and exit the script
if [ -z "$day_number" ]; then
    echo "Error: No day number provided"
    exit 1
fi

# Convert the day number into a string representation
declare -A number_to_string=( [1]="one" [2]="two" [3]="three" [4]="four" [5]="five" [6]="six" [7]="seven" [8]="eight" [9]="nine" [10]="ten" )
day_string=${number_to_string[$day_number]}

# Create the directory for the day
mkdir "./src/day_$day_string"

# Create the mod.rs file with the boilerplate code
echo 'pub mod part_one;
pub mod part_two;' > "./src/day_$day_string/mod.rs"

# Create the part_1.rs and part_2.rs files with the boilerplate code
echo 'use crate::utils::file;

#[allow(dead_code)]
pub fn run() {
    let input = file::read_file_lines("src/day_'$day_string'/input.txt").unwrap();
}' > "./src/day_$day_string/part_one.rs"

echo 'use crate::utils::file;

#[allow(dead_code)]
pub fn run() {
    let input = file::read_file_lines("src/day_'$day_string'/input.txt").unwrap();
}' > "./src/day_$day_string/part_two.rs"

# Create an empty input.txt file
touch "./src/day_$day_string/input.txt"

# Read the existing content of the src/main.rs file
existing_content=$(cat src/main.rs)

# Prepend the module declaration and import statements to the existing content
echo "mod day_$day_string;
$existing_content
" > src/main.rs