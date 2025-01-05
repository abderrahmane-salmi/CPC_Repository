use std::fs::File;
use std::io::{BufRead, BufReader};

// ---------------------------------- PROBLEM 1 ----------------------------------

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
            for day in 1..=attractions[city - 1].len() {
                prefix_sum[day] = prefix_sum[day - 1] + attractions[city - 1][day - 1];
            }

            // Explore all ways to spend possible days to this city
            for days_spent in 0..=days {
                // Find the maximum number of attractions possible by spending days_spent in the current city
                // here we can either skip the city (max param 1), or visit the city (max param 2) by adding
                // the attractions gained to the best result from the remaining days

                dp[days][city] =
                    dp[days][city].max(dp[days - days_spent][city - 1] + prefix_sum[days_spent]);

                // ps: attractions[city - 1] because the city index is 1-based, but the attractions vector is 0-based
                // ps: days - days_spent because we are considering the remaining days after spending days_spent in the current city
            }
        }
    }

    // Return the result from the DP table, which is the maximum attractions visited with d days and n cities
    dp[d][n]
}

// ---------------------------------- PROBLEM 2 ----------------------------------

fn max_topics(topics: Vec<(usize, usize)>, n: usize) -> usize {
    // Initialize the DP array, lis[i] will store the length of the longest increasing subsequence (LIS)
    // that ends at the topic i. Initially, each topic can at least form a subsequence of length 1 (itself)
    let mut lis = vec![1; n];

    // Sort topics by beauty in ascending order. If two topics have the same beauty, sort them by difficulty in descending order
    let mut sorted_topics = topics.clone();
    sorted_topics.sort_by(|a, b| a.0.cmp(&b.0).then_with(|| b.1.cmp(&a.1)));

    // Apply the Longest Increasing Subsequence (LIS) algorithm based on difficulty
    for i in 1..n {
        // Compare each topic i with all previous topics j
        for j in 0..i {
            // The difficulty must be greater than the previous topic's difficulty
            // to maintain an increasing subsequence
            if sorted_topics[i].1 > sorted_topics[j].1 {
                // Update the LIS value for topic i by taking the maximum between the current value and the j value + 1
                lis[i] = lis[i].max(lis[j] + 1);
            }
        }
    }

    // Return the maximum length found in LIS, which is the maximum number of topics that can be selected
    lis.iter().max().unwrap().clone()
}

// ---------------------------------- TESTING ----------------------------------

// read and parse data files, run the algorithm on input data, and compare the results with expected output
fn run_tests_p1(directory: &str, nb_of_files: usize) {
    println!("Running tests for Problem 1...");
    for i in 0..=nb_of_files {
        let input_file_path = format!("{}/input{}.txt", directory, i);
        let output_file_path = format!("{}/output{}.txt", directory, i);

        // Read the input file
        let input_file = BufReader::new(File::open(input_file_path).unwrap());
        let mut input_file_lines = input_file.lines().map(|line| line.unwrap());

        // Parse the first line for n and d
        let first_line = input_file_lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let (n, d) = (first_line[0], first_line[1]);

        // Parse the itineraries for each city without calculating cumulative sums
        let mut attractions = vec![];
        for _ in 0..n {
            let daily_values = input_file_lines
                .next()
                .unwrap()
                .split_whitespace()
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

fn run_tests_p2(directory: &str, nb_of_files: usize) {
    println!("Running tests for Problem 2...");
    for i in 0..=nb_of_files {
        let input_file_path = format!("{}/input{}.txt", directory, i);
        let output_file_path = format!("{}/output{}.txt", directory, i);

        // Read the input file
        let input_file = BufReader::new(File::open(input_file_path).unwrap());
        let mut input_file_lines = input_file.lines().map(|line| line.unwrap());

        // Parse the first line for n
        let first_line = input_file_lines
            .next()
            .unwrap()
            .split_whitespace()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let n = first_line[0];
        // let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

        // Parse the itineraries for each city without calculating cumulative sums
        let mut topics = Vec::with_capacity(n);
        for _ in 0..n {
            let topic_values: Vec<usize> = input_file_lines
                .next()
                .unwrap()
                .split_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect();
            topics.push((topic_values[0], topic_values[1])); // (beauty, difficulty)
        }

        // Run the algorithm
        let result = max_topics(topics, n);

        // Read the expected output from the output file
        let mut file_iter_output = BufReader::new(File::open(output_file_path).unwrap())
            .lines()
            .map(|x| x.unwrap());
        let binding = file_iter_output.next().unwrap();
        let mut iter = binding.split_whitespace();
        let expected_result = iter.next().unwrap().parse::<usize>().unwrap();

        // Assert that the algortihm result matches the expected result
        assert_eq!(result, expected_result, "Test {} failed", i);
        println!("Test {}: Success", i);
    }

    println!("All tests passed successfully!");
}

// ---------------------------------- MAIN ----------------------------------

fn main() {
    run_tests_p1("./data/problem1", 4);
    println!();
    run_tests_p2("./data/problem2", 10);
}
