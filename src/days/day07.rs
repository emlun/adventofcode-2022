use crate::common::Solution;
use std::collections::BTreeMap;

#[derive(Debug, Default)]
struct FsDir<'a> {
    dirs: BTreeMap<&'a str, FsDir<'a>>,
    files_size: usize,
}

impl<'a> FsDir<'a> {
    fn total_size(&self) -> usize {
        self.files_size + self.dirs.values().map(FsDir::total_size).sum::<usize>()
    }
}

fn solve_a(fs_tree: &FsDir) -> usize {
    const SIZE_LIMIT: usize = 100000;
    let size_here = fs_tree.total_size();
    (if size_here <= SIZE_LIMIT {
        size_here
    } else {
        0
    }) + fs_tree.dirs.values().map(solve_a).sum::<usize>()
}

fn visit_all<'a, 'b: 'a>(
    mut visited: Vec<&'b FsDir<'a>>,
    current: &'b FsDir<'a>,
) -> Vec<&'a FsDir<'a>> {
    visited.push(current);
    current.dirs.values().fold(visited, visit_all)
}

fn solve_b(fs_tree: &FsDir) -> usize {
    const MAX_SIZE: usize = 70000000;
    const TARGET_FREE_SIZE: usize = 30000000;
    let size_here = fs_tree.total_size();

    let delete_size = TARGET_FREE_SIZE - (MAX_SIZE - size_here);

    let candidate_dirs: Vec<&FsDir> = visit_all(Vec::new(), fs_tree);
    candidate_dirs
        .into_iter()
        .map(FsDir::total_size)
        .filter(|size| *size >= delete_size)
        .min()
        .unwrap()
}

pub fn solve(lines: &[String]) -> Solution {
    let (fs_tree, _, _): (FsDir, _, _) = lines.iter().filter(|line| !line.is_empty()).fold(
        (FsDir::default(), Vec::new(), false),
        |(mut fs_tree, mut cwd_stack, is_ls_cmd): (FsDir, Vec<&str>, bool), line| {
            if line == "$ ls" {
                (fs_tree, cwd_stack, true)
            } else if let Some(cd) = line.strip_prefix("$ cd ") {
                match cd {
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
                let cwd: &mut FsDir = cwd_stack
                    .iter()
                    .fold(&mut fs_tree, |mutref, cd| mutref.dirs.get_mut(cd).unwrap());

                if let Some(dir_name) = line.strip_prefix("dir ") {
                    cwd.dirs.insert(dir_name, FsDir::default());
                } else {
                    cwd.files_size += line.split(' ').next().unwrap().parse::<usize>().unwrap();
                }
                (fs_tree, cwd_stack, is_ls_cmd)
            } else {
                unimplemented!()
            }
        },
    );
    (solve_a(&fs_tree).to_string(), solve_b(&fs_tree).to_string())
}
