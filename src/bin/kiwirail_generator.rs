use rand::prelude::*;
use std::fs::File;
use std::io::Write;
use color_eyre::eyre::Result;
use clap::Clap;


#[derive(Clap, Debug)]
#[clap(version = "1.0", author = "Albino Cordeiro <albino@intuitionlogic.com>", 
    about = "Generate random graphs file for testing kiwicli")]
struct Options {
    #[clap(short, long, required = true, 
        about = "Number of nodes. Edges will be generated randomly.")]
    num_nodes: usize,
}

fn main () -> Result<()> {
    color_eyre::install()?;
    let options = Options::parse();
    let num_nodes: usize;
    if options.num_nodes > 26 {
        println!("Number of nodes needs to be less than or equal to 26");
        num_nodes = 26;
    } else {
        num_nodes = options.num_nodes;
    }

    let file_name = format!("random_{}.kiwi", num_nodes);
    
    let mut file = File::create(&file_name)?;
    let chars: Vec<char> = "ABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().collect();
    let mut rng = thread_rng();
    for i in 0..num_nodes {
        for j in 0..num_nodes {
            if j != i {
                let dice_throw = rng.gen_range(1..=100*num_nodes);
                if dice_throw < 30 * num_nodes { // connect to 30% other nodes approximately
                    let mut nodepair = (vec![chars[i], chars[j]]).iter().collect::<String>();
                    nodepair.push_str(&dice_throw.to_string());
                    writeln!(&mut file, "{}", nodepair)?;    
                }                
            }
        }
    }
    file.flush()?;
    drop(file); // close file

    Ok(())
}
