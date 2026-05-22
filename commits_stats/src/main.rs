use commits_stats::{commits_per_author, commits_per_week};

fn main() {
    let contents = include_str!("../commits.json");

    let serialized = json::parse(contents).unwrap();

    println!("{:?}", commits_per_week(&serialized));
    println!("{:?}", commits_per_author(&serialized));
}