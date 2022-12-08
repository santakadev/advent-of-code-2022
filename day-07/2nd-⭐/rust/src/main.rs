use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug)]
enum Command {
    Ls(Vec<String>),
    Cd(String)
}

#[derive(Debug, Clone)]
struct TreeNode {
    name: String,
    files_sizes: Vec<u64>,
    children: Vec<Rc<RefCell<TreeNode>>>,
    parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(name: String) -> Self {
        TreeNode {
            name,
            files_sizes: Vec::new(),
            children: vec![],
            parent: None,
        }
    }
}

fn parse_commands(input: String) -> Vec<Command> {
    let unparsed_commands = input.split("$ ").collect::<Vec<&str>>()[1..].to_vec();

    let mut parsed_commands: Vec<Command> = Vec::new();
    unparsed_commands.iter().for_each(|command| {
        let command_parts: Vec<&str> = command.trim_end().split("\n").collect();

        let command = command_parts[0].split(" ").collect::<Vec<&str>>();

        match command[0] {
            "ls" => {
                let mut args = Vec::new();
                for arg in command_parts[1..].iter() {
                    args.push(arg.to_string());
                }
                parsed_commands.push(Command::Ls(args));
            },
            "cd" => {
                parsed_commands.push(Command::Cd(command[1].to_string()));
            },
            _ => {
                println!("Unknown command: {}", command[0]);
            }
        }

    });

    parsed_commands
}

fn build_tree(commands: &mut Vec<Command>) -> Rc<RefCell<TreeNode>> {
    let first = commands.first().unwrap();

    let root: Rc<RefCell<TreeNode>> = match first {
        Command::Cd(args) => {
            Rc::new(RefCell::new(TreeNode::new(args.to_string())))
        },
        _ => {
            panic!("First command must be cd");
        }
    };
    let mut current = Rc::clone(&root);
    commands.remove(0);

    for c in commands {
        current = match c {
            Command::Ls(args) => {
                args.iter().for_each(|arg| {
                    let parts = arg.split(" ").collect::<Vec<&str>>();
                    let name = parts[1];

                    if parts[0] == "dir" {
                        let child = Rc::new(RefCell::new(TreeNode::new(name.to_string())));
                        child.borrow_mut().parent = Some(Rc::clone(&current));
                        current.borrow_mut().children.push(Rc::clone(&child));
                    } else {
                        current.borrow_mut().files_sizes.push(parts[0].parse::<u64>().unwrap());
                    }

                });
                current
            },
            Command::Cd(arg) => {
                if arg == ".." {
                    Rc::clone(current.borrow().parent.as_ref().unwrap())
                } else {
                    Rc::clone(current.borrow().children.iter().find(|c| c.borrow().name == arg.to_string()).unwrap())
                }
            }
        };
    }

    root
}


fn main() {
    let input = std::fs::read_to_string("src/input.txt").unwrap();

    let mut parsed_commands = parse_commands(input);
    let root = build_tree(&mut parsed_commands);
    let mut dir_sizes = directories_sizes(root.borrow().to_owned());
    let result = directory_to_remove(&mut dir_sizes);
    println!("Directory to remove: {:?}", result);
}

#[derive(Debug, Clone)]
struct Directory {
    name: String,
    size: u64,
}

fn directory_to_remove(dir_sizes: &mut Vec<Directory>) -> Directory {
    dir_sizes.sort_by(|a, b| b.size.cmp(&a.size));
    //println!("{:?}", dir_sizes.iter().map(|f| f.name.clone()).collect::<Vec<String>>());
    //println!("{:?}", dir_sizes.iter().map(|f| f.size).collect::<Vec<u64>>());

    let disk_space = 70000000;
    let target_size = 30000000;
    let total_used = dir_sizes.first().unwrap().size;
    let unused = disk_space - total_used;
    let size_to_remove = target_size - unused;

    let mut target_index = 0;
    for (i, dir) in dir_sizes.iter().enumerate() {
        if dir.size < size_to_remove {
            target_index = i - 1;
            break;
        }
    }

    dir_sizes[target_index].clone()
}

fn directories_sizes(root: TreeNode) -> Vec<Directory> {
    let mut vec: Vec<Directory> = Vec::new();
   walk(root, &mut vec);
   return vec;
}   

fn walk(node: TreeNode, acc: &mut Vec<Directory>) -> u64 {
    let mut total: u64 = 0;

    let mut children = node.children;
    children.iter_mut().for_each(|child| {
        total += walk(child.borrow().clone(), acc);
    });

    total += node.files_sizes.iter().sum::<u64>();

    acc.push(Directory {
        name: node.name,
        size: total,
    });

    return total;
}

