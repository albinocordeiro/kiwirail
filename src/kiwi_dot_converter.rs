use color_eyre::eyre::Result;
use super::input_parsing::*;
use petgraph::dot::Dot;
use std::fs::File;
use std::io::Write;


pub fn kiwi2dot(file_name: &str) -> Result<()> {
    let graph = from_kiwi_file(file_name)?;
    let dot_file_name = format!("{}.dot", file_name);
    let mut f = File::create(&dot_file_name)?;
    let output = format!("{}", Dot::new(&graph));
    f.write_all(&output.as_bytes())?;
    println!("Output written to {} ", dot_file_name);
    println!("If you have graphviz installed you can generate a visual representation of the graph with: $> dot -T png -O {}", dot_file_name);

    Ok(())
}