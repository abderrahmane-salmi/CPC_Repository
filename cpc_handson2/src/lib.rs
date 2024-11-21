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
    
    // Initialize the segment tree from an array
    pub fn init(a: &[i32], node_function: &NodeFunction) -> Self {
        let length = a.len();

        let mut segment_tree = SegmentTree {
            n: length,
            tree: vec![0; 4 * length],
            lazy: vec![None; 4 * length],
        };
        
        segment_tree.populate(a, 0, length - 1, 0, node_function);
        segment_tree
    }

    // Populate the segment tree with the data
    fn populate(
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

        self.populate(arr, start_pos, mid, left_child(curr_node_pos), node_function);
        self.populate(arr, mid + 1, end_pos, right_child(curr_node_pos), node_function);

        // set the value of the current node to the max of the left and right children
        // TODO min for 2 and max for 1
        self.tree[curr_node_pos] = match node_function {
            NodeFunction::Min => self.tree[left_child(curr_node_pos)].min(self.tree[right_child(curr_node_pos)]),
            NodeFunction::Max => self.tree[left_child(curr_node_pos)].max(self.tree[right_child(curr_node_pos)]),
        };
        // self.tree[curr_node_pos] = self.tree[left_child(curr_node_pos)].min(self.tree[right_child(curr_node_pos)]);
    }

    // --------------------- MAX QUERY ---------------------

    // max query in range [l, r]
    pub fn query_max(&mut self, l: usize, r: usize) -> Option<i32> {
        self.max_query_lazy(0, 0, self.n - 1, l-1, r-1)
    }

    fn max_query_lazy(
        &mut self, 
        curr_node_pos: usize, 
        start: usize, 
        end: usize, 
        l: usize, 
        r: usize
    ) -> Option<i32> {
        self.apply_lazy_update(curr_node_pos, start, end);

        if start > r || end < l {
            // no overlap
            return None;
        }

        if start >= l && end <= r {
            // total overlap
            return Some(self.tree[curr_node_pos]);
        }

        // partial overlap
        let mid = (start + end) / 2;
        
        let left_max = self.max_query_lazy(left_child(curr_node_pos), start, mid, l, r);
        let right_max = self.max_query_lazy(right_child(curr_node_pos), mid + 1, end, l, r);
        
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
        self.update_range_lazy(0, 0, self.n - 1, l-1, r-1, t);
    }

    fn update_range_lazy(&mut self, curr_node_pos: usize, start: usize, end: usize, l: usize, r: usize, t: i32) {
        
        // make sure our tree is up-to-date by applying any lazy updates at the start
        self.apply_lazy_update(curr_node_pos, start, end);

        if start > r || end < l { // todo maybe start > end
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
            // partial overlap
            let mid = (start + end) / 2;
            
            self.update_range_lazy(left_child(curr_node_pos), start, mid, l, r, t);
            self.update_range_lazy(right_child(curr_node_pos), mid + 1, end, l, r, t);
            
            self.tree[curr_node_pos] = self.tree[left_child(curr_node_pos)].max(self.tree[right_child(curr_node_pos)]);
        }
    }

    // ---------------------- IS THERE ----------------------

     // Build segment tree from a frequency array
    pub fn build_from_frequency(freq: &[i32], node_function: &NodeFunction) -> Self {
        Self::init(freq, &node_function)
    }

    // Check if there exists a position within range [l, r] with exactly `k` segments covering it
    pub fn exists_exact_coverage(&mut self, l: usize, r: usize, k: i32) -> bool {
        self.range_exact_check(0, 0, self.n - 1, l, r, k)
    }

    // Helper function for exact coverage check in a range
    fn range_exact_check(
        &mut self,
        curr_node_pos: usize,
        start: usize,
        end: usize,
        l: usize,
        r: usize,
        k: i32
    ) -> bool {
        // Apply any pending lazy updates
        self.apply_lazy_update(curr_node_pos, start, end);

        // No overlap
        if start > r || end < l {
            return false;
        }

        // Total overlap
        if start >= l && end <= r {
            // return self.tree[curr_node_pos] == k;
            return self.lower_bound_search(curr_node_pos, k).is_some();
        }

        // Partial overlap
        let mid = (start + end) / 2;
        let left_exists = self.range_exact_check(left_child(curr_node_pos), start, mid, l, r, k);
        let right_exists = self.range_exact_check(right_child(curr_node_pos), mid + 1, end, l, r, k);

        left_exists || right_exists
    }

    fn lower_bound_search(&mut self, current: usize, k: i32) -> Option<i32> {
        if current >= self.tree.len() || self.tree[current] > k {
            return None;
        }

        if self.tree[current] == k {
            return Some(self.tree[current]);
        }

        let left_result = self.lower_bound_search(left_child(current), k);
        let right_result = self.lower_bound_search(right_child(current), k);
        if left_result.is_some() {
            return left_result;
        }
        right_result
    }
    
}

