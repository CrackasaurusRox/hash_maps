use std::collections::HashMap;

fn main() {
    ex1();
    ex2();
    ex3();
    ex4();
    ex5();
    prob1();
}

fn ex1() {

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);

}

fn ex2() {

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    println!("{:?}", teams);
    println!("{:?}", initial_scores);

    let scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();

    println!("{:?}", scores);

    let team_name = String::from("Blue");
    let score = scores.get(&team_name);

    println!("{:?}", score);
}

fn ex3() {
    let mut scores = HashMap::new();
    
    scores.insert(String::from("Green"), 25);
    scores.insert(String::from("Brown"), 80);
    
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
}

fn ex4() {    
    let mut scores = HashMap::new();

    scores.insert(String::from("Red"), 20);

    println!("{:?}", scores);

    scores.insert(String::from("Red"), 100);

    println!("{:?}", scores);

    scores.entry(String::from("Purple")).or_insert(75);
    scores.entry(String::from("Red")).or_insert(40);

    println!("{:?}", scores);
}

fn ex5() {

    let text = "hello world wonderful world";   
    let mut map = HashMap::new();   
    
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}

fn prob1() {
    let mut vec = vec![8,1,6,9,3,4,7,2,5];
    
    vec.sort_unstable();

    println!("{}", mean(&vec))

}

fn mean(vec: &Vec<usize>) -> usize {
    let mut total = 0;
    for int in vec {
        total += int;
    }
    total/vec.len()
}

