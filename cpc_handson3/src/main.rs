use std::fs::{File};
use std::io::{self, BufRead, BufReader, Write};

// This function implements the algorithm for calculating the maximum attractions
fn holiday_planning(n: usize, d: usize, attractions: Vec<Vec<usize>>) -> usize {
    // Initialize the DP table with zero values
    let mut dp = vec![vec![0; n + 1]; d + 1];

    // Loop over the number of days from 1 to D
    for days in 1..=d {
        // Loop over the number of cities from 1 to n
        for city in 1..=n {
            // Case: No days spent in the current city
            dp[days][city] = dp[days][city - 1];
            
            // Try all possible ways to distribute the days across the cities
            for spent in 0..=days {
                // Find the maximum number of attractions possible by spending `spent` days in the current city
                dp[days][city] = dp[days][city].max(
                    dp[days - spent][city - 1] + attractions[city - 1][spent]
                );
            }
        }
    }

    // Return the result from the DP table, which is the maximum attractions visited with `d` days and `n` cities
    dp[d][n]
}

// This function processes all input files from 0 to 4, runs the algorithm, and compares the results
fn process_all_tests() -> io::Result<()> {
    // Loop through input files named input0.txt to input4.txt
    for i in 0..=4 {
        let input_file = format!("./data/problem1/input{}.txt", i);
        let output_file = format!("./data/problem1/output{}.txt", i);

        // Read the input file
        let file = File::open(&input_file)?;
        let mut lines = BufReader::new(file).lines();

        // Parse the first line for `n` and `d`
        let first_line = lines.next().unwrap()?.split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let (n, d) = (first_line[0], first_line[1]);

        // Parse the itineraries for each city and calculate cumulative sums
        let mut attractions = vec![vec![0; d + 1]; n];
        for i in 0..n {
            let daily_values = lines.next().unwrap()?.split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            for j in 0..d {
                attractions[i][j + 1] = attractions[i][j] + daily_values[j]; // Cumulative sum
            }
        }

        // Run the holiday planning algorithm
        let result = holiday_planning(n, d, attractions);

        // Read the expected output from the output file
        let mut file_iter_output = BufReader::new(File::open(output_file).unwrap())
        .lines()
        .map(|x| x.unwrap());
        let binding = file_iter_output.next().unwrap();
        let mut iter = binding.split_whitespace();
        let expected_result = iter.next().unwrap().parse::<usize>().unwrap();

        // let mut file = File::open(&output_file)?;
        // let mut expected_result = String::new();
        // file.read_to_string(&mut expected_result)?;
        // let expected_result: usize = expected_result.trim().parse().unwrap();

        // Assert that the result matches the expected result
        if result == expected_result {
            println!("Test {}: Success", i);  // Print success if the result matches
        } else {
            eprintln!("Test {}: Error. Expected {} but got {}", i, expected_result, result);  // Print error if the result does not match
            return Err(io::Error::new(io::ErrorKind::Other, "Test failed"));
        }
    }

    println!("All tests passed successfully!");  // Print a success message if all tests passed
    Ok(())
}

fn main() -> io::Result<()> {
    // Call the function to process all input files from 0 to 4 and compare the results
    process_all_tests()
}
