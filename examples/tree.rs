use will_o_wisp::files::tree::{scan_dir, tree};

fn main() {
    let mut node = scan_dir(&"explorer".into());

    println!("{:#?}\n", node);
}