// UTIL FUNCTIONS
pub fn left_child(index: usize) -> usize {
    index * 2 + 1
}

pub fn right_child(index: usize) -> usize {
    index * 2 + 2
}

// Utility function to build frequency array
fn build_frequency_array(n: usize, segments: &[(usize, usize)]) -> Vec<i32> {
    let mut freq = vec![0; n + 1];

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

// Main function to process queries
fn process_is_there_queries(n: usize, segments: &[(usize, usize)], queries: &[(usize, usize, Option<i32>)]) -> Vec<i32> {
    let freq = build_frequency_array(n, segments);

    let node_function = NodeFunction::Min;

    let mut segment_tree = SegmentTree::build_from_frequency(&freq, &node_function);
    let mut results = Vec::new();

    for &(i, j, k) in queries {
        let exists = segment_tree.exists_exact_coverage(i, j, k.unwrap());
        results.push(if exists { 1 } else { 0 });
    }

    results
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
                None => results.push(segment_tree.query_max(query.0, query.1).unwrap()),
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

        let test = get_tests_ex2("data/problem2", i);

        let data = test.data();

        assert_eq!(
            test.queries().len(),
            test.expected_outputs().len(),
            "Error in reading test data"
        );

        // Count array
        let mut count = vec![0; data.len() + 1];
        for &elem in data {
            count[elem.0 as usize] += 1;
            count[elem.1 as usize + 1] -= 1;
        }

        // Prefix sums array
        let mut prefix_sum: Vec<i32> = vec![0; data.len() + 1];
        prefix_sum[0] = count[0];
        for i in 1..data.len() {
            prefix_sum[i] = prefix_sum[i - 1] + count[i];
        }

        // Last element is useless for the segment tree
        prefix_sum.pop();

        // Actually we only need propagate because we are not updating any values
        let node_function = NodeFunction::Min;
        let mut segment_tree = SegmentTree::init(&prefix_sum, &node_function);
        // segment_tree.print();

        let queries = test.queries();
        let expected_outputs = test.expected_outputs();

        println!("data: {:?}", data);
        println!("queries: {:?}", queries);
        println!("expected_outputs: {:?}", expected_outputs);

        let data_usize: Vec<(usize, usize)> = data.iter().map(|&(x, y)| (x as usize, y as usize)).collect();
        println!("data_usize: {:?}", data_usize);
        let results2 = process_is_there_queries(data.len(), &data_usize, &queries);

        println!("results2: {:?}", results2);

        assert!(
            results2
                .iter()
                .zip(expected_outputs.iter())
                .all(|(a, b)| a == b),
            "Problem 2: test failed!"
        );

        println!("---------> Test {} succeeded!", i + 1);
    }
}

pub fn get_tests_ex2(directory: &str, file_number: usize) -> Test<(i32, i32)> {
    let input_file_path = format!("{}/input{}.txt", directory, file_number);
    let output_file_path = format!("{}/output{}.txt", directory, file_number);

    let mut file_iter_input = BufReader::new(File::open(input_file_path).unwrap())
        .lines()
        .map(|x| x.unwrap());

    let mut file_iter_output = BufReader::new(File::open(output_file_path).unwrap())
        .lines()
        .map(|x| x.unwrap());

    // Read the first line for n and m
    let mut binding = file_iter_input.next().unwrap();
    let mut iter = binding.split_whitespace();
    let n = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    let mut data = Vec::with_capacity(n);

    for _ in 0..n {
        binding = file_iter_input.next().unwrap();
        iter = binding.split_whitespace();
        let x = iter.next().unwrap().parse::<i32>().unwrap();
        let y = iter.next().unwrap().parse::<i32>().unwrap();
        data.push((x, y));
    }

    let mut queries = Vec::new();
    let mut expected_outputs = Vec::new();

    for _ in 0..m {
        binding = file_iter_input.next().unwrap();
        iter = binding.split_whitespace();

        let output = file_iter_output.next().unwrap().parse::<i32>().unwrap();
        let l = iter.next().unwrap().parse::<usize>().unwrap();
        let r = iter.next().unwrap().parse::<usize>().unwrap();
        let k = iter.next().unwrap().parse::<i32>().unwrap();
        queries.push((l, r, Some(k)));
        expected_outputs.push(output);
    }

    Test {
        data,
        queries,
        expected_outputs,
    }
}
