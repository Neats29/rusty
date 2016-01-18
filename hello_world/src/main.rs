fn main() {
    println!("Hello, world!");
}

//main is the begining of every rust program. 
//4 spaces not tabs
//println!(), ! means that we are calling a *macro* instead of a normal function. 
//"Hello, world!" is a *statically allocated* string
//in rust most things are expressions rather than statements

//rustc <filename.rs> we *compile rust*, this creates a *binary executable* file under <filename>
//then ./filename *runs* the file
