fn pig_latin(english_words:&str) -> String{
    let mut wordsmut= english_words.to_string();
    wordsmut.make_ascii_lowercase();
    if wordsmut.starts_with(|c:char|(c == 'a') || (c == 'e') || (c == 'i') || (c == 'o') || (c == 'u')){
        wordsmut.push_str("-hay");

    }
    else {
        let first_char = wordsmut.remove(0);
        wordsmut.push('-');
        wordsmut.push(first_char);
        wordsmut.push_str("ay");
    }
    wordsmut
}
fn main() {
    let english_word = String::from("BOOK");
    println!("{} pig latin word is {}",english_word,pig_latin(&english_word[..]));
}
