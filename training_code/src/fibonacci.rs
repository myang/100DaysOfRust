struct Fibonacci {
    a: u64;
    b: u64,
    cur: u8,
    total: u8,
}

impl Fibonacci {
    fn new(total: u8) -> Self {
        Self {
            a: 0,
            b: 0,
            cur: 0,
            total,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let fibo = Fibonacci::new(10);
        for item in fibo {
            println!("tem: {}", item);
        }
    }
}