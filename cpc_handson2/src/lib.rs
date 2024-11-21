use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub struct SegmentTree {
    n: usize,
    tree: Vec<i32>,
    lazy: Vec<Option<i32>>,
}

pub enum NodeFunction {
    Min,
    Max,
}

impl SegmentTree {

    // --------------------- CONSTRUCTOR ---------------------
    
    // init the segment tree object from the given array array
    pub fn init(a: &[i32], node_function: &NodeFunction) -> Self {
        let length = a.len();

        let mut segment_tree = SegmentTree {
            n: length,
            tree: vec![0; 4 * length],
            lazy: vec![None; 4 * length],
        };
        
        segment_tree.build(a, 0, length - 1, 0, node_function);
        segment_tree
    }

    // fill the segment tree with the array data
    fn build(
        &mut self,
        arr: &[i32],
        start_pos: usize,
        end_pos: usize,
        curr_node_pos: usize,
        node_function: &NodeFunction
    ) {
        // base case: if the start and end positions are the same, then we are at a leaf node
        if start_pos == end_pos {
            self.tree[curr_node_pos] = arr[end_pos];
            return;
        }

        // split the range into two halves and populate the left and right subtrees
        let mid = (start_pos + end_pos) / 2;

        self.build(arr, start_pos, mid, left_child(curr_node_pos), node_function);
        self.build(arr, mid + 1, end_pos, right_child(curr_node_pos), node_function);

        // set the value of the current node to the max or min of the left and right children, depending on the given problem
        self.tree[curr_node_pos] = match node_function {
            NodeFunction::Min => self.tree[left_child(curr_node_pos)].min(self.tree[right_child(curr_node_pos)]),
            NodeFunction::Max => self.tree[left_child(curr_node_pos)].max(self.tree[right_child(curr_node_pos)]),
        };
    }

    // --------------------- MAX QUERY ---------------------

    // max query in range [l, r]
    pub fn max_query(&mut self, l: usize, r: usize) -> Option<i32> {
        self.max_query_rec(0, 0, self.n - 1, l-1, r-1)
    }

    fn max_query_rec(
        &mut self, 
        curr_node_pos: usize, 
        start: usize, 
        end: usize, 
        l: usize, 
        r: usize
    ) -> Option<i32> {
        // make sure our tree is up-to-date by applying any lazy updates at the start
        self.apply_lazy_update(curr_node_pos, start, end);

        if start > r || end < l {
            // no overlap, skip node
            return None;
        }

        if start >= l && end <= r {
            // total overlap, return the value of the current node
            return Some(self.tree[curr_node_pos]);
        }

        // partial overlap, recurse on the children
        let mid = (start + end) / 2;
        
        let left_max = self.max_query_rec(left_child(curr_node_pos), start, mid, l, r);
        let right_max = self.max_query_rec(right_child(curr_node_pos), mid + 1, end, l, r);
        
        match (left_max, right_max) {
            // return the max of the two values
            (Some(left_max), Some(right_max)) => Some(left_max.max(right_max)),
            
            // return the value that is not None
            (Some(left_max), None) => Some(left_max),
            (None, Some(right_max)) => Some(right_max),
            
            // return None if both values are None
            (None, None) => None, // TODO check if this is correct
        }
    }

    // apply the lazy update to a node
    fn apply_lazy_update(&mut self, node_pos: usize, start: usize, end: usize) {
        if let Some(lazy_update_value) = self.lazy[node_pos] {
            // update the current node
            self.tree[node_pos] = self.tree[node_pos].min(lazy_update_value);
            
            // propagate the lazy value to the node's children if its not a leaf node
            if start != end {
                self.lazy_min_or_set(left_child(node_pos), lazy_update_value);
                self.lazy_min_or_set(right_child(node_pos), lazy_update_value);
            }

            // clear the lazy value for this node
            self.lazy[node_pos] = None;
        }

        // otherwise, there is no lazy update to do
    }

    // set the lazy value for a node by taking the min between the current lazy value and the new lazy value
    fn lazy_min_or_set(&mut self, node_pos: usize, new_lazy_value: i32) {
        if let Some(old_lazy_value) = self.lazy[node_pos] {
            // the lazy tree has a already value for this node, choose the min between this and the new lazy value
            self.lazy[node_pos] = Some(old_lazy_value.min(new_lazy_value));
        } else {
            // the lazy tree has no value for this node, set the new lazy value
            self.lazy[node_pos] = Some(new_lazy_value);
        }
    }

    // ---------------------- RANGE UPDATE ----------------------

    pub fn update_range(&mut self, l: usize, r: usize, t: i32) {
        self.update_range_rec(0, 0, self.n - 1, l-1, r-1, t);
    }

