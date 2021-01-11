use std::{
    collections::HashMap,
    io::{stdin, stdout, Write},
};

fn main() {
    num_demo();
    emp_demo();
}

fn num_demo() {
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Blue")).or_insert(25);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }

    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);

    let intlist = vec![2, 2, 1, 5, 4, 5, 6, 3, 7];
    println!("avg: {}", avg(&intlist));
    println!("median: {}", median(&intlist));
    println!("mode: {}", mode(&intlist));
    println!("pig: {}", pig_latin("first"));
}

fn avg(data: &Vec<u32>) -> f64 {
    let sum: u32 = data.iter().sum();
    let size = data.len();
    sum as f64 / size as f64
}

fn median(data: &Vec<u32>) -> u32 {
    let mut s = Vec::new();
    for i in data {
        s.push(i);
    }
    s.sort();
    *s[s.len() / 2]
}

fn mode(data: &Vec<u32>) -> u32 {
    let mut h = HashMap::new();
    for i in data {
        let count = h.entry(i).or_insert(0);
        *count += 1;
    }
    let mut mode = 0;
    let mut max_count = 0;
    for (key, value) in h {
        if max_count < value {
            max_count = value;
            mode = *key;
        }
    }
    mode
}

fn pig_latin(data: &str) -> String {
    let mut word = String::from(data);
    let first = word.chars().nth(0).unwrap();
    match first {
        'a' | 'i' | 'u' | 'e' | 'o' => {
            word.push_str("hay");
        }
        _ => {
            word.push(first);
            word.remove(0);
            word.push_str("ay");
        }
    }
    word
}

fn emp_demo() {
    let mut employees: HashMap<String, Vec<String>> = HashMap::new();
    loop {
        let mut input = String::new();
        println!("Command: quit, add 部署 名前, list 部署");
        print!("> ");
        stdout().flush().unwrap();
        stdin().read_line(&mut input).ok();
        let commands: Vec<&str> = input.trim().split_whitespace().collect();

        if commands.len() > 0 {
            if !run_command(commands, &mut employees) {
                break;
            }
        }
    }
}

fn run_command(commands: Vec<&str>, employees: &mut HashMap<String, Vec<String>>) -> bool {
    match commands[0] {
        "quit" => false,
        "add" if commands.len() == 3 => {
            add_employee(employees, commands[1], commands[2]);
            true
        }
        "list" => {
            match commands.get(1) {
                Some(department) => println!("{:?}", employees.get::<str>(department)),
                None => println!("{:?}", employees),
            }
            true
        }
        _ => true,
    }
}

fn add_employee(employees: &mut HashMap<String, Vec<String>>, department: &str, employee: &str) {
    employees
        .entry(department.to_string())
        .or_insert(Vec::new());
    employees
        .get_mut(department)
        .unwrap()
        .push(employee.to_string());
}
