#![allow(dead_code)]
#![allow(unused_variables)]
use core::cmp::Reverse;
use core::fmt::Display;
use std::collections::HashMap;

pub type SizeOfFile = u128;

type DirectoryJournal = HashMap<String, Directory>;
type FileSizes = Vec<(String, SizeOfFile)>;
type ChangeAndLsPair<'a> = Vec<(Vec<&'a str>, Vec<&'a str>)>;

#[derive(Debug, Default)]
struct DirListing(Vec<ChangeDirectory>, Vec<HardNode>);
#[derive(Debug)]
struct FileTree(String, DirectoryJournal);

#[derive(Debug)]
enum HardNode {
    File(String, usize),
    Dir(String),
}
#[derive(Debug)]
enum ChangeDirectory {
    GoToParent,
    GoTo(String),
}

#[derive(Debug)]
struct Directory {
    parent: String,
    entries: Vec<HardNode>,
    name: String,
    full_path: String,
}

pub fn get_number_size_at_most(input: &str, max: usize) -> SizeOfFile {
    let input_parsed = parse_file_system(input);
    let tree = build_file_tree(input_parsed);
    let size_journal = create_list_dir_sizes(&tree);

    size_journal
        .into_iter()
        .filter_map(|directory| {
            let (name, size) = directory;

            if size <= (max as SizeOfFile) {
                Some((name, size))
            } else {
                None
            }
        })
        .map(|journal| journal.1)
        .sum()
}

pub fn get_directory_to_delete(
    input: &str,
    system_capacity: SizeOfFile,
    needed_free_space: SizeOfFile,
) -> (String, SizeOfFile, SizeOfFile) {
    assert!(
        system_capacity > needed_free_space,
        "Need more system space capacity then required"
    );
    let input_parsed = parse_file_system(input);
    let tree = build_file_tree(input_parsed);

    let mut size_journal = create_list_dir_sizes(&tree);
    size_journal.sort_by_key(|entry| {
        let (_, size) = entry;

        Reverse(*size)
    });

    let (_, used_up) = size_journal.first().expect("No root");

    let needed_additional_free = calc_amount_to_free(system_capacity, needed_free_space, *used_up);

    return get_smallest_possible_dir(&size_journal, needed_additional_free);

    fn get_smallest_possible_dir(
        size_journal: &Vec<(String, u128)>,
        needed_additional_free: SizeOfFile,
    ) -> (String, SizeOfFile, SizeOfFile) {
        let smallest_enough_dir = size_journal
            .into_iter()
            .filter(|entry| {
                let (_, size) = entry;

                *size >= needed_additional_free
            })
            .min_by_key(|entry| entry.1)
            .expect("No directory found to be freed for needed amount");

        (
            smallest_enough_dir.0.to_string(),
            smallest_enough_dir.1,
            needed_additional_free,
        )
    }

    fn calc_amount_to_free(
        system_capacity: SizeOfFile,
        needed_free_space: SizeOfFile,
        used_up: SizeOfFile,
    ) -> SizeOfFile {
        let free = system_capacity - used_up;
        assert!(
            needed_free_space >= free,
            "needed_free_space needs to larger than free"
        );

        needed_free_space - free
    }
}

fn create_list_dir_sizes(tree: &FileTree) -> FileSizes {
    let mut sizes: FileSizes = Default::default();

    traverse_to(tree, "/", &mut sizes);
    return sizes;

    fn traverse_to(tree: &FileTree, next: &str, size_journal: &mut FileSizes) -> SizeOfFile {
        let directory = tree.1.get(next).expect("No directory found for given next");
        let mut total_size = SizeOfFile::default();
        for entry in directory.entries.iter() {
            match entry {
                HardNode::File(_, size) => total_size += *size as SizeOfFile,
                HardNode::Dir(name) => {
                    let next_path = FileTree::construct_path(&name, &directory);
                    total_size += traverse_to(tree, &next_path, size_journal);
                }
            }
        }

        let new_size_journal = (next.to_string(), total_size);
        size_journal.push(new_size_journal);

        total_size
    }
}

fn build_file_tree(build_from: Vec<DirListing>) -> FileTree {
    let mut tree = FileTree("/".to_string(), Default::default());
    let mut cwd = String::from("/");
    let mut to_walk = build_from.into_iter();

    let DirListing(_, root_entries) = to_walk.next().expect("No root");
    let root_path = &cwd;
    tree.insert_unique_path(Directory::root());
    let root = tree.get_entries_ref_from(root_path).unwrap();
    root.entries = root_entries;

    for exploration in to_walk {
        let DirListing(steps, nodes) = exploration;

        for cd in steps.into_iter() {
            match cd {
                ChangeDirectory::GoToParent => {
                    let resolved_cwd = tree.1.get(cwd.as_str()).expect("No cwd found");
                    cwd = resolved_cwd.get_parent().to_string();
                }
                ChangeDirectory::GoTo(go_to) => {
                    let parent = cwd.as_str();
                    let resolved_cwd = tree.1.get(cwd.as_str()).expect("No cwd found");
                    let new_dir = Directory::new(parent, &go_to);
                    cwd = new_dir.get_full_path();
                    tree.insert_unique_path(new_dir);
                }
            }
        }

        let current_dir = tree
            .1
            .get_mut(&cwd)
            .expect("No directory found on cwd for insertion.");

        current_dir.entries = nodes;
    }

    tree
}

