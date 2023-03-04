use std::{fs::File, io::BufRead};

#[derive(Debug, Clone, Copy)]
struct Tree {
    height: i32,
    seen: bool,
}

struct Forest {
    tree_matrix: Vec<Vec<Tree>>,
}

impl Forest {
    fn new() -> Forest {
        Forest {
            tree_matrix: vec![],
        }
    }

    fn add_tree_row(&mut self, treeRow: String) {
        let mut row: Vec<Tree> = vec![];
        for i in treeRow.chars() {
            let tree = Tree {
                height: i.to_digit(10).unwrap() as i32,
                seen: false,
            };
            row.push(tree);
        }
        self.tree_matrix.push(row);
    }

    fn get_total_seen(&self) -> i32 {
        let mut total = 0;
        for row in &self.tree_matrix {
            for tree in row {
                if tree.seen {
                    total += 1;
                }
            }
        }
        total
    }

    fn update_tree_seen(&mut self) {
        let n = self.tree_matrix.len();
        let m = self.tree_matrix[0].len();

        // from left
        for j in 0..m {
            let mut lo = -1;
            for i in 0..n {
                if self.tree_matrix[i][j].height > lo {
                    self.tree_matrix[i][j].seen = true;
                    lo = self.tree_matrix[i][j].height;
                }
            }
        }

        // from right
        for j in (0..m) {
            let mut lo = -1;
            for i in (0..n).rev() {
                if self.tree_matrix[i][j].height > lo {
                    self.tree_matrix[i][j].seen = true;
                    lo = self.tree_matrix[i][j].height;
                }
            }
        }

        // from top
        for i in 0..n {
            let mut lo = -1;
            for j in 0..m {
                if self.tree_matrix[i][j].height > lo {
                    self.tree_matrix[i][j].seen = true;
                    lo = self.tree_matrix[i][j].height;
                }
            }
        }

        // from bottom
        for i in (0..n) {
            let mut lo = -1;
            for j in (0..m).rev() {
                if self.tree_matrix[i][j].height > lo {
                    self.tree_matrix[i][j].seen = true;
                    lo = self.tree_matrix[i][j].height;
                }
            }
        }
    }

    fn get_scenic_score(&self, i: usize, j: usize) -> i32 {
        // println!("i:{} j:{}", i, j);
        // println!("left:{}", self.get_left_tree(i, j));
        // println!("right:{}", self.get_right_tree(i, j));
        // println!("top:{}", self.get_top_tree(i, j));
        // println!("bottom:{}", self.get_bottom_tree(i, j));

        self.get_left_tree(i, j)
            * self.get_right_tree(i, j)
            * self.get_top_tree(i, j)
            * self.get_bottom_tree(i, j)
    }

    fn get_top_tree(&self, old_i: usize, j: usize) -> i32 {
        let original_height = self.tree_matrix[old_i][j].height;
        let mut no_of_trees_in_view = 0;
        let mut block_height: Option<i32> = None;
        for i in (0..old_i).rev() {
            let (updated_block_height, is_in_view) =
                self.is_in_view(i, j, block_height, original_height);
            if is_in_view {
                no_of_trees_in_view += 1;
            }

            if let Some(height) = updated_block_height {
                break;
            }
        }
        no_of_trees_in_view
    }

    fn get_bottom_tree(&self, old_i: usize, j: usize) -> i32 {
        let original_height = self.tree_matrix[old_i][j].height;
        let mut no_of_trees_in_view = 0;
        let mut block_height: Option<i32> = None;
        for i in (old_i + 1)..self.tree_matrix.len() {
            let (updated_block_height, is_in_view) =
                self.is_in_view(i, j, block_height, original_height);
            block_height = updated_block_height;
            if is_in_view {
                no_of_trees_in_view += 1;
            }

            if let Some(height) = updated_block_height {
                break;
            }
        }
        no_of_trees_in_view
    }

    fn get_left_tree(&self, i: usize, old_j: usize) -> i32 {
        let original_height = self.tree_matrix[i][old_j].height;
        let mut no_of_trees_in_view = 0;
        let mut block_height: Option<i32> = None;
        for j in (0..old_j).rev() {
            let (updated_block_height, is_in_view) =
                self.is_in_view(i, j, block_height, original_height);
            // block_height = updated_block_height;
            if is_in_view {
                no_of_trees_in_view += 1;
            }

            if let Some(height) = updated_block_height {
                break;
            }
        }
        no_of_trees_in_view
    }

    fn get_right_tree(&self, i: usize, old_j: usize) -> i32 {
        let original_height = self.tree_matrix[i][old_j].height;
        let mut no_of_trees_in_view = 0;
        let mut block_height: Option<i32> = None;
        for j in (old_j + 1)..self.tree_matrix[0].len() {
            let (updated_block_height, is_in_view) =
                self.is_in_view(i, j, block_height, original_height);
            // block_height = updated_block_height;
            if is_in_view {
                no_of_trees_in_view += 1;
            }

            if let Some(height) = updated_block_height {
                break;
            }
        }
        no_of_trees_in_view
    }

    // return is_blocked and is_in_view
    fn is_in_view(
        &self,
        i: usize,
        j: usize,
        block_height: Option<i32>,
        original_height: i32,
    ) -> (Option<i32>, bool) {
        // original height
        let new_tree = self.tree_matrix[i][j];
        match block_height {
            Some(block_height) => {
                if new_tree.height > block_height {
                    return (Some(new_tree.height), true);
                } else {
                    return (Some(block_height), false);
                }
            }
            None => {
                if new_tree.height >= original_height {
                    return (Some(new_tree.height), true);
                } else {
                    return (None, true);
                }
            }
        }
    }

    fn print(&self) {
        for row in &self.tree_matrix {
            for tree in row {
                if (tree.seen) {
                    print!("1");
                } else {
                    print!("0");
                }
            }
            println!();
        }
    }
}

fn main() {
    // buf reader
    let mut file = File::open("input").expect("Unable to open file");
    let mut forest = Forest::new();
    for line in std::io::BufReader::new(file).lines() {
        forest.add_tree_row(line.unwrap());
    }

    forest.update_tree_seen();
    forest.print();
    let total = forest.get_total_seen();
    println!("{}", total);

    // TEST for input

    // assert!(forest.get_top_tree(1, 2) == 1);
    // assert!(forest.get_bottom_tree(1, 2) == 2);
    // assert!(forest.get_left_tree(1, 2) == 1);
    // assert!(forest.get_right_tree(1, 2) == 2);

    // assert!(forest.get_scenic_score(1, 2) == 4);

    // assert!(forest.get_top_tree(3, 2) == 2);
    // assert!(forest.get_bottom_tree(3, 2) == 1);
    // assert!(forest.get_left_tree(3, 2) == 2);
    // assert!(forest.get_right_tree(3, 2) == 2);

    // assert!(forest.get_scenic_score(3, 2) == 8);
    // assert!(solution_2(forest)== 8);

    solution_2(forest);
}

fn solution_2(forest: Forest) -> i32 {
    let mut max_score = 0;
    let mut x = 0;
    let mut y = 0;
    for i in 0..forest.tree_matrix.len() {
        for j in 0..forest.tree_matrix[0].len() {
            let score = forest.get_scenic_score(i, j);
            if score > max_score {
                max_score = score;
                x = i;
                y = j;
            }
        }
    }

    println!("x:{}, y:{} ans:{}", x, y, max_score);
    max_score
}
