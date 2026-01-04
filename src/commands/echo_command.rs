use crate::helpers::output;

pub fn execute(argv: Vec<String>) -> () {
    let text = argv[1..argv.len()].join(" ");
    output(format!("{text}\n",).as_str());
}
