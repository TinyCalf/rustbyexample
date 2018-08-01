 #[derive(Debug)]
 struct Structure(i32);

 #[derive(Debug)]
 struct Deep(Structure);

 fn main() {
     println!("Hello World!");
     println!("I am a Rustacean!");


     println!("{} days", 31);

     println!("{0}, this is {1}. {1}, this is {0}", "Alice", "Bob");

     println!("{} of {:b} people know binary,  the other half doesn't", 1, 2);

     println!("{number:>width$}", number = 1, width = 6);

     println!("{number:>0width$}", number = 1, width = 6);

     println!("My name is {0}, {1} {0}", "Bond", "James");

    
     //println!("This sturct `{}` won't print...", Structure(3));

     let pi = 3.1415926;
     let formatted_pi = format!("{:.*}", 2, pi);
     println!("Pi is roughly {}", formatted_pi);
     println!("{:#?} months in a year. " , 12);
     println!("{1:#?} {0:?} is the {actor:?} name.",
              "Slater",
              "Christian",
              actor="actor's");
     println!("Now {:#?} will print!" , Structure(3));
     println!("Now {:#?} will print!", Deep(Structure(7)));

 }
