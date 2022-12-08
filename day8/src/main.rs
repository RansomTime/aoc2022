const INPUT: &str = include_str!("../inputs/input");

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> u32 {
    let grid = TreeGrid::from(INPUT);
    let mut res = 0;
    let size = grid.contents.len();
    for i in 0..size {
        for j in 0..size {
            if grid.visible_at(i,j) {
                res += 1
            }
        }
    }
    res

}

fn part_2() -> usize {
    let grid = TreeGrid::from(INPUT);
    let mut res = 0;
    let size = grid.contents.len();
    for i in 0..size {
        for j in 0..size {
            let score = grid.get_score(i,j);
            if score > res { 
                res = score;
            }
        }
    }
    res
}

struct TreeGrid {
    contents: Vec<Vec<i32>>,
}

impl TreeGrid {
    fn from(input: &str) -> TreeGrid {
        // Assumption: It's square (both input and demo fit this)
        let size:usize = input.split('\n').count();
        let mut contents =  vec![vec![0;size];size];
        for (row, line) in input.split('\n').enumerate() {
            for (col, c) in line.chars().enumerate() {
                contents[row][col] = c.to_string().parse().unwrap();
            }
        }
        TreeGrid { contents }
    }

    /*
        A tree is visible if all of the other trees between it
        and an edge of the grid are shorter than it.
    */

    fn vis_from_n(&self, row: usize, col: usize) -> bool {
        let tree = self.contents[row][col];
        for i in 0..row{
            if self.contents[i][col] >= tree {
                return false;
            }            
        }
        true
    }
    fn vis_from_s(&self, row: usize, col: usize) -> bool {
        let tree = self.contents[row][col];
        for i in row+1..self.contents.len() {
            if self.contents[i][col] >= tree {
                return false;
            }            
        }
        true
    }
    fn vis_from_w(&self, row: usize, col: usize) -> bool {
        let tree = self.contents[row][col];
        for i in 0..col {
            if self.contents[row][i] >= tree {
                return false;
            }            
        }
        true
    }
    fn vis_from_e(&self, row: usize, col: usize) -> bool {
        let tree = self.contents[row][col];
        for i in col+1..self.contents.len() {
            if self.contents[row][i] >= tree {
                return false;
            }            
        }
        true
    }
    
    /* 
        A tree can view up to the edge of the map
        or until we see a tree that is taller
    */
    
    fn view_n(&self, row:usize, col:usize) -> usize {
        let tree = self.contents[row][col];
        let mut seen = 0;
        for i in (0..row).rev() {
            seen += 1;
            if self.contents[i][col] >= tree {
                return seen;
            }

        }
        seen
    }
    fn view_s(&self, row:usize, col:usize) -> usize {
        let tree = self.contents[row][col];
        let mut seen = 0;
        for i in row+1..self.contents.len() {
            seen += 1;
            if self.contents[i][col] >= tree {
                return seen;
            }

        }
        seen
    }
    fn view_w(&self, row:usize, col:usize) -> usize {
        let tree = self.contents[row][col];
        let mut seen = 0;
        for i in (0..col).rev() {
            seen += 1;
            if self.contents[row][i] >= tree {
                return seen;
            }
        }
        seen
    }
    fn view_e(&self, row:usize, col:usize) -> usize {
        let tree = self.contents[row][col];
        let mut seen = 0;
        for i in col+1..self.contents.len() {
            seen += 1;
            if self.contents[row][i] >= tree {
                return seen;
            }
        }
        seen
    }

    fn get_score(&self, row: usize, col: usize) -> usize {
        self.view_n(row, col) *
        self.view_s(row, col) *
        self.view_e(row, col) *
        self.view_w(row, col)
    }
        

    fn visible_at(&self, row: usize, col: usize) -> bool {
        let max = self.contents.len() - 1;
        if row == 0 || col == 0 ||
        row == max || col == max {  // optimisation: special case of 0
            return true;
        }

        self.vis_from_n(row,col) ||
        self.vis_from_s(row,col) ||
        self.vis_from_e(row,col) ||
        self.vis_from_w(row,col)     

    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn test_tree_from() {
        let demo = include_str!("../inputs/demo");
        let grid = TreeGrid::from(demo);
        assert_eq!(grid.contents[0][0],3);
        assert_eq!(grid.contents[0][1],0);
        assert_eq!(grid.contents[0][2],3);
    }

    #[test]
    fn test_edge_is_always_visible() {
        let demo = include_str!("../inputs/demo");
        let grid = TreeGrid::from(demo);
        for i in 0..5 {
            assert!(grid.visible_at(0, i));
            assert!(grid.visible_at(4, i));
            assert!(grid.visible_at(i, 0));
            assert!(grid.visible_at(i, 4));
        }
    }

    #[test]
    fn test_demo_2_1() {
        let demo = include_str!("../inputs/demo");
        let grid = TreeGrid::from(demo);
        assert!(!grid.vis_from_n(2, 1));
        assert!(!grid.vis_from_s(2, 1));
        assert!(!grid.vis_from_w(2, 1));
        assert!( grid.vis_from_e(2, 1));
    }

    #[test]
    fn test_demo_full() {
        let demo = include_str!("../inputs/demo");
        let grid = TreeGrid::from(demo);
        assert!( grid.visible_at(1, 1));
        assert!( grid.visible_at(1, 2));
        assert!(!grid.visible_at(1, 3));
        assert!( grid.visible_at(2, 1));
        assert!(!grid.visible_at(2, 2));
        assert!( grid.visible_at(2, 3));
        assert!(!grid.visible_at(3, 1));
        assert!( grid.visible_at(3, 2));
        assert!(!grid.visible_at(3, 3));
    }

    #[test]
    fn test_part_1() {
        let demo = include_str!("../inputs/demo");
        let grid = TreeGrid::from(demo);
        let mut res = 0;
        let size = grid.contents.len();
        for i in 0..size {
            for j in 0..size {
                if grid.visible_at(i,j) {
                    res += 1
                }
            }
        }
        assert_eq!(res,21);
    }

    #[test]
    fn test_view_part_2() {
        let demo = include_str!("../inputs/demo");
        let grid = TreeGrid::from(demo);

        // the 2 examples for part 2
        assert_eq!(grid.view_n(1,2),1);
        assert_eq!(grid.view_w(1,2),1);
        assert_eq!(grid.view_e(1,2),2);
        assert_eq!(grid.view_s(1,2),2);   
        
        assert_eq!(grid.view_n(3,2),2);
        assert_eq!(grid.view_w(3,2),2);
        assert_eq!(grid.view_e(3,2),2);
        assert_eq!(grid.view_s(3,2),1);   

    }

    #[test]
    fn test_part_2() {
        let demo = include_str!("../inputs/demo");
        let grid = TreeGrid::from(demo);
        let size = grid.contents.len();
        let mut res = 0;
        for i in 0..size {
            for j in 0..size {
                let score = grid.get_score(i,j);
                if score > res { 
                    res = score;
                }
            }
        }
    assert_eq!(res,8);
    }
}