    fn update_range_rec(
        &mut self,
        curr_node_pos: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
        t: i32
    ) {
        
        // make sure our tree is up-to-date by applying any lazy updates at the start
        self.apply_lazy_update(curr_node_pos, start, end);

        if start > r || end < l {
            // no overlap, skip
            return;
        }

        if start >= l && end <= r {
            // total overlap
            // update the current node
            self.tree[curr_node_pos] = self.tree[curr_node_pos].min(t);

            // if not a leaf, propagate a lazy update to children
            if start != end {
                self.lazy_min_or_set(left_child(curr_node_pos), t);
                self.lazy_min_or_set(right_child(curr_node_pos), t);
            }

        } else {
            // partial overlap, recurse on the children
            let mid = (start + end) / 2;
            
            self.update_range_rec(left_child(curr_node_pos), start, mid, l, r, t);
            self.update_range_rec(right_child(curr_node_pos), mid + 1, end, l, r, t);
            
            // update the current node to the max of the children
            self.tree[curr_node_pos] = self.tree[left_child(curr_node_pos)].max(self.tree[right_child(curr_node_pos)]);
        }
    }

    // ---------------------- PROBLEM 2 ----------------------
    // ---------------------- IS THERE ----------------------

    // Check if there exists a position within range [l, r] with exactly 'k' segments covering it
    pub fn exists_exact_coverage(&mut self, l: usize, r: usize, k: i32) -> bool {
        self.range_exact_check(0, 0, self.n - 1, l, r, k)
    }

    fn range_exact_check(
        &mut self,
        curr_node_pos: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
        k: i32
    ) -> bool {
        // apply any pending lazy updates
        self.apply_lazy_update(curr_node_pos, start, end);

        // no overlap
        if start > r || end < l {
            return false;
        }

        // total overlap
        if start >= l && end <= r {
            // return self.tree[curr_node_pos] == k;
            return self.find_first_match(curr_node_pos, k).is_some();
        }

        // Partial overlap, recurse on the children
        let mid = (start + end) / 2;
        let left_exists = self.range_exact_check(left_child(curr_node_pos), start, mid, l, r, k);
        let right_exists = self.range_exact_check(right_child(curr_node_pos), mid + 1, end, l, r, k);

        // return true if either the left or right child has a match
        left_exists || right_exists
    }

    // helper function to find the first match of a value in the st
    fn find_first_match(&mut self, node: usize, value: i32) -> Option<i32> {
        // Base case: if the current node is out of bounds or the node value is greater than the search value, return None
        if node >= self.tree.len() || self.tree[node] > value {
            return None;
        }
    
        // If the current node holds an exact match, return it
        if self.tree[node] == value {
            return Some(self.tree[node]);
        }
    
        // Otherwise, search the left and right children recursively
        let left_match = self.find_first_match(left_child(node), value);
        let right_match = self.find_first_match(right_child(node), value);
    
        // If a match is found in the left subtree, return it; otherwise, return the right subtree's result
        left_match.or(right_match)
    }
    
}

// UTIL FUNCTIONS
pub fn left_child(index: usize) -> usize {
    index * 2 + 1
}

pub fn right_child(index: usize) -> usize {
    index * 2 + 2
}

// Builds a frequency array to track the coverage of positions based on the input segments
fn build_frequency_array(n: usize, segments: &[(usize, usize)]) -> Vec<i32> {
    // freq[x] = how many segments cover the position x
    let mut freq = vec![0; n + 1];

    // for each segment (l, r), increment freq[l] and decrement freq[r+1] (using a differential array technique)
    for &(l, r) in segments {
        freq[l] += 1;
        if r + 1 < n {
            freq[r + 1] -= 1;
        }
    }

    // Accumulate to get coverage at each position
    for i in 1..n {
        freq[i] += freq[i - 1];
    }

    freq.pop(); // Remove extra element due to (n+1) initialization
    freq
}

// data structure to help us manupulate and manage the files
pub struct Test<T> {
    data: Vec<T>,
    queries: Vec<(usize, usize, Option<i32>)>,
    expected_outputs: Vec<i32>,
}

impl<T> Test<T> {
    pub fn data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn queries(&self) -> &Vec<(usize, usize, Option<i32>)> {
        &self.queries
    }

    pub fn expected_outputs(&self) -> &Vec<i32> {
        &self.expected_outputs
    }
}

pub fn load_test(directory: &str, index: usize) -> Test<i32> {
    let input_path = format!("{}/input{}.txt", directory, index);
    let output_path = format!("{}/output{}.txt", directory, index);

    let input_file = BufReader::new(File::open(input_path).unwrap());
    let output_file = BufReader::new(File::open(output_path).unwrap());

    let mut input_lines = input_file.lines().map(|line| line.unwrap());
    let mut output_lines = output_file.lines().map(|line| line.unwrap());

    let header = input_lines.next().unwrap();
    let mut split_header = header.split_whitespace();
    let _n = split_header.next().unwrap().parse::<usize>().unwrap();
    let m = split_header.next().unwrap().parse::<usize>().unwrap();

    let data_line = input_lines.next().unwrap();
    let data = data_line
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut queries = Vec::new();
    let mut expected_outputs = Vec::new();

    for _ in 0..m {
        let query_line = input_lines.next().unwrap();
        let mut split_query = query_line.split_whitespace();

        let query_type = split_query.next().unwrap().parse::<usize>().unwrap();

        if query_type == 0 {
            // Update query
            let l = split_query.next().unwrap().parse::<usize>().unwrap();
            let r = split_query.next().unwrap().parse::<usize>().unwrap();
            let k = split_query.next().unwrap().parse::<i32>().unwrap();
            queries.push((l, r, Some(k)));
        } else {
            // Max query
            let expected = output_lines.next().unwrap().parse::<i32>().unwrap();
            let l = split_query.next().unwrap().parse::<usize>().unwrap();
            let r = split_query.next().unwrap().parse::<usize>().unwrap();
            queries.push((l, r, None));
            expected_outputs.push(expected);
        }
    }

    Test {
        data,
        queries,
        expected_outputs,
    }
}

