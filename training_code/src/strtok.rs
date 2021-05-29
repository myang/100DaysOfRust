// strtok(s = "hello world", " ")
// return "hello", s = "world"

// 
pub fn strtok<'a>(s: &'a mut &str, pat: char) -> &'a str {
    match s.find(pat) {
        Some(i) => {
            let prefix: &str = &s[..i];
            let suffix: &str = &s[i + pat.len_utf8()..];
            *s = suffix;
            prefix
        }
        None => {
            &s
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s: &str = "hello world";
        assert_eq!(s.find(' '), Some(5));

        let s1: &mut &str = &mut s;
        let t: &str = strtok(s1,' ');
        assert_eq!(t, "hello");
        assert_eq!(*s1, "world");
    }
}