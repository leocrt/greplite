use regex::Regex;
use clap::{App,Arg};

fn main() {

    let args = App::new("grep-ite")
        .version("0.1")
        .about("searches for patterns")
        .arg(Arg::with_name("pattern")
            .help("The pattern to search for")
            .takes_value(true)
            .required(true))
        .get_matches();

    let pattern = args.value_of("pattern").unwrap();
    let re = Regex::new(pattern).unwrap();
    let quote =  "\
    Every face, every shop, 
    bedroom window, public-house, and
    dark square is a picture 
    feverishly turned--in search of what?
    It is the same with books. 
    What do we seek
    through millions of pages?";

    for line in quote.lines(){
        let contains_substring = re.find(line);
        match contains_substring{
            Some(_)=> println!("{}", line),
            None => (),
        }      
    }

    // let mut tags: Vec<usize> = Vec::new();
    // let mut ctx:  Vec<Vec<(usize, String)>> = Vec::new(); 

    // for (i, line) in haystack.lines().enumerate(){
    //     if line.contains(needle){
    //         tags.push(i);
    //         let v = Vec::with_capacity(2*context_lines + 1);
    //         ctx.push(v);
    //     }
    // }

    // if tags.len() == 0 {
    //     return;
    // }

    // for (i, line) in haystack.lines().enumerate() { 
    //     for (j, tag) in tags.iter().enumerate() {
    //         let lower_bound = tag.saturating_sub(context_lines); 
    //         let upper_bound = tag + context_lines;
    //         if (i >= lower_bound) && (i <= upper_bound) {
    //             let line_as_string = String::from(line); 
    //             let local_ctx = (i, line_as_string);
    //             ctx[j].push(local_ctx);
    //         }
    //     }
    // }
    // for local_ctx in ctx.iter() {
    //     for &(i, ref line) in local_ctx.iter() { 
    //         let line_num = i + 1;
    //         println!("{}: {}", line_num, line);
    //     }
    // }
}
