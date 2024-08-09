#![allow(unused_function)]
pub fn string1(){
    let my_name = "pascal";
    greet(my_name);
}

fn greet(name: String){
    println!("Hello,{}",name);
}