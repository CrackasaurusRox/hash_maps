fn main() {
    use std::collections::HashMap;
    /*
    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("{:?}", scores);
    */
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

   for (key, value) in &scores {
       println!("{}: {}", key, value);
   }

   let mut scores2 = HashMap::new();

   scores2.insert(String::from("Red"), 20);
   println!("{:?}", scores2);
   
   scores2.insert(String::from("Red"), 100);
   println!("{:?}", scores2);

   scores2.entry(String::from("Purple")).or_insert(75);
   scores2.entry(String::from("Red")).or_insert(40);

   println!("{:?}", scores2);
}