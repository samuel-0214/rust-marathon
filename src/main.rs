
use std::{collections::HashMap, fmt::Display, fs, sync::mpsc, thread, vec};

fn main() {
    stack_fn();
    heap_fn();
    update_string();

    let s1 = String::from("Hello");
    let s2 = s1;
    println!("{}", s2);
    let s3 = s2;
    println!("{}", s3);

    let mut s = String::from("Hello");
    s.push_str(", World!");
    println!("{}", s);
    update_str(&mut s);
    println!("{}", s);


    let user1 = User{
        active: true,
        username: String::from("suleman"),
        email: String::from("example@gmail.com"),
        sign_in_count: 1,
    };

    println!("User: {}, Email: {}, Active: {}, Sign In Count: {}",
             user1.username, user1.email, user1.active, user1.sign_in_count);

    let rect = Shape::Rectangle(1.0, 2.0);
    let a1 = calculate_area(rect);
    let circle = Shape::Cicle(1.0);
    let a2 = calculate_area(circle);
    println!("Area of rectangle:{},circle:{}",a1,a2);


    let my_string = String::from("i dont know");
    match find_first_a(&my_string){
        Some(index) => println!("The letter 'a' in string '{}' was found at {}th position",my_string,index),
        None => println!("The letter 'a' was not found in the string '{}'",my_string)
    }


    let greet_file_result = fs::read_to_string("text.txt");

    match greet_file_result{
        Ok(file_content) => {
            println!("File read successfully: {:?}",file_content);
        },
        Err(error) => {
            println!("Failed to read file {:?}",error);
        }
    }

    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    println!("{:?}",even_filter(&vec));
    println!("{:?}",vec);

    let mut vec1 = Vec::new();
    vec1.push(1);
    vec1.push(2);
    vec1.push(3);
    vec1.push(4);
    even_filter_memory_efficient(&mut vec1); 
    println!("{:?}",vec1);

    //hashmaps 
    let mut users : HashMap<String,i32> = HashMap::new();

    users.insert(String::from("saleem"), 21);
    users.insert(String::from("reiner"), 23);

    let name = users.get("harkirat");

    match name {
        Some(name ) => println!("{}",name),
        None => println!("Value not found")
    }

    let input_vec = vec![(String::from("rahul"),21),(String::from("rina"),17)];
    let hm = group_vaules_by_keys(input_vec);

    println!("{:?}",hm);

    let mut nums = vec![1,2,3,4];

    let iter = nums.iter();
    for value in iter {
        println!("Got value {}",value);
    }

    let iter_mut = nums.iter_mut();

    for value in iter_mut {
        *value = *value + 1;
    }
    println!("{:?}",nums);

    let mut iter_nums = nums.iter_mut();

    let first_number = iter_nums.next();
    let second_number = iter_nums.next();
    let third_number = iter_nums.next();
    let fourth_number = iter_nums.next();
    let fifth_number = iter_nums.next();

    println!("{:?}",first_number);
    println!("{:?}",second_number);
    println!("{:?}",third_number);
    println!("{:?}",fourth_number);
    println!("{:?}",fifth_number);

    // This following lines of code will do the exact same as the above  
    // while let Some(val) = iter_mut.next() {
    //     println!("Got values {}",val);
    // }

    let v1_iter = nums.iter();
    let v4_iter = nums.iter();

    let v2_iter = v1_iter.map(|x| x+1);
    for i in v2_iter {
        println!("{:?}",i);
    }

    println!("-------------------------");

    let v3_iter = v4_iter.filter(|x| *x % 2 == 0);
    for x in v3_iter{
        println!("{}",x)
    }

    //Write the logic to first filter all odd values then double each value and create a new vector

    let mut new = Vec::new();
    let v5_iter = nums.iter();
    let odd_values = v5_iter.filter(|x| *x % 2 == 1).map(|x| x * 2);
    for i in odd_values{
        new.push(i);
    }
    println!("{:?}",new);

    //function way
    let v1 : Vec<i32> = vec![1,2,3,4,5];
    let ans = filer_map(v1);
    println!("{:?}",ans);

    // vector -> hashmap -> iterator -> hashmap -> vector

    let new_vec= vec![(String::from("sareena"),32),(String::from("abdul"),23)];
    println!("{:?}",new_vec);
    let new_hm : HashMap<String, i32>= new_vec.into_iter().collect();

    for (key,val) in &new_hm{
        println!("{}:{}",key,val);
    }

    let back_to_vector : Vec<(String,i32)> = new_hm.iter().map(|(k,v)| (k.clone(),*v)).collect();

    println!("{:?}",back_to_vector);

    // Strings vs slices 

    //creating a string
    let mut str = String::from("Sameer");
    println!("String : {}",str);
    //appending a string 
    str.push_str(" babu");
    println!("String : {}",str);
    //deleting a string 
    str.replace_range(6..str.len(), "");
    println!("String : {}",str);


    //function that takes an input and returns the first name
    //Approach1
    let str1 = String::from("sudheer bhai");
    let ans = first_name(&str1);
    println!("{}",ans);
    // the above approach is not optimal cause we double the space cause we are creating two different string variable which takes up space in the heap 

    //Approach2
    let ans1 = first_word(&str1);
    println!("{}",ans1);

    //Generics 
    let bigger = largest(1, 2);
    let bigger_str = largest("a", "b");
    println!("{}",bigger);
    println!("{}",bigger_str);

    //Traits
    //creating a user
    let user = User1{
        name : String::from("Ved"),
        age:21,
    };
    notify(user);

    //Lifetimes
    let ans;
    let word1 = String::from("small");
    let word2 = String::from("longest");
    ans = longest(&word1, &word2);
    println!("longer word is: {}",ans);


    //Lifetimes with structs 
    let name = String::from("ved");
    let user = User2{
        name : &name
    };

    println!("{}",user.name);

    //Generic Type parameters,trait bounds and lifetimes together
    println!("{}",longest_with_an_announcement(&word1, &word2, "ved"));

    //multithreading 
    let handle0 = thread::spawn(||{
        for i in 0..5{
            println!("Printing the {i}th of spawned thread");
        }
    });

        for i in 0..5{
        println!("Printing the {i}th of main thread");
    }
    handle0.join();

    //messaging between threads 
    let (tx,rx) = mpsc::channel();

    thread::spawn(move || {
        let val = String::from("hi");
        tx.send(val).unwrap();
    });

    let received = rx.recv().unwrap();
    println!("Got : {received}");

    
    //code that finds the sum from 1-10^8
    let handle = thread::spawn(move || {
        let mut val : u128 = 0;
        for i in 0..=25000000{
            val = val + i;
        }
        val
    });
    let partialsum = handle.join().unwrap();

    let handle2 = thread::spawn(move ||{
        let mut val : u128 = 0;
        for i in 25000001..=50000000{
            val = val +i;
        }
        val
    });
    let partialsum1 = handle2.join().unwrap();

    let handle3 = thread::spawn(move || {
        let mut val : u128 = 0;
        for i in 50000001..=75000000{
            val = val +i;
        }
        val
    });
    let partialsum2 = handle3.join().unwrap();

    let handle4 = thread::spawn(move || {
        let mut val : u128 = 0;
        for i in 75000001..=100000000{
            val = val +i;
        }
        val
    });
    let partialsum3 = handle4.join().unwrap();

    let whole_sum = partialsum + partialsum1 + partialsum2 + partialsum3;

    println!("sum of 1-10^8 : {whole_sum}");


    //more robust code for finding the sum from 1 - 10^8

    let range_size = 25000000;
    let mut handles = vec![];

    for i in 0..4{
        let start = i*range_size +1;
        let end = (i + 1) *range_size;

        let handle = thread::spawn(move || {
            partial_sum(start,end)
        });

        handles.push(handle);
    }

    let total : u128 = handles.into_iter().map(|h| h.join().unwrap()).sum();

    println!("Total is : {total}");
}

