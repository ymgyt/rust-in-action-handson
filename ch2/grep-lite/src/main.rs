use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;

fn main() -> Result<(), anyhow::Error>{
    let mut args = env::args();
    let re = Regex::new(&&args.nth(1).expect("search word required")).unwrap();
    let path = args.next().expect("file required");
    let path = Path::new(&path);
    let f = File::open(&path)?;
    let reader = BufReader::new(f);

    let context_lines = 2;

    let mut tags: Vec<usize> = Vec::new();
    let mut ctx: Vec<Vec<(usize, String)>> = Vec::new();
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        if re.is_match(&line){
            tags.push(i);

            let v = Vec::with_capacity(2*context_lines+1);
            ctx.push(v);
        }
    }
    if tags.len() == 0 {
        return Ok(())
    }

    let reader = BufReader::new(File::open(&path)?);
    for (i, line) in reader.lines().enumerate() {
        let line = line?;
        for (j, tag) in tags.iter().enumerate() {
            let lower_bound = tag.saturating_sub(context_lines);
            let upper_bound = tag + context_lines;
            if (i >= lower_bound) && (i <= upper_bound) {
                let local_ctx = (i, line.clone());
                ctx[j].push(local_ctx)
            }
        }
    }

    for local_ctx in ctx.iter() {
        for &(i, ref line) in local_ctx.iter() {
            let line_num = i+1;
            println!("{}: {}", line_num, line);
        }
    }
    Ok(())
}
