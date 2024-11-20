use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub struct SegmentTree {
    n: usize,
    tree: Vec<i32>,
    lazy: Vec<Option<i32>>,
}

impl SegmentTree {
    
    pub fn from_vec(a: &[i32]) -> Self {
        let length = a.len();

        let mut segment_tree = SegmentTree {
            n: length,
            tree: vec![0; 4 * length],
            lazy: vec![None; 4 * length],
        };
        
        segment_tree.populate(a, 0, length - 1, 0);
        segment_tree
    }

    fn populate(&mut self, arr: &[i32], min_pos: usize, max_pos: usize, current_pos: usize) {
        // Base case: leaves are the same as the data
        if min_pos == max_pos {
            self.tree[current_pos] = arr[max_pos];
            return;
        }

        let mid = (min_pos + max_pos) / 2;
        let left_child = left_child(current_pos);
        let right_child = right_child(current_pos);

        // Left
        self.populate(arr, min_pos, mid, left_child);

        // Right
        self.populate(arr, mid + 1, max_pos, right_child);

        // Combine
        self.tree[current_pos] = self.tree[left_child].max(self.tree[right_child]);
    }

    // ---------------------

    pub fn query_max(&mut self, l: usize, r: usize) -> Option<i32> {
        print!("Query max: [{}, {}]\n", l, r);
        let result = self.max_query_lazy(0, 0, self.n - 1, l-1, r-1); // maybe l-1, r-1?
        self.print();
        print!("Result: {}\n", result.unwrap_or(-1));
        result
    }

    // max query in range [l, r]
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

    // ----------------------

    pub fn update_range(&mut self, l: usize, r: usize, t: i32) {
        print!("Update range: [{}, {}] with {}\n", l, r, t);
        self.print();
        print!("Updating...\n");
        
        self.update_range_lazy(0, 0, self.n - 1, l-1, r-1, t); // todo check if l-1, r-1
        
        
        self.print();
    }


    // range update
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

    

    // TODO: add print fun
    pub fn print(&self) {
        self.print_recursive(0, 0, self.n - 1);
    }

    // Helper function to print the segment tree recursively
    fn print_recursive(&self, pos: usize, left: usize, right: usize) {
        println!(
            "Node: {}, Range: [{}, {}], Value: {:?}, Lazy: {:?}",
            pos, left, right, self.tree[pos], self.lazy[pos]
        );

        if left != right {
            let mid = (left + right) / 2;
            self.print_recursive(left_child(pos), left, mid);
            self.print_recursive(right_child(pos), mid + 1, right);
        }
    }
    
}

// UTIL FUNCTIONS
pub fn left_child(index: usize) -> usize {
    index * 2 + 1
}

pub fn right_child(index: usize) -> usize {
    index * 2 + 2
}

// data structure to help us manupulate and manage the files
pub struct Test<T> {
    data: Vec<T>,
    queries: Vec<(usize, usize, Option<i32>)>,
    expected_outputs: Vec<i32>,
}

impl<T> Test<T> {
    pub fn get_data(&self) -> &Vec<T> {
        &self.data
    }

    pub fn get_queries(&self) -> &Vec<(usize, usize, Option<i32>)> {
        &self.queries
    }

    pub fn get_expected_outputs(&self) -> &Vec<i32> {
        &self.expected_outputs
    }
}

pub fn get_tests(directory: &str, file_number: usize) -> Test<i32> {
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
    let _ = iter.next().unwrap().parse::<usize>().unwrap();
    let m = iter.next().unwrap().parse::<usize>().unwrap();

    // Read the second line for the array
    binding = file_iter_input.next().unwrap();
    iter = binding.split_whitespace();
    let data = iter
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<i32>>();

    let mut queries = Vec::new();
    let mut expected_outputs = Vec::new();

    for _ in 0..m {
        binding = file_iter_input.next().unwrap();
        iter = binding.split_whitespace();

        // Update query
        if iter.next().unwrap().parse::<usize>().unwrap() == 0 {
            let l = iter.next().unwrap().parse::<usize>().unwrap();
            let r = iter.next().unwrap().parse::<usize>().unwrap();
            let k = iter.next().unwrap().parse::<i32>().unwrap();
            queries.push((l, r, Some(k)));

        // Max query
        } else {
            let output = file_iter_output.next().unwrap().parse::<i32>().unwrap();
            let l = iter.next().unwrap().parse::<usize>().unwrap();
            let r = iter.next().unwrap().parse::<usize>().unwrap();
            queries.push((l, r, None));
            expected_outputs.push(output);
        }
    }

    Test {
        data,
        queries,
        expected_outputs,
    }
}

pub fn main() {
    // todo update n
    let n = 9;
    for i in 0..n {
        let test = get_tests("data/problem1", i);
        let data = test.get_data();
        let expected_outputs = test.get_expected_outputs();
        let queries = test.get_queries();

        println!("Test {}", i);
        println!("Data: {:?}", data);
        println!("Queries: {:?}", queries);
        println!("Expected Outputs: {:?}", expected_outputs);
        
        let mut segment_tree = SegmentTree::from_vec(data);
        // let mut segment_tree = SegmentTree::new(n);
        // segment_tree.build(&data, 0, 0, n-1);

        let mut results: Vec<i32> = Vec::new();
        for query in queries {
            match query.2 {
                Some(t) => segment_tree.update_range(query.0, query.1, t),
                None => results.push(segment_tree.query_max(query.0, query.1).unwrap()),
            };
        }

        assert!(
            results
                .iter()
                .zip(expected_outputs.iter())
                .all(|(a, b)| a == b),
            "Exercise 1: test failed!"
        );
    }
}