use std::collections::HashMap;

use clap::Parser;
use lazy_static::lazy_static;

fn main() {
    let args = Args::parse();

    let mut changes = vec![];
    for (i, char) in args.text.char_indices() {
        if let Some(replacement) = SHAV2LAT.get(&char) {
            changes.push((i..(i + char.len_utf8()), replacement))
        }
    }

    let mut text = args.text;
    for (range, replacement) in changes.iter().rev() {
        text.replace_range(range.clone(), replacement);
    }

    println!("{text}");
}

/// Program to translate Shavian characters to Latin characters.
///     Doesn't work very well.
#[derive(Parser, Debug)]
#[command(version, long_about)]
struct Args {
    /// Text with Shavian characters
    text: String,
}

lazy_static! {
    static ref SHAV2LAT: HashMap<char, &'static str> = HashMap::from([
        ('𐑐', "p"),
        ('𐑑', "t"),
        ('𐑒', "k"),
        ('𐑓', "f"),
        ('𐑔', "th"),
        ('𐑕', "s"),
        ('𐑖', "sh"),
        ('𐑗', "ch"),
        ('𐑘', "y"),
        ('𐑙', "ng"),
        ('𐑚', "b"),
        ('𐑛', "d"),
        ('𐑜', "g"),
        ('𐑝', "v"),
        ('𐑞', "th"),
        ('𐑟', "z"),
        ('𐑠', "sh"),
        ('𐑡', "j"),
        ('𐑢', "w"),
        ('𐑣', "h"),
        ('𐑤', "l"),
        ('𐑮', "r"),
        ('𐑥', "m"),
        ('𐑯', "n"),
        ('𐑦', "i"),
        ('𐑰', "ee"),
        ('𐑧', "e"),
        ('𐑱', "ay"),
        ('𐑨', "a"),
        ('𐑲', "ai"),
        ('𐑩', "a"),
        ('𐑳', "u"),
        ('𐑪', "o"),
        ('𐑴', "oh"),
        ('𐑫', "oo"),
        ('𐑵', "oo"),
        ('𐑬', "ou"),
        ('𐑶', "oi"),
        ('𐑭', "ah"),
        ('𐑷', "aw"),
        ('𐑸', "ar"),
        ('𐑹', "or"),
        ('𐑺', "air"),
        ('𐑻', "er"),
        ('𐑼', "ar"),
        ('𐑽', "ear"),
        ('𐑾', "ia"),
        ('𐑿', "you"),
    ]);
}
