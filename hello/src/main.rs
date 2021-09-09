use std::io::Write;

fn main() {
    println!("Hello, world!");

    // str, string
    let s1: String = String::from("Hello, world!");
    let s2: &str = &s1;
    let s3: String = s2.to_string();
    println!("{}", s3);

    // tuple
    let mut t = (1, "2");
    t.0 = 2;
    t.1 = "3";
    println!("{} {}", t.0, t.1);

    // array
    let mut a: [i32; 3] = [0, 1, 2];
    let b: [i32; 3] = [0; 3];
    a[1] = b[1];
    a[2] = b[2];
    println!("{:?}", &a[1..3]);
    println!("{:?}", &b[0..3]);

    // struct
    struct Person {
        name: String,
        age: u32,
    }

    let p = Person {
        name: String::from("John"),
        age: 8,
    };

    // enum
    enum Event {
        Quit,
        KeyDown(u8),
        MouseDown { x: i32, y: i32 },
    }

    let e1 = Event::Quit;
    let e2 = Event::MouseDown { x: 10, y: 10 };

    // Option implementation
    // pub enum Option<T> {
    //     None,
    //     Same(T),
    // }

    // Result implementation
    // pub enum Result<T, E> {
    //     Ok(T),
    //     Err(E),
    // }

    let result: Result<i32, String> = Ok(200);

    match result {
        Ok(code) => println!("code: {}", code),
        Err(err) => println!("Err: {}", err),
    }

    let result: Result<i32, String> = Ok(200);

    if let Ok(code) = result {
        println!("code: {}", code);
    }

    // not using nest, unwrap_or()
    let result: Result<i32, String> = Ok(200);
    println!("code: {}", result.unwrap_or(-1)); // => "code: 200"
    let result: Result<i32, String> = Err("error".to_string());
    println!("code: {}", result.unwrap_or(-1)); // => "code: -1"

    // and_then()
    fn func(code: i32) -> Result<i32, String> {
        println!("code: {}", code);
        Ok(100)
    }

    let result: Result<i32, String> = Ok(200);
    let next_result = result.and_then(func); // func will be executed
    let result: Result<i32, String> = Err("error".to_string());
    let next_result = result.and_then(func); // func will not be executed

    // others: ?
    fn error_handling(result: Result<i32, String>) -> Result<i32, String> {
        let code = result?; // if error, return result
        println!("code: {}", code);
        Ok(100)
    }

    // Vec
    let v1 = vec![1, 2, 3, 4, 5];
    let v2 = vec![0; 5];

    let v = vec![1, 2, 3, 4, 5];
    println!("{}", v[0]);

    let v = vec![1, 2, 3, 4, 5];
    for element in &v {
        println!("{}", element);
    }

    // Box
    fn print(s: Box<[u8]>) {
        println!("{:?}", s);
    }

    let byte_array = [b'h', b'e', b'l', b'l', b'o'];
    print(Box::new(byte_array));
    let byte_array = [b'w', b'o', b'r', b'l', b'd'];
    print(Box::new(byte_array));

    // if
    let number = 1;
    if 0 < number {
        println!("0 < number");
    } else if number < 0 {
        println!("number < 0");
    } else {
        println!("0 == number");
    }

    let number = 1;
    let result = if 0 <= number { number } else { -number };

    println!("result: {}", result);

    // loop
    let mut count = 0;

    let result = loop {
        println!("count: {}", count);
        count += 1;
        if count == 10 {
            break count;
        }
    };
    println!("result: {}", result);

    let mut count = 0;

    while count < 10 {
        println!("count: {}", count);
        count += 1;
    }

    let count: i32;
    for count in 0..10 {
        println!("count: {}", count);
    }

    let mut array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    for element in &array {
        println!("element: {}", element);
    }

    'main: loop {
        println!("main loop start");
        'sub: loop {
            println!("sub loop start");

            break 'main;
            println!("sub loop end");
        }
        println!("main loop end");
    }

    // match
    let i: i32 = 1;
    match i {
        1 => println!("1"),
        2 => println!("2"),
        3 => println!("3"),
        _ => println!("misc"),
    }

    enum Color {
        Red,
        Blue,
        Green,
    }

    let c = Color::Red;
    match c {
        Color::Red => println!("Red"),
        Color::Blue => println!("Blue"),
        Color::Green => println!("Green"),
    }

    let result: Result<i32, String> = Ok(100);
    // let result: Result<i32, String> = Err(String::from("hoge"));
    let result_number = match result {
        Ok(number) => number,
        Err(message) => {
            println!("Error: {}", message);
            -1
        }
    };

    // range
    for number in 1..5 {
        println!("{}", number);
    }

    // iterator
    let it = Iter {
        current: 0,
        max: 10,
    };
    for num in it {
        println!("{}", num);
    }

    struct Iter {
        current: usize,
        max: usize,
    }

    impl Iterator for Iter {
        type Item = usize;

        fn next(&mut self) -> Option<usize> {
            self.current += 1;
            if self.current - 1 < self.max {
                Some(self.current - 1)
            } else {
                None
            }
        }
    }

    let x = add(1, 2);
    println!("x = {}", x);

    fn add(a: i32, b: i32) -> i32 {
        a + b
    }

    let x = abs(-10);
    println!("x = {}", x);

    fn abs(number: i32) -> i32 {
        if number < 0 {
            return -number;
        }
        number
    }

    struct Person2 {
        name: String,
        age: u32,
    }

    // impl Person2 {
    //     fn say_name(&self) {
    //         println!("I am {}", self.name);
    //     }

    //     fn say_age(&self) {
    //         println!("I am {} year(s) old.", self.age);
    //     }
    // }

    // let p = Person2 {
    //     name: String::from("Taro"),
    //     age: 20,
    // };

    // p.say_name();
    // p.say_age();

    impl Person2 {
        fn new(name: &str, age: u32) -> Person2 {
            Person2 {
                name: String::from(name),
                age: age,
            }
        }

        fn say_name(&self) -> &Self {
            println!("I am {}.", self.name);
            self
        }

        fn say_age(&self) -> &Self {
            println!("I am {} year(s) old.", self.age);
            self
        }
    }

    let p = Person2::new("Taro", 20);
    p.say_name().say_age();

    // macro
    let s = concat!("A", "b2", 3);
    println!("s: {}", s);
    let s = format!("{}-{:?}", s, ("D", 5));
    println!("s: {}", s);
    let s = format!("{}{}", "abc", "def");
    println!("s: {}", s);

    print!("hello");
    println!("hello {}", "world");
    eprint!("hello {}", "error");
    eprintln!("hello");

    let mut w = Vec::new();
    write!(&mut w, "{}", "ABC");
    writeln!(&mut w, " is 123"); // 10は改行
    dbg!(w);

    // panic
    // panic!("it will panic");
    // thread 'main' panicked at 'it will panic', src/main.rs:318:5

    // vector
    let v = vec![1, 2, 3];

    // access to outside of program
    println!("defined in file: {}", file!());
    println!("defined on line: {}", line!());
    println!("is test: {}", cfg!(unix));
    println!("CARGO_HOME: {}", env!("CARGO_HOME"));

    // assertion
    assert!(true);
    assert_eq!(1, 1);
    assert_ne!(1, 0);
    debug_assert!(false); // cargo run --releaseで使用されない
    debug_assert_eq!(1, 1);
    debug_assert_ne!(1, 0);

    // suupport macro
    enum Emotion {
        Anger,
        Happy,
    }

    trait Emotional {
        fn get_happy(&mut self) -> String;
        fn get_anger(&mut self) -> String;
        fn tell_state(&self) -> String;
    }

    struct HappyPerson {
        name: String,
        state: Emotion,
    }

    impl Emotional for HappyPerson {
        fn get_anger(&mut self) -> String {
            unimplemented!()
        }

        fn get_happy(&mut self) -> String {
            format!("{} is always happy.", self.name)
        }

        fn tell_state(&self) -> String {
            todo!();
        }
    }

    let mut p = HappyPerson {
        name: "Takashi".to_string(),
        state: Emotion::Happy,
    };
    println!("{}", p.get_happy());

    fn f(x: usize) -> &'static str {
        match x {
            n if n * n % 3 == 0 => "3n",
            n if n * n % 3 == 1 => "3n+1 or 3n+2",
            _ => unreachable!(),
        }
    }

    println!("{}", f(10));

    // derive trait
    #[derive(Eq, PartialEq)]
    struct A(i32);

    #[derive(PartialEq, PartialOrd)]
    struct B(f32);

    #[derive(Copy, Clone)]
    struct C;

    #[derive(Clone)]
    struct D;

    #[derive(Debug)]
    struct E;

    #[derive(Default)]
    struct F;

    println!("{:?}", A(0)==A(1));

    println!("{:?}", B(1.0) > B(0.0));

    let c0 = C;
    let _c1 = c0;
    let _c2 = c0;

    let d0 = D;
    let _d1 = d0.clone();

    println!("{:?}", E);

    let _f = F::default();

}
