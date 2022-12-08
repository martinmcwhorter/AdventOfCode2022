use std::{collections::{HashMap, HashSet, hash_set}, env::current_exe, borrow::BorrowMut};

#[derive(Debug)]
#[derive(Clone)]
struct File {
    name: String,
    size: u32
}

#[derive(Debug)]
#[derive(Clone)]
struct Directory {
    name: String,
    files: Vec<File>,
    directory_ids: Vec<usize>,
    parent_id: usize,
    id: usize
}

impl Directory {
    fn new(name: String, parend_id: usize, id: usize) -> Self { Self {name, files: vec![], directory_ids: vec![], parent_id: parend_id, id}}

    fn add_file(&mut self, name: String, size: u32) {
        self.files.push(File { name, size });
    }

    fn add_directory_id(&mut self, id: usize) {
        self.directory_ids.push(id);
    }
}

#[derive(Debug)]
struct MasterMap {
    map: HashMap<usize, Directory>,
    index: usize,
    current_directory_id: usize
}

impl MasterMap {
    fn add_directory(&mut self, mut directory: Directory) -> usize {
        self.index += 1;
        directory.id = self.index; 
        self.map.insert(directory.id, directory.clone());
        return self.index;
    }
    
    fn add_directory_name(&mut self, name: String) -> usize {
        let directory = Directory::new(name.to_string(), self.current_directory_id,  0);
        return self.add_directory(directory);
    }

    fn add_file(&mut self, name: String, size: u32) {
        let mut current_directory: Directory = self.map.get(&self.current_directory_id).unwrap().clone();
        current_directory.add_file(name, size);
        self.map.insert(self.current_directory_id, current_directory);
    }


    fn add_directory_id(&mut self, directory_id: usize) {
        let mut current_directory: Directory = self.map.get(&self.current_directory_id).unwrap().clone();
        current_directory.add_directory_id(directory_id);
        self.map.insert(self.current_directory_id, current_directory);
    }

    fn change_directory(&mut self, name: &str) {

        self.current_directory().directory_ids.clone().into_iter().for_each(|d| {
            let candidate = self.map.get(&d).unwrap();
            if candidate.name == name.to_string() {
                self.current_directory_id = candidate.id;
                return;
            }
        });

    }

    fn current_directory(&mut self) -> &Directory {
        return self.map.get(&self.current_directory_id).unwrap();
    }

    fn change_to_parent(&mut self) {
        self.current_directory_id = self.current_directory().parent_id;        
    }

    fn new() -> Self { Self { map: HashMap::new(), index: 0, current_directory_id: 0 } }
}

fn main() {

    let mut root = Directory {
        name: "/".to_string(),
        files: vec![],
        directory_ids: vec![],
        parent_id: 1,
        id: 1
    };

    let mut master = MasterMap::new();
    master.add_directory(root);

    let mut lines: Vec<_> = include_str!("../input.txt").lines().collect();

    while lines.len() > 0 {
        parse_change_directory(&mut master, &mut lines);
        parse_list_directory(&mut master, &mut lines);
    }

    let mut sum_at_most_100k = 0;
    master.map.clone().into_iter().for_each(|d| {
        let sum = get_size_of_dir(&master, &d.1);
        if sum <= 100_000 {
            sum_at_most_100k += sum;
        }
    });

    println!("part 1 {}", sum_at_most_100k);
    
    let space_used = get_size_of_dir(&master, master.map.get(&(1 as usize)).unwrap());
    let space_available = 70000000 - space_used;
    let space_needed = 30000000 - space_available;

    println!("space used {}", space_used);
    println!("space avalabile {}", space_available);
    println!("space needed {}", space_needed);

    let mut deletion_candidates = vec![];
    master.map.clone().into_iter().for_each(|d| {
        let sum = get_size_of_dir(&master, &d.1);
        if sum >= space_needed {
            deletion_candidates.push(sum)
        }
    });

    deletion_candidates.sort();
    println!("part 2 {}", deletion_candidates.first().unwrap());

}

fn parse_change_directory(master: &mut MasterMap, lines: &mut Vec<&str>) -> bool {
    let line = lines.first().unwrap();
    if !line.starts_with("$ cd") {
        return false;
    }

    let split: Vec<&str> = line.split_whitespace().collect();
    let directory_name = split.get(2).unwrap();

    if directory_name.to_string() == "/" {
        master.current_directory_id = 1;
    } else if directory_name.to_string() == ".." {
        master.change_to_parent();
    } else {
        master.change_directory(directory_name);
    }

    lines.remove(0);

    return true;
} 

fn parse_list_directory(master: &mut MasterMap, lines: &mut Vec<&str>) -> bool {
    let line = lines.first().unwrap();
    if !line.contains("$ ls") {
        return false;
    }
    lines.remove(0);


    while lines.len() > 0 && !Option::unwrap(lines.first()).starts_with("$") {
        let line: Vec<_> = lines.remove(0).split_whitespace().collect();

        if line.get(0).unwrap().contains("dir") {
            let dir_id = master.add_directory_name(line.get(1).unwrap().to_string()).clone();
            master.add_directory_id(dir_id);
        } else {
            master.add_file(line.get(1).unwrap().to_string(), line.get(0).unwrap().parse().unwrap())
        }
    }

    return true;
}

fn get_size_of_dir(master: &MasterMap, directory: &Directory) -> u32 {
    
    let mut size = 0;

    let mut directories_to_traverse = vec![directory];
    let mut directories_done: Vec<usize> = vec![];

    while directories_to_traverse.len() > 0 {
        let dir = directories_to_traverse.pop().unwrap();
        directories_done.push(dir.id);

        dir.directory_ids.clone().into_iter().for_each(|d| {
            if !directories_done.contains(&d) {
                directories_to_traverse.push(&master.map.get(&d).unwrap());
            }
        });

        dir.files.clone().into_iter().for_each(|f| size += f.size);
    }

    return size;
}