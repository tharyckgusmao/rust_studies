use std::collections::HashMap;
// #[derive(Debug)]

// enum SpreadsheetCell {
//     Int(i32),
//     Float(f64),
//     Text(String),
// }
fn main() {
    //Vetores

    // let mut v = Vec::new();

    // v.push(5);
    // v.push(6);
    // v.push(7);
    // v.push(8);

    // println!("Hello, world!{:?}", v);

    // let v = vec![1, 2, 3, 4, 5];

    // let third: &i32 = &v[2];
    // let third: Option<&i32> = v.get(2);
    // println!("Hello, world!{:?}", third);

    // let row = vec![
    //     SpreadsheetCell::Int(3),
    //     SpreadsheetCell::Text(String::from("blue")),
    //     SpreadsheetCell::Float(10.12),
    // ];
    // println!("Hello, world!{:?}", row);

    // let mut s = String::new();

    // let data = "teste";
    // let s = data.to_string();

    // let s1 = String::from("Hello, ");
    // let s2 = String::from("world!");
    // let s3 = s1 + &s2; // Note que s1 foi movido aqui e não pode ser mais usado

    // println!("{:?}", s3);

    // let s1 = String::from("tic");
    // let s2 = String::from("tac");
    // let s3 = String::from("toe");

    // let s = format!("{}-{}-{}", s1, s2, s3);

    // println!("{}", s);

    // let mut scores = HashMap::new();
    // scores.insert(String::from("Blue"), 10);

    // scores.entry(String::from("Yellow")).or_insert(50);
    // scores.entry(String::from("Blue")).or_insert(50);

    //Calculate avg,mode, median

    let mut list = vec![1, 2, 1, 2, 5, 3, 4, 5];
    let mut values = HashMap::new();
    let mut sum = 0;
    let mut avg = 0;
    let mut len = list.len() / 2;
    if list.len() % 2 == 0 {
        avg = (list[(len) - 1] + list[(len) - 1]) / 2;
    } else {
        avg = list[len];
    }

    println!("{}", avg);

    for value in &list {
        sum = sum + value;
    }
    let mut numbers_count = HashMap::new();
    for num in &list {
        let count = numbers_count.entry(num).or_insert(0);
        *count += 1;
    }

    let mode = numbers_count.keys().max().unwrap();
    values.insert(String::from("Median"), sum / list.len());
    values.insert(String::from("Avg"), avg);
    values.insert(String::from("MostFrequency"), **mode);

    println!("{:?}", numbers_count);
    println!("{:?}", values);
}
