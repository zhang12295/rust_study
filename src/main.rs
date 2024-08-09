mod complex_num;
mod char_bool_unit;
mod string_study;


fn _greet_world() {
    let southren = "Grüß Gott!";
    let chinese = "世界你好";
    let english = "world,hello";
    let regions = [southren,chinese,english];

    for region in regions.iter(){
        println!("{}",&region);
    } 
}

fn main() {
//     //greet_world();
//     let penguin_data="\
//    common name,length (cm)
//    Little penguin,33
//    Yellow-eyed penguin,65
//    Fiordland penguin,60
//    Invalid,data";

//    let records = penguin_data.lines();

//    for(i,record) in records.enumerate(){
//     if i==0 || record.trim().len() == 0 {
//         continue;
//     }
//     // 声明一个 fields 变量，类型是 Vec
//      // Vec 是 vector 的缩写，是一个可伸缩的集合类型，可以认为是一个动态数组
//      // <_>表示 Vec 中的元素类型由编译器自行推断，在很多场景下，都会帮我们省却不少功夫
//     let fields: Vec<_> = record
//     .split(',')
//     .map(|field| field.trim())
//     .collect();
//     if cfg!(debug_assertions){
//         eprintln!("debug:{:?}->{:?}",
//         record,fields);
//     }

//     let name = fields[0];
// // 1. 尝试把 fields[1] 的值转换为 f32 类型的浮点数，如果成功，则把 f32 值赋给 length 变量
//      //
//      // 2. if let 是一个匹配表达式，用来从=右边的结果中，匹配出 length 的值：
//      //   1）当=右边的表达式执行成功，则会返回一个 Ok(f32) 的类型，若失败，则会返回一个 Err(e) 类型，if let 的作用就是仅匹配 Ok 也就是成功的情况，如果是错误，就直接忽略
//      //   2）同时 if let 还会做一次解构匹配，通过 Ok(length) 去匹配右边的 Ok(f32)，最终把相应的 f32 值赋给 length
//      //
//      // 3. 当然你也可以忽略成功的情况，用 if let Err(e) = fields[1].parse::<f32>() {...}匹配出错误，然后打印出来，但是没啥卵用
//     if let Ok(length) = fields[1].parse::<f32>() {
//         println!("{},{}cm",name,length)
//     }

//     assert!((0.1_f64+0.2 - 0.3).abs() <0.00001)
//    }
    
    //float_calc();
    //sqrt_calc();
    //complex_num::complex();
    //char_bool_unit::reference1();
   
   string_study::string1();
}


fn _float_calc() {
    
    let abc: (f32, f32, f32) = (0.1, 0.2, 0.3);
    let xyz: (f64, f64, f64) = (0.1, 0.2, 0.3);

    println!("abc (f32)");
    println!("   0.1 + 0.2: {:x}", (abc.0 + abc.1).to_bits());
    println!("         0.3: {:x}", (abc.2).to_bits());
    println!();

    println!("xyz (f64)");
    println!("   0.1 + 0.2: {:x}", (xyz.0 + xyz.1).to_bits());
    println!("         0.3: {:x}", (xyz.2).to_bits());
    println!();

    assert!(abc.0 + abc.1 == abc.2);
    assert!(xyz.0 + xyz.1 == xyz.2);
}


fn _sqrt_calc(){
    let x = (-42.0_f32).sqrt();
    if x.is_nan(){
        println!("undefined")
    }
}
