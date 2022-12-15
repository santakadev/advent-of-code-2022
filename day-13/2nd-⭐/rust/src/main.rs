use std::fs;
use std::fmt::Debug;

#[derive(Debug)]
enum Token {
    OpenBrace,
    CloseBrace,
    Colon,
    Integer(usize)
}

#[derive(Debug, Clone)]
enum Item {
    Integer(usize),
    List(Vec<Box<Item>>)
}

trait Push {
    fn push(&mut self, item: Item);
}

impl Push for Item
{
    fn push(&mut self, item: Item) {
        match self {
            Item::Integer(_) => panic!("Cannot push to an integer"),
            Item::List(list) => list.push(Box::new(item))
        }
    }
}

#[derive(Debug, PartialEq)]
enum CmpResult {
    Equal,
    Greater,
    Less
}

trait Cmp {
    fn compare(&self, other: &Item) -> CmpResult;
}

impl Cmp for Item {
    fn compare(&self, other: &Item) -> CmpResult {
        match (self, other) {
            (Item::Integer(a), Item::Integer(b)) => {
                if a == b {
                    CmpResult::Equal
                } else if a > b {
                    CmpResult::Greater
                } else {
                    CmpResult::Less
                }
            }
            (Item::List(_), Item::Integer(b)) => {
                self.compare(&Item::List(vec![Box::new(Item::Integer(*b))]))
            }
            (Item::Integer(a), Item::List(_)) => {
                Item::List(vec![Box::new(Item::Integer(*a))]).compare(other)
            }
            (Item::List(a), Item::List(b)) => {
                for (i, j) in a.iter().zip(b.iter()) {
                    match i.compare(j) {
                        CmpResult::Equal => continue,
                        CmpResult::Greater => return CmpResult::Greater,
                        CmpResult::Less => return CmpResult::Less
                    }
                }
                if a.len() == b.len() {
                    CmpResult::Equal
                } else if a.len() > b.len() {
                    CmpResult::Greater
                } else {
                    CmpResult::Less
                }
            },
        }
    }
}
impl PartialEq for Item {
    fn eq(&self, other: &Item) -> bool {
        self.compare(other) == CmpResult::Equal
    }
}

impl Eq for Item {}

impl Ord for Item {
    fn cmp(&self, other: &Item) -> std::cmp::Ordering {
        match self.compare(other) {
            CmpResult::Equal => std::cmp::Ordering::Equal,
            CmpResult::Greater => std::cmp::Ordering::Greater,
            CmpResult::Less => std::cmp::Ordering::Less
        }
    }
}

impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Item) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}


fn main() {
    let input = fs::read_to_string("src/input.txt").unwrap();

    let mut items = input
        .trim_end()
        .replace("\n\n", "\n")
        .split("\n")
        .map(|i| parse(&tokenize(i)))
        .collect::<Vec<Item>>();

    let divider1 = parse(&tokenize("[[2]]"));
    let divider2 = parse(&tokenize("[[6]]"));

    items.push(divider1.clone());
    items.push(divider2.clone());

    items.sort();

    let mut decoder_key = 1;

    for (i, item) in items.iter().enumerate() {
        if item == &divider1 || item == &divider2 {
            decoder_key *= i + 1;
        }
    }

    println!("Decoder key: {:?}", decoder_key);
}

fn parse(tokens: &Vec<Token>) -> Item {
    let mut list_stack: Vec<Item> = Vec::new();
    let list: Item = Item::List(Vec::new());
    list_stack.push(list);

    for t in tokens[1..tokens.len()-1].iter() {
        match t {
            Token::OpenBrace => {
                let new_list = Item::List(Vec::new());
                list_stack.push(new_list);
            },
            Token::CloseBrace => {
                let pop_list = list_stack.pop().unwrap();
                list_stack.last_mut().unwrap().push(pop_list);
            },
            Token::Colon => {
            },
            Token::Integer(i) => {
                list_stack.last_mut().unwrap().push(Item::Integer(*i));
            }
        }
    }

    return list_stack.pop().unwrap();
}

fn tokenize(item: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();

    let mut index = 0;
    while index < item.len() {
        let c = item.chars().nth(index).unwrap();
        match c {
            '[' => tokens.push(Token::OpenBrace),
            ']' => tokens.push(Token::CloseBrace),
            ',' => tokens.push(Token::Colon),
            '0'..='9' => {
                let mut number = String::new();
                while index < item.len() {
                    let c = item.chars().nth(index).unwrap();
                    if c.is_numeric() {
                        number.push(c);
                        index += 1;
                    } else {
                        break;
                    }
                }
                tokens.push(Token::Integer(number.parse::<usize>().unwrap()));
                continue;
            }
            _ => panic!("Unexpected character: {}", c)
        }
        index += 1;
    }

    tokens
}
