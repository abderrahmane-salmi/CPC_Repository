use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub struct SegmentTree {
    n: usize,
    tree: Vec<i32>,
    lazy: Vec<i32>,
}

impl SegmentTree {

    // TODO: merge new and build functions
    pub fn from_vec(a: &[i32]) -> Self {
        let length = a.len();

        let mut segment_tree = SegmentTree {
            n: length,
            tree: vec![0; 4 * length],
            lazy: vec![i32::MAX; 4 * length],
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

    // // initialize the segment tree with given array size n
    // pub fn new(n: usize) -> Self {
    //     SegmentTree {
    //         n,
    //         tree: vec![0; 4 * n],
    //         lazy: vec![i32::MAX; 4 * n],
    //     }
    // }

    // // build the segment tree using the array values
    // pub fn build(&mut self, arr: &[i32], node: usize, start: usize, end: usize) {
    //     print!("{} {} {}\n", node, start, end);
    //     if start == end {
    //         // base case: this is a leaf node
    //         print!("this is a leaf node {} {}\n", node, start);
    //         self.tree[node] = arr[start];
    //         return;
    //     }

    //     let mid = (start + end) / 2;
        
    //     // left child
    //     self.build(arr, left_child(node), start, mid);
        
    //     // right child
    //     self.build(arr, right_child(node), mid + 1, end);
        
    //     // merge the children nodes
    //     self.tree[node] = self.tree[left_child(node)].max(self.tree[right_child(node)]);
    // }

    // apply the lazy update to a node
    fn apply_lazy(&mut self, node_pos: usize, start: usize, end: usize) {
        if self.lazy[node_pos] == i32::MAX {
            // there are no updates to apply
            return;
        }

        // apply the pending min operation to this segment
        self.tree[node_pos] = self.tree[node_pos].min(self.lazy[node_pos]);
        
        // propagate the lazy value to the node's children if its not a leaf node
        if start != end {
            self.lazy[left_child(node_pos)] = self.lazy[left_child(node_pos)].min(self.lazy[node_pos]);
            self.lazy[right_child(node_pos)] = self.lazy[right_child(node_pos)].min(self.lazy[node_pos]);
        }
        
        // clear the lazy value for this node
        self.lazy[node_pos] = i32::MAX;
    }

    // range update
    fn update(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize, t: i32) {
        
        // make sure our tree is up-to-date by applying any lazy updates at the start
        self.apply_lazy(node, start, end);

        if start > r || end < l {
            // no overlap, skip
            return;
        }

        if start >= l && end <= r {
            // total overlap
            self.lazy[node] = t;
            self.apply_lazy(node, start, end);
        } else {
            // partial overlap
            let mid = (start + end) / 2;
            
            self.update(left_child(node), start, mid, l, r, t);
            self.update(right_child(node), mid + 1, end, l, r, t);
            
            self.tree[node] = self.tree[left_child(node)].max(self.tree[right_child(node)]);
        }
    }

    // max query in range [l, r]
    fn max_query(&mut self, node: usize, start: usize, end: usize, l: usize, r: usize) -> i32 {
        self.apply_lazy(node, start, end);

        if start > r || end < l {
            // no overlap
            return i32::MIN;
        }

        if start >= l && end <= r {
            // total overlap
            return self.tree[node];
        }

        // Partial overlap
        let mid = (start + end) / 2;
        
        let left_max = self.max_query(left_child(node), start, mid, l, r);
        let right_max = self.max_query(right_child(node), mid + 1, end, l, r);
        
        left_max.max(right_max)
    }

    // wrapper functions for update and max_query for easier access
    pub fn update_range(&mut self, l: usize, r: usize, t: i32) {
        print!("Update range: [{}, {}] with {}\n", l, r, t);
        self.print();
        print!("Updating...\n");
        self.update(0, 0, self.n - 1, l, r, t);
        self.print();
    }

    pub fn query_max(&mut self, l: usize, r: usize) -> i32 {
        print!("Query max: [{}, {}]\n", l, r);
        let result = self.max_query(0, 0, self.n - 1, l, r);
        self.print();
        print!("Result: {}\n", result);
        result
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
                None => results.push(segment_tree.query_max(query.0, query.1)),
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