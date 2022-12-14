use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::Rc;

const INPUT: &str = include_str!("../inputs/input");

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn parse_input(input: &str) -> Rc<RefCell<DirectoryTree>> {
    let root = Rc::from(RefCell::from(DirectoryTree::new_root()));
    let mut pwd = Rc::clone(&root);
    for line in input.split('\n') {
        if line.starts_with("$ cd /") {
            pwd = Rc::clone(&root);
            continue;
        }
        if line.starts_with("$ ls") {
            continue;
        }
        if line.starts_with("$ cd ..") {
            let next = Rc::clone(&pwd.borrow_mut().get_parent());
            pwd = next;
            continue;
        }
        if line.starts_with("$ cd ") {
            let search = line.strip_prefix("$ cd ").unwrap();
            let pwd_ = Rc::clone(&pwd);
            // telling rust softly that I love it
            // and I'm not going to ruin it's nicely allocated memory
            let mut canary = true;
            for child in &pwd_.borrow().children {
                if child.borrow().name == *search {
                    pwd = Rc::clone(child);
                    canary = false;
                    break;
                }                     
            }
            if canary {
                panic!("directory {search} not found.");
            }
            continue;
        }
        if line.starts_with("dir ") {
            let new_name = line.strip_prefix("dir ").unwrap();
            let next = Rc::clone(&pwd);
            pwd.borrow_mut().deref_mut().add_dir(new_name, next);
            continue;
        }
        // else
        let mut file = line.split(' ');
        let size:i32 = file.next().unwrap().parse().unwrap();
        pwd.borrow_mut().add_file(size);                
    }
    root
}

fn part_1() -> i32 {
    let root = parse_input(INPUT);
    let mut res = 0;
    let kids = root.borrow().better_size_of_children();
    for e in kids {
        if e <= 100_000 {
            res += e;
        }
    }
    res
}

fn part_2() -> i32 {
    let root = parse_input(INPUT);
    let unused_space = 70_000_000-root.borrow().size_of_children();
    let needed_space = 30_000_000-unused_space;

    let kids = root.borrow().better_size_of_children();
    let mut res = i32::MAX;
    for e in kids {
        if e >= needed_space && e <= res {
            res = e;
        }
    }
    res
}
#[derive(Debug)]
struct DirectoryTree {
    parent: Option<Rc<RefCell<DirectoryTree>>>,
    name: String,
    children: Vec<Rc<RefCell<DirectoryTree>>>,
    files: Vec<i32>
}

impl DirectoryTree {
    fn new_root() -> DirectoryTree{
        DirectoryTree {
            parent: None,
            name: String::from("/"),
            children: vec![],
            files: vec![],    
        }
    }

    fn get_parent(&self) -> Rc<RefCell<DirectoryTree>> {
        match &self.parent {
            Some(parent) => Rc::clone(parent),
            None => unreachable!(),
        }
    }

    fn add_file(&mut self, size: i32) {
        self.files.push(size);
    }

    fn add_dir(&mut self, name: &str, parent: Rc<RefCell<DirectoryTree>>) {
        let dir = Rc::new(RefCell::new(
            DirectoryTree{
                parent: Some(parent),
                name: String::from(name),
                children: Vec::new(),
                files: Vec::new(),
            }));
        self.children.push(dir);
    }

    fn size_of_children(&self) -> i32 {
        let mut sum = 0;
        for file in &self.files {
            sum += file;
        }
        for child in &self.children {
            sum += child.borrow().size_of_children();
        }
        sum
    }

    fn better_size_of_children(&self) -> Vec<i32> {
        let mut res = Vec::new();
        res.push(self.size_of_children());
        for child in &self.children {
            res.extend(child.borrow().better_size_of_children());
        }
        res

    }
}

#[cfg(test)]
mod test {
    use std::ops::{DerefMut};

    use crate::*;
    #[test]
    fn test_root_dir() {
        let test = DirectoryTree::new_root();

        assert!(test.parent.is_none());
        assert_eq!(test.name,String::from("/"));
        assert_eq!(test.children.len(),0);
    }

    #[test]
    fn test_add_file() {
        let mut test = DirectoryTree::new_root();
        test.add_file(23);
        test.add_file(18);

        assert_eq!(test.size_of_children(),41);
    }

    #[test]
    fn test_add_dir() {
        let test = Rc::from(RefCell::from(DirectoryTree::new_root()));
        test.borrow_mut().add_dir("ab/",Rc::clone(&test));
        test.borrow_mut().add_dir("cd/",Rc::clone(&test));

        assert_eq!(test.borrow().children.len(),2);
    }

    

    #[test]
    fn test_sum_with_dirs() {
        let test = Rc::from(RefCell::from(DirectoryTree::new_root()));
        test.borrow_mut().add_dir("ab/",Rc::clone(&test));
        test.borrow_mut().add_dir("cd/",Rc::clone(&test));
        test.borrow_mut().add_file(10);
        assert_eq!(test.borrow().size_of_children(),10);
    }

    #[test]
    fn test_sum_with_children() {
        let root = Rc::from(RefCell::from(DirectoryTree::new_root()));
        root.borrow_mut().add_dir("ab/",Rc::clone(&root));
        root.borrow_mut().add_dir("cd/",Rc::clone(&root));
        root.borrow_mut().add_file(10);
        root.borrow().children[0].borrow_mut().deref_mut().add_file(23);
        assert_eq!(root.borrow().size_of_children(),33);
    }

    
    #[test]
    fn test_part_1() {
        let input: &str = include_str!("../inputs/demo");
        let root = parse_input(input);

        assert_eq!(48_381_165,root.borrow().size_of_children());
        let mut res = 0;
        let kids = root.borrow().better_size_of_children();
        for e in kids {
            if e <= 100_000 {
                res += e;
            }
        }
        assert_eq!(res, 95_437)
    }
}