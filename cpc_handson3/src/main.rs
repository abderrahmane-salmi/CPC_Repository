use std::fs::File;
use std::io::{BufRead, BufReader};

fn holiday_planning(n: usize, d: usize, attractions: Vec<Vec<usize>>) -> usize {
    
    // initialize the DP matrix with dimensions (D+1) x (n+1) and zero values
    // D total days (rows) -- n cities considered (columns).
    let mut dp = vec![vec![0; n + 1]; d + 1];

    // Loop through all days from 1 to D
    for days in 1..=d {
        // Loop through all cities from 1 to n
        for city in 1..=n {
            // Default case: assume that no days are spent in the current city, i.e. skip the current city
            // PS: skipping a city means the best result for dp[d][c] is the same as dp[d][c-1] (the best result
            // from the previous city).
            dp[days][city] = dp[days][city - 1];

            // Compute the prefix sum of attractions for this city
            // The goal is to efficiently calculate the total attractions for any subarray of days
            let mut prefix_sum = vec![0; d + 1];
            for day in 1..=d.min(attractions[city - 1].len()) {
                prefix_sum[day] = prefix_sum[day - 1] + attractions[city - 1][day - 1];
            }
            
            // Explore all ways to spend possible days to this city
            for days_spent in 0..=days {
                // Find the maximum number of attractions possible by spending days_spent in the current city
                // here we can either skip the city (max param 1), or visit the city (max param 2) by adding
                // the attractions gained to the best result from the remaining days

                dp[days][city] = dp[days][city].max(
                    dp[days - days_spent][city - 1] + prefix_sum[days_spent]
                );

                // ps: attractions[city - 1] because the city index is 1-based, but the attractions vector is 0-based
                // ps: days - days_spent because we are considering the remaining days after spending days_spent in the current city
            }
        }
    }

    // Return the result from the DP table, which is the maximum attractions visited with d days and n cities
    dp[d][n]
}

// read and parse data files, run the algorithm on input data, and compare the results with expected output
fn run_tests_p1(directory: &str, nb_of_files: usize) {
    for i in 0..=nb_of_files {
        let input_file_path = format!("{}/input{}.txt", directory, i);
        let output_file_path = format!("{}/output{}.txt", directory, i);

        // Read the input file
        let input_file = BufReader::new(File::open(input_file_path).unwrap());
        let mut input_file_lines = input_file.lines().map(|line| line.unwrap());

        // Parse the first line for n and d
        let first_line = input_file_lines.next().unwrap().split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let (n, d) = (first_line[0], first_line[1]);

        // Parse the itineraries for each city without calculating cumulative sums
        let mut attractions = vec![];
        for _ in 0..n {
            let daily_values = input_file_lines.next().unwrap().split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect::<Vec<_>>();
            attractions.push(daily_values);
        }

        // Run the holiday planning algorithm
        let result = holiday_planning(n, d, attractions);

        // Read the expected output from the output file
        let mut file_iter_output = BufReader::new(File::open(output_file_path).unwrap())
        .lines()
        .map(|x| x.unwrap());
        let binding = file_iter_output.next().unwrap();
        let mut iter = binding.split_whitespace();
        let expected_result = iter.next().unwrap().parse::<usize>().unwrap();

        // let mut file = File::open(&output_file)?;
        // let mut expected_result = String::new();
        // file.read_to_string(&mut expected_result)?;
        // let expected_result: usize = expected_result.trim().parse().unwrap();

        // Assert that the algortihm result matches the expected result
        assert_eq!(result, expected_result, "Test {} failed", i);
        println!("Test {}: Success", i);
    }

    println!("All tests passed successfully!");
}

fn main() {
    run_tests_p1("./data/problem1", 4);
}