pub trait Summary{
    fn summarize(&self) -> String; 
}

pub trait Fix{}

pub fn notify<T : Summary + Fix>(item : T){
    println!("This is the notify function : {}",item.summarize());
}

fn update_str(s1 : &mut String){
    s1.push_str(" World");
}

fn stack_fn(){
    let a = 10;
    let b = 20;
    let c = a + b;
    println!("Stack: a = {}, b = {}, c = {}", a, b, c);
}

fn heap_fn(){
    let a = String::from("Hello");
    let b = String::from("World");
    let combined = format!("{} {}",a,b);
    println!("Heap: a = {}, b = {}, combined = {}", a, b, combined);
}

fn update_string() {
    let mut s = String::from("Initial String");
    println!("Before update: {}", s);
    println!("Capacity:{}, Length:{}, Pointer:{:p}", s.capacity(), s.len(), s.as_ptr());
    s.push_str("and some added stuff");
    println!("Capacity:{}, Length:{}, Pointer:{:p}", s.capacity(), s.len(), s.as_ptr());
    println!("After update: {}", s);
}

fn calculate_area(shape : Shape) -> f64{
    let area = match shape {
        Shape::Rectangle(a,b ) => a * b,
        Shape::Cicle(r) => 3.14  * r * r,
    };
    return area;
}

