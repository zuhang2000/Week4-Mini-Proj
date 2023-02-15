//use clap to parse command line arguments
use clap::Parser;

#[derive(Parser)]
#[clap(version = "1.0", author = "Zuhang Xu", about = "file word count")]

// build a cli that calculates the number of words in a file and also how many different words are in the file
struct Opts {
    #[clap(short, long, default_value = "demonstrate.txt")]
    input: String,
}

//main function that reads the file and counts the number of words and different words
fn main() {
    let opts: Opts = Opts::parse();
    let file = std::fs::read_to_string(opts.input).expect("Unable to read file");
    let mut word_count = 0;
    let mut different_words = 0;
    let mut word_list = Vec::new();
    for line in file.lines() {
        for word in line.split_whitespace() {
            word_count += 1;
            if !word_list.contains(&word) {
                different_words += 1;
                word_list.push(word);
            }
        }
    }
    println!("Word Count:{}, Count Different words:{}", word_count, different_words);
}

