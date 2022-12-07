use crate::common::Solution;
use std::collections::HashMap;

#[derive(Debug, Default)]
struct FsDir<'a> {
    dirs: HashMap<&'a str, FsDir<'a>>,
    files: HashMap<&'a str, usize>,
}

fn total_size(fs_tree: &FsDir) -> usize {
    let self_size: usize = fs_tree.files.values().sum();
    let sub_size: usize = fs_tree.dirs.values().map(total_size).sum();
    self_size + sub_size
}

fn solve_a(fs_tree: &FsDir) -> usize {
    const SIZE_LIMIT: usize = 100000;
    let size_here = total_size(fs_tree);
    (if size_here <= SIZE_LIMIT {
        size_here
    } else {
        0
    }) + fs_tree.dirs.values().map(|dir| solve_a(dir)).sum::<usize>()
}

pub fn solve(lines: &[String]) -> Solution {
    let (fs_tree, _, _): (FsDir, _, _) = lines.iter().filter(|line| !line.is_empty()).fold(
        (FsDir::default(), vec![], false),
        |(mut fs_tree, mut cwd_stack, is_ls_cmd): (FsDir, Vec<&str>, bool), line| {
            if line == "$ ls" {
                (fs_tree, cwd_stack, true)
            } else if line.starts_with("$ cd ") {
                match &line[5..] {
                    ".." => {
                        cwd_stack.pop();
                    }
                    "/" => {
                        cwd_stack.clear();
                    }
                    new_dir => cwd_stack.push(new_dir),
                };
                (fs_tree, cwd_stack, false)
            } else if is_ls_cmd {
                let mut cwd: &mut FsDir = &mut fs_tree;
                for cd in &cwd_stack {
                    cwd = cwd.dirs.get_mut(cd).unwrap();
                }
                let mut splits = line.split(' ').peekable();
                if splits.peek().unwrap() == &"dir" {
                    splits.next();
                    let name = splits.next().unwrap();
                    cwd.dirs.insert(name, FsDir::default());
                } else {
                    let size = splits.next().unwrap().parse().unwrap();
                    let name = splits.next().unwrap();
                    cwd.files.insert(name, size);
                }
                (fs_tree, cwd_stack, is_ls_cmd)
            } else {
                unimplemented!()
            }
        },
    );
    println!("{:?}", fs_tree);
    (solve_a(&fs_tree).to_string(), "".to_string())
}