fn find_first_a(string : &str) -> Option<i32>{
    for (index,character) in string.chars().enumerate(){
        if character == 'a'{
            return Some(index as i32);
        }
    }
    return None;
}

fn even_filter(vec: &Vec<i32>) -> Vec<i32>{
    let mut new_vec = Vec::new();

    for val in vec{
        if val % 2 == 0 {
            new_vec.push(*val);
        }
    }

    new_vec
}

fn even_filter_memory_efficient(vec : &mut Vec<i32>){
    let mut i = 0;

    while i < vec.len(){
        if vec[i] % 2 != 0 {
            vec.remove(i);
        }
        else {
            i += 1;
        }
    }
}

fn group_vaules_by_keys(vec : Vec<(String,i32)>) -> HashMap<String , i32>{
    let mut hm = HashMap::new();
    for (key,value) in vec {
        hm.insert(key, value);
    }

    return hm;
}

fn filer_map(v : Vec<i32>) -> Vec<i32>{
    let new_iter = v.iter().filter(|x| *x % 2 == 1).map(|x| x * 2);
    let new_vec : Vec<i32> = new_iter.collect();
    return new_vec;
}

fn first_name(str : &String) -> String{
    let mut ans = String::from("");
    for i in str.chars(){
        if i == ' '{
            break;
        }
        ans.push_str(&i.to_string());
    }
    return ans;
}

fn first_word(str : &String) -> &str{
    let mut space_index = 0;
    for i in str.chars(){
        if i == ' '{
            break;
        }
        space_index = space_index+1;
    }
    return &str[0..space_index];
}

fn largest <T: std::cmp::PartialOrd>(a:T,b:T) -> T{
    if a > b{
        return a;
    }
    b
}

fn longest<'a>(a:&'a str,b:&'a str) -> &'a str{
    if a.len() > b.len(){
        return a;
    }
    b
}

fn longest_with_an_announcement<'a,T>(
    x : &'a str,
    y : &'a str,
    ann : T) -> &'a str where T : Display,{
        println!("Announcement {ann}");
        if x.len() > y.len(){
            return x;
        }
        y
    }

fn partial_sum(a:u128, b:u128) -> u128{
    let mut value : u128 = 0;
    for i in a..=b{
        value = value + i;
    }
    value
}

impl Summary for User1{
    fn summarize(&self) -> String {
        return format!("The name is {},the age is {}",self.name,self.age);
    }
}

impl Fix for User1{}

struct User {
    active : bool,
    username : String,
    email : String,
    sign_in_count : u64,
}

enum Shape{
    Cicle(f64),
    Rectangle(f64,f64),
}

struct User1{
    name : String,
    age : u32
}

struct User2<'a>{
    name : &'a str,
}