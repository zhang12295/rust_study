pub fn _char_use(){
    let x = '中';
    println!("字符'中'占用了{}字节的内存大小",std::mem::size_of_val(&x));
}


pub fn _reference() {
    let x = 5;
    let y = &x;
    assert_eq!(5,x);
    assert_eq!(5,*y);
}

pub fn _reference1(){
    let mut s1 = String::from("hello");

    change(&mut s1);
    let len = calculate_len(&s1);
    

    println!("The length of '{}' is {}.", s1, len);
}


fn calculate_len(s: &String) -> usize{
    s.len()
}

fn change(s:&mut String){
    s.push_str(",world");
}


fn dangle() -> String{
    let s = String::from("hello");
    s
}