struct Note {
    title: String,
    body: String,
}

fn count_char(s: &str, c: char) -> usize {
    let mut nr:usize=0;
    for i in s.chars() { 
        if i==c{
            nr+=1;
        }
    }
    nr
}

fn first_word(s: &str) -> &str { 
    let bytes: &[u8] = s.as_bytes();
    let mut i:usize=0;
    while i<bytes.len(){
        if bytes[i]==32{
            return &s[0..i];
        }
        i+=1;
    }
    s
}

fn reverse(s: &str) -> String { 
    let mut gnirts = String::new();
    for c in s.chars() {
        gnirts.insert(0,c);
    }
    gnirts
}

fn is_palindrome(s: &str) -> bool { 
    let palindrom = reverse(s);
    let mut i:usize=0;
    for c in s.chars() {
        if c!=palindrom.as_bytes()[i] as char{
            return false;
        }
        i+=1;
    }
    true
}

fn repeat_str(s: &str, n: usize) -> String {
    let mut strain = String::new();
    let mut i:usize=0;
    while i<n{
        strain.push_str(s);
        i+=1;
    }
    strain
}

fn add_tag(s: &mut String, tag: &str) { 
    let mut full_tag = String::from("[");
    full_tag.push_str(&tag);
    full_tag.push(']');
    s.push_str(&full_tag);
}

fn censor(s: &mut String, c: char) { 
    let mut new_s = String::new();
    for ch in s.chars() {
        if ch==c {
            new_s.push('*');
        }
        else{
            new_s.push(ch);
        }
    }
    *s=new_s;
}

fn shout(s: String) -> String {  
    s.to_uppercase()
}

fn longer_of(a: String, b: String) -> String { 
    if a.len()<b.len(){
        b
    }
    else {
        a
    }
}

fn make_note(title: &str, body: &str) -> Note {  
    Note {
        title: title.to_string(),
        body: body.to_string(),
    }   
}

fn word_count(note: &Note) -> usize { 
    let mut word_nr:usize=0;
    let mut is_word:bool;
    let mut was_word:bool=false;

    for c in note.body.chars(){
        if c.is_alphabetic() || c.is_numeric(){
            is_word=true;
        }    
        else {
            is_word=false;
        }
        if is_word && !was_word{
            word_nr+=1;
        }
        was_word=is_word;
    }
    word_nr
}

fn append(note: &mut Note, text: &str) { 
    if !note.body.is_empty(){
        note.body.push(' ');
    }
    note.body.push_str(text);
}

fn archive(note: Note) -> String { 
    let mut result = String::from("[ARCHIVED] ");
    result.push_str(&note.title);
    result.push_str(": ");
    result.push_str(&note.body);
    result
}

fn merge(a: Note, b: &Note) -> Note { 
    let mut ab=a;
    ab.title.push('+');
    ab.title.push_str(&b.title);
    ab.body.push(' ');
    ab.body.push_str(&b.body);
    ab
}

fn main() {
    let text = String::from("level up your rust");
    println!("{}", count_char(&text, 'e'));
    println!("{}", count_char("banana", 'a'));
    println!("{}", first_word(&text));
    println!("{}", first_word("rust"));
    println!("{}", reverse(&text));
    println!("{} {}", is_palindrome("level"), is_palindrome(&text));
    println!("{}", repeat_str("ab", 3));

    let mut s = String::from("hello world");
    add_tag(&mut s, "v2");
    println!("{s}");
    censor(&mut s, 'o');
    println!("{s}");

    let loud = shout(s);
    println!("{loud}");
    let winner = longer_of(String::from("tiny"), loud);
    println!("{winner}");

    let title = String::from("shopping");
    let mut n1 = make_note(&title, "milk");
    let n2 = make_note("ideas", "learn rust borrowing");
    println!("{title}");
    println!("{} word(s)", word_count(&n1));
    append(&mut n1, "eggs");
    append(&mut n1, "bread");
    println!("{}: {}", n1.title, n1.body);
    let merged = merge(n1, &n2);
    println!("{}: {}", merged.title, merged.body);
    println!("{} word(s)", word_count(&merged));
    println!("{}", archive(n2));
}