pub fn main() {
    // problem1();
    problem2();
}

pub fn problem1() {
    let n = 10;
    for i in 0..n {
        // MAKE SURE THE DIRECTORY IS CORRECT
        let test = load_test("data/problem1", i);
        let data = test.data();
        let expected_outputs = test.expected_outputs();
        let queries = test.queries();

        println!("\n------------------------------------");
        println!("Test {}", i + 1);
        println!("Test Data: {:?}", data);
        println!("Test Queries: {:?}", queries);
        println!("Expected Outputs: {:?}", expected_outputs);
        
        let node_function = NodeFunction::Max;
        let mut segment_tree = SegmentTree::init(data, &node_function);

        let mut results = Vec::new();
        for query in queries {
            match query.2 {
                Some(value) => segment_tree.update_range(query.0, query.1, value),
                None => results.push(segment_tree.max_query(query.0, query.1).unwrap()),
            };
        }

        assert!(
            results
                .iter()
                .zip(expected_outputs.iter())
                .all(|(result, expected)| result == expected),
            "Test {} failed: outputs do not match expected values",
            i + 1
        );

        println!("---------> Test {} succeeded!", i + 1);
    }
}

pub fn problem2() {
    println!("***************** Problem 2 *****************");

    let n = 7;
    for i in 0..n {
        println!("\n------------------------------------ Test {}", i + 1);

        let test = load_test_files2("data/problem2", i);

        let data = test.data();
        let queries = test.queries();
        let expected_outputs = test.expected_outputs();

        println!("data: {:?}", data);
        println!("queries: {:?}", queries);
        println!("expected_outputs: {:?}", expected_outputs);

        let data_usize: Vec<(usize, usize)> = data.iter().map(|&(x, y)| (x as usize, y as usize)).collect();
        println!("data_usize: {:?}", data_usize);

        let freq = build_frequency_array(data.len(), &data_usize);

        let node_function = NodeFunction::Min;
        let mut segment_tree = SegmentTree::init(&freq, &node_function);
        
        let mut results = Vec::new();
        for &(i, j, k) in queries {
            let exists = segment_tree.exists_exact_coverage(i, j, k.unwrap());
            results.push(if exists { 1 } else { 0 });
        }

        println!("results: {:?}", results);

        assert!(
            results
                .iter()
                .zip(expected_outputs.iter())
                .all(|(a, b)| a == b),
            "Problem 2: test failed!"
        );

        println!("---------> Test {} succeeded!", i + 1);
    }
}

pub fn load_test_files2(directory: &str, file_number: usize) -> Test<(i32, i32)> {
    let input_file_path = format!("{}/input{}.txt", directory, file_number);
    let output_file_path = format!("{}/output{}.txt", directory, file_number);

    // Open input and output files
    let input_file = BufReader::new(File::open(input_file_path).unwrap());
    let output_file = BufReader::new(File::open(output_file_path).unwrap());

    let mut input_lines = input_file.lines().map(|x| x.unwrap());
    let mut output_lines = output_file.lines().map(|x| x.unwrap());

    // Read n and m
    let (n, m) = {
        let first_line = input_lines.next().unwrap();
        let mut split = first_line.split_whitespace();
        (
            split.next().unwrap().parse().unwrap(),
            split.next().unwrap().parse().unwrap(),
        )
    };

    // Collect data
    let data: Vec<(i32, i32)> = (0..n)
        .map(|_| {
            let line = input_lines.next().unwrap();
            let mut split = line.split_whitespace();
            (
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();

    // Collect queries and expected outputs
    let (queries, expected_outputs): (Vec<_>, Vec<_>) = (0..m)
        .map(|_| {
            let query_line = input_lines.next().unwrap();
            let output = output_lines.next().unwrap().parse::<i32>().unwrap();

            let mut split = query_line.split_whitespace();
            let l = split.next().unwrap().parse().unwrap();
            let r = split.next().unwrap().parse().unwrap();
            let k = split.next().unwrap().parse().unwrap();

            ((l, r, Some(k)), output)
        })
        .unzip();

    Test {
        data,
        queries,
        expected_outputs,
    }
}