fn parse_file_system(to_parse: &str) -> Vec<DirListing> {
    let sectioned = section_commands_and_entries(to_parse);
    sectioned
        .into_iter()
        .map(|commands_directory| {
            let (raw_commands, raw_entries) = commands_directory;

            let commands = parse_commands(raw_commands);
            let entries = parse_entries(raw_entries);

            DirListing(commands, entries)
        })
        .collect()
}

fn parse_entries(to_convert: Vec<&str>) -> Vec<HardNode> {
    to_convert
        .into_iter()
        .map(|entry| {
            let mut left_right = entry.split(" ");
            match (left_right.next(), left_right.next()) {
                (Some(left), Some(right)) => {
                    if left.starts_with("dir") {
                        HardNode::Dir(right.to_string())
                    } else {
                        HardNode::File(
                            right.to_string(),
                            left.parse().expect("Could parse file size."),
                        )
                    }
                }
                _ => panic!("Entry not splittable by white space"),
            }
        })
        .collect()
}

fn parse_commands(to_convert: Vec<&str>) -> Vec<ChangeDirectory> {
    to_convert
        .into_iter()
        .map(|line| {
            let dir_name = line.split(" ").nth(2).expect("No dir name found");
            match dir_name {
                ".." => ChangeDirectory::GoToParent,
                otherwise => ChangeDirectory::GoTo(otherwise.to_string()),
            }
        })
        .collect()
}

fn section_commands_and_entries(input: &str) -> ChangeAndLsPair {
    let mut extracted: ChangeAndLsPair = Vec::new();
    let mut commands: Vec<&str> = Default::default();
    let mut entries: Vec<&str> = Default::default();
    let mut expecting_command = true;

    for line in input.lines() {
        if expecting_command {
            if line.contains("$ ls") {
                expecting_command = false;
            } else {
                commands.push(line);
            }
        } else {
            if line.starts_with("$") {
                expecting_command = true;
                extracted.push((commands, entries));
                commands = vec![line];
                entries = Default::default();
            } else {
                entries.push(line);
            }
        }
    }

    extracted.push((commands, entries));

    extracted
}

impl FileTree {
    pub fn insert_unique_path(&mut self, dir: Directory) {
        if self
            .1
            .insert(dir.get_full_path().to_string(), dir)
            .is_some()
        {
            panic!("Path is duplicate");
        }
    }

    fn construct_path(base: &str, dir: &Directory) -> String {
        format!("{}{}/", dir.get_full_path(), base)
    }

    pub fn get_entries_ref_from(&mut self, path: &str) -> Option<&mut Directory> {
        self.1.get_mut(path)
    }
}

impl Display for FileTree {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();
        output.push_str(&format!("{} (dir)\n", "/"));
        traverse(self, "/", 2, &mut output);
        return f.write_str(&output);

        fn traverse(tree: &FileTree, next: &str, indent: usize, output: &mut String) {
            let directory = tree.1.get(next).expect("No directory found for given next");
            let mut prefix = " ".repeat(indent);
            prefix = format!("-{prefix}");

            for entry in directory.entries.iter() {
                match entry {
                    HardNode::File(name, size) => {
                        output.push_str(&format!("{prefix}{} (file, size={})\n", name, size))
                    }
                    HardNode::Dir(name) => {
                        output.push_str(&format!("{prefix}{} (dir)\n", name));
                        let new_path = FileTree::construct_path(&name, &directory);
                        traverse(tree, &new_path, indent + 2, output);
                    }
                }
            }
        }
    }
}

impl Directory {
    fn new(parent: &str, name: &str) -> Self {
        Self {
            parent: parent.to_string(),
            entries: Default::default(),
            name: name.to_string(),
            full_path: format!("{}{}/", parent, name),
        }
    }

    fn root() -> Self {
        let root_name = String::from("/");
        Self {
            parent: String::new(),
            entries: Default::default(),
            name: root_name.clone(),
            full_path: root_name,
        }
    }

    fn get_parent(&self) -> &str {
        &self.parent
    }

    fn get_full_path(&self) -> String {
        self.full_path.clone()
    }
}
