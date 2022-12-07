use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;

const INPUT: &str = include_str!("../inputs/input");

fn main() {
    println!("part 1: {}", part_1());
    println!("part 2: {}", part_2());
}

fn part_1() -> u32 {
    0
}

fn part_2() -> u32 {
    0
}
#[derive(Debug)]
struct DirectoryTree {
    parent: Option<Rc<RefCell<DirectoryTree>>>,
    name: String,
    children: Vec<Rc<RefCell<DirectoryTree>>>,
    files: Vec<DirectoryFile>
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

    fn add_file(&mut self,name: &str, size: i32) {
        self.files.push( DirectoryFile{
            name: String::from(name),
            size
        });
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
            sum += file.size;
        }
        for child in &self.children {
            sum += child.borrow().size_of_children();
        }
        sum
    }
}

#[derive(Debug)]
struct DirectoryFile {
    name: String,
    size: i32,
}

#[derive(Debug)]
enum DirectoryItem {
    Directory(DirectoryTree),
    Item(DirectoryFile)
}

#[cfg(test)]
mod test {
    use std::ops::{Deref, DerefMut};

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
        test.add_file("a", 23);
        test.add_file("b", 18);

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
        test.borrow_mut().add_file("e",10);
        assert_eq!(test.borrow().size_of_children(),10);
    }

    #[test]
    fn test_sum_with_children() {
        let root = Rc::from(RefCell::from(DirectoryTree::new_root()));
        root.borrow_mut().add_dir("ab/",Rc::clone(&root));
        root.borrow_mut().add_dir("cd/",Rc::clone(&root));
        root.borrow_mut().add_file("e",10);
        root.borrow().children[0].borrow_mut().deref_mut().add_file("f", 23);
        assert_eq!(root.borrow().size_of_children(),33);
    }

    /*
    #[test]
    fn test_part_1() {
        let root = Rc::from(RefCell::from(DirectoryTree::new_root()));
        let mut pwd = Rc::clone(&root);
        let input: &str = include_str!("../inputs/demo");
        let mut lines = input.split('\n');
        let mut parserMode = "cmd";
        while let Some(line) = lines.next() {
            if line == "$ cd /" {
                pwd = Rc::clone(&root);
                continue;
            }
            if line == "$ ls" {
                continue;
            }
            if line == "$ cd .." {
                let next = Rc::clone(&pwd.borrow_mut().get_parent());
                pwd = next;
                continue;
            }
            /*if line.starts_with("$ cd ") {
                let search = line.get_mut(6..).unwrap();
                for child in pwd.borrow().children {
                    match child.borrow_mut().deref_mut() {
                        DirectoryItem::Directory(dir) => {
                            if dir.name == String::from(search) {
                                pwd = Rc::clone(&pwd);
                            }                            
                        },
                        _ => (),
                    }
                }
            }*/
                
            
            print!("{}",line);
        }
    }*/
}