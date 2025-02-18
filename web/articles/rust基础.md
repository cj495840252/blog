# Rust

[IdeaWand](https://blog.ideawand.com/)

## Base command

###### Install and view version

```shell
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
rustc --version
```

###### Open office documents

```shell
rustup doc
```

###### Compile source

```shell
rustc main.py
```

## Cargo

> Project manage tool

###### create project

```shell
cargo new `project_name`
```

###### build project

```shell
# when finished, a lock file is created
cargo build

# compile➕excute
cargo run

#build for release
cargo build --release
```

###### check code

```shell
# it does not produce binary file, only for debugging
cargo check
```

## Base operate

### Base

###### variable

```rust
let mut a = String::new()
```

###### read from command line

```rust
use std::io;
fn main() {
    let mut a = String::new();
    // read_line返回一个Result的对象, exprect是他的方法，成功了返回读取到的信息，否者返回err msg
    io::stdin().read_line(&mut a).expect("cannot read");//
    println!("{}", a);
}
```

### 使用第三方库的命令

```shell
cargo install sqlx-cil # 这里可以换成二进制crate
cargo sqlx prepare
```



###### rand number

```rust
use rand::Rng;
fn main() {
    //左闭右开
    let n = rand::thread_rng().gen_range(1, 100);
    println!("{}",n)
}
```

###### match function

```rust

use rand::Rng;
use std::cmp::Ordering;
fn main() {
    let mut n = rand::thread_rng().gen_range(1, 100);
    println!("随机数已生成，请猜测！！！");
    loop {
        let mut inp: String = String::new();

        std::io::stdin().read_line(&mut inp).expect("???????");
        //类型转换
        // let inp:u32 = inp.trim().parse().expect("not integer");

        let mut inp:u32 = match inp.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        //match方法
        match n.cmp(&inp) {
            Ordering::Less => { println!("small");continue; },
            Ordering::Equal => { println!("equal") ;break;},
            Ordering::Greater => {
                println!("big");
                continue;
            }
        }
    }
}

```

### Struct

- Self代表一个类型，self代表实例化后的对象

```rust
#[derive(Debug)]
struct User{
    name: String,
    age: i32,
    sex: Sex
}
#[derive(Debug)]
enum Sex{
    男(String),
    女
}
impl Sex {
    //类方法
    fn area_rect(&self){
    }
    // 关联函数
}
impl Rectangle {
    //类方法
    fn area_rect(&self) -> u32 {
        return self.width * self.length;
    }
    // 关联函数
}
fn main(){
    let user = User{name:"zack".to_string(),age:24,sex:Sex::男("💪".to_string())};
    println!("{:?}",user);
    let rec = Rectangle{width:30,length:50};
    println!("inner func: {}",rec.area_rect());
}


```

### Enum

```rust
fn main() {

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("{:?}", list_of_statuses)

}
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}
```



### null

rust中没有null值，它由enum实现

```rust
fn main{
  let some_number = Some(5);
  let som_string = Some("???");
  let absent_number:Option<i32> = None;
}
```


### Vector

```sql
use crate::Cell::Text;

fn  main() {
    // 1.创建
    let mut v:Vec<i32> = Vec::new();
    // 快捷创建
    let mut v2 = vec![1,2,3];

    // 2.添加元素
    v.push(14);
    v2.push(2);

    // 3.read
    match v.get(0) {
        Some(a) => println!("{}",a),
        None => println!("None"),
    }

    println!("{}",&v2[1]);
    println!("{}",&v2[1]);

    // 不能同借用一个元素为可变和不可变不影响
    let mut v3 = vec![1,2,3,4,5];
    let first = &v3[0];
    // v3.push(2);
    println!("{}",first);

    // 遍历
    for i in &v3 {
        println!("i={}",*i);
    }
    for i in &mut v3{
        *i = *i+1; // &i32 cannot plus i32
    }
    println!("{:?}",&v3);

    // 使用Vec存放不同类型
    let v4 = vec![Cell::Int(1),Cell::Float(2.4),Text("zack".to_string())];
    println!("{:?}",v4)
}
#[derive(Debug)]
enum Cell {
    Int(i32),
    Float(f64),
    Text(String)
}
```

### String

string是一个对象存放在heap中，*string 相当于一个指针存放在stack中

str是一个切片，存放在编译后的binaryfile中或者在heap中，&str是指向这个切片的指针，存放在stack中

\*String会被自动转换成\*str, 但是\*str不能被自动转换成\*String

```rust
fn  main() {
    let mut s1 = String::from("aaa");
    let s2 = String::from("bbb");
    s1.push_str(&s2);
    println!("{}", s1);
    //println!("{}", s1 + &s2);// 后面的必须是一个string的引用，切第一个所有权转移
    // println!("{}",s1)已经借走了

    let s3 = String::from("ccc");
    let s4 = format!("{}-{}-{}",s1, s2, s3);//这个宏和print不会取得任何所有权
    println!("{}",s4);

    let arr = ["a","b","c"];
    let new = arr.join("_");//arr 没有被借走
    println!("{:?}",new);

    func(&"avasd".to_string());

    //
    let s5 = "👎💰".to_string();
    // println!(s5[1])//不能用下标索引

    // 需要用下列方法
    for i in s5.chars() {
        println!("{}",i)
    }
    for i in s5.bytes(){
        println!("{}",i)
    }
    // 字形簇，需要其他库实现


    let s6 = "a👎7⃣️1";
    //这个切分要按照字符边界切分，字母1字节，比如这个切出a
    println!("{}",&s6[0..1]);
    println!("{}",&s6[1..5]);// 👎4字节，&s6[0..3]错误等等
    println!("==> {}", &s6[5..12]);// 这个数字表情占7字节
    println!("==>{}", &s6[12..13]); //数字1，1字节
    println!("{}",&s6.len())

}

fn func(s:&String){
    println!("{s}")
}
```

### HashMap

```rust
use std::collections::HashMap;

fn main() {
    // hashmap是同构的，所有的k是同一种类型，所有的value是同一种类型
    let mut scores = HashMap::new();
    // 若将引用传入，则必须保证HashMap存在时，这些值是有效的
    scores.insert("key".to_string(),10);
    println!("{:?}",&scores);

    // collect 方法
    let teams = vec!["blue".to_string(),"yellow".to_string()];
    let inital_scores = vec![10,59];
    let scores:HashMap<_,_> = teams.iter().zip(inital_scores.iter()).collect();
    println!("{:?}",scores);

    let s = scores.get(&"blew".to_string());
    match s {
        Some(a)=> println!("{}",a),
        None=>println!("None"),
    };

    // 遍历
    for (k,v) in &scores {
        println!("{}:{}",k,v)
    }

    //更新数据
    // 1.替换
    // 2.存在则忽略
    // 3.存在更新
    let mut scores1 = HashMap::new();
    scores1.insert("str".to_string(),10);
    scores1.insert("str".to_string(),11);
    println!("{:?}",scores1);

    scores1.entry("str".to_string()).or_insert(23);
    scores1.entry("str2".to_string()).or_insert(30);
    println!("{:?}",scores1);

    //wordcount
    let mut map = HashMap::new();
    for word in "hello world wonderful world".split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count+=1;
    }
    println!("{:?}",map)
}
```

### panic

```rust
panic!("这个宏相当于print Exception")
```



当程序发生异常时，有两种结束程序的方式，

1. 程序展开stack，遇见函数或者数据就会清理掉，工作量比较大，编译出来的文件也比较大
2. 直接停止程序，内存由os进行清理

**展开改为终止**:

- 在cargo.toml的profile部分设置panic='abort'

```toml
[profile.release]
panic = 'about'
```



### Result

Result是一个枚举,通常和match一起使用，如下面open file，panic处理

```rust
enum Result<T,E> {
  Ok(T),
  Err(E),
}

// use std::collections::HashMap;

use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");
    match f {
        Ok(file)=> file,
        // Error是一个对象
        // Err(err) => panic!("Error opening file {:?}",err),
        Err(err) => match err.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Error creat file:{:?}",e)
            }
            // 这里的错误名随便取的
            other => panic!("Error open file:{:?}",other)
        }
    };

  // 这一句是match表达式的简写
  // 1.如果Result的结果是Ok，返回Ok里面的值
  // 2.如果Result的结果是Err，调用panic!
  let f = File::open("hello.txt").unwrap();
  // 建议用这个，方便排错
  let f2 = File::open("hello.txt").expect("自定义的错误信息,后面会带上Err的信息");
}



// 最简洁的写法
//读取文件的内容，返回一个string
fn read_file(path:&str) -> Result<String, io::Error>{
    // 这两种写法是一样的，上面简写
    let mut s = String::new();
    File::open(path)?.read_to_string(&mut s)?;
    Ok(s)
}
```

问号简化

```rust
// use std::collections::HashMap;

use std::fs::File;
use std::io;
use std::io::Read;
// use std::io::ErrorKind;

fn main() {
    let s = read_file("test.txt").expect("读取失败");
    println!("{:?}",s)
}

//读取文件的内容，返回一个string
fn read_file(path:&str) -> Result<String, io::Error>{
    // 这两种写法是一样的，上面简写
    let mut f1 = File::open(path)?;
    let mut f2 = match File::open(path) {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();
    f1.read_to_string(&mut s)?;
    Ok(s)
}
```

### 范型

使用范型需要先声明

```rust
//范型方法
fn test<T>(param: &[T]){};


// 范型结构体
struct Point<T,U> {
  x: T,
  y: T,
  z: U,
}
// 范型结构体方法
impl <T,U> Point<T,U> {

}
//范型枚举
enum Option<T> {
  Some(T),
  None,
}
```



### Trait

#### 基本用法

```rust
fn main() {
    let zack = Tweet{username:"zack".to_string(),said: "i am realy handsome".to_string()};
    let res_string = zack.summarize();
    println!("{}",res_string)
}
//定义一个trait，里面一个方法，若方法有实现，则为默认方法
pub trait Summary {
    fn summarize(&self) -> String;
}
//两个结构体
pub struct NewsArticle {
    pub  content: String,
}

pub  struct Tweet {
    pub username: String,
    pub said: String,
}

//为两个结构体实现Summary这个trait
impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        todo!()
    }
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}",&self.username,&self.said)
    }
}

// trait作为传参
// 简单情况,复杂情况的一个语法糖,两个trait用➕连接
pub fn notify(item: impl Summary+std::fmt::Display)) {
    println!("{}",item.summarize())
}
//复杂情况
pub fn notify2<T:Summary+std::fmt::Display>(item: T) {
    println!("{}",item.summarize())
}

// where子句
pub fn notify3<T, U>(item: T, item2: U) -> String
where T:Summary+std::fmt::Display,
      U: std::fmt::Debug+std::fmt::Octal
{
    format!("{}",item.summarize())
}
```

#### 动态用法

- trait中方法不能返回Self
- 方法中不包含任何范型类型参数

```rust
pub trait Draw {
    fn draw(&self);
}
pub struct Screen {
  // 这里的Draw是个trait，Box是个指针，dyn代表这里是动态的，大小会在运行时确定
    pub components: Vec<Box<dyn Draw>>,
}
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // code to actually draw a button
    }
}

use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
```



### lifetime

```rust

fn main() {
    let s1 = "abcd".to_string();
    let s2 = "xyz";
    let res = longest(s1.as_str(),s2);

    println!("{}", res)

}

fn longest<'a>(x:&'a str, y: &'a str) -> &'a str {
    if x.len()>=y.len() {
        return x;
    }else {
        return y
    }
}
```



###  test

```shell
# unit test
cargo test

# unit test specify function
cargo test test_func_name

# integration test
cargo test --test test_file
```





```rust
#[cfg(test)]
mod test;
//声明一个mod做测试用，编译时不编译



// 两个test方法，
#[test]
#[ignore] // add this macros,when run : cargo test -- --ignored it will not run
fn test(){
    println!("today")
}

#[test]
#[should_panic(expected="denominator is zero")]
fn test2(){

    chufa()
}

fn chufa(){
    let mut a = 1;
    let mut b = 0;
    if b==0{
        panic!("denominator is zero")
    }
    let c = a/b;

}

#[test]
fn test3(){
    result_test();
}
fn result_test() -> Result<(), String>{
    if 1 == 2{
        return Ok(())
    }
    return Err("error code 1".to_string())
}
```



使用Result

```rust
#[test]
fn test3() ->Result<(),String>{
    match result_test() {
        Ok(t) => Ok(t),
        Err(e) => Err(e)
    }
}
fn result_test() -> Result<(), String>{
    if 2+3 == 4 {
        Ok(())
    }else {
        Err(String::from("error code 1"))
    }
}

```



### doc-comment

```shell
cargo doc --no-deps
```



### closure

```rust
// sum is colsure function name
let sum = |param1, param2, ... |{
  return param1+param2.....
}

println!("{:?}",sum(2,3))


// instance
fn main() {
    // 可以省略返回类型和参数类型,它会在第一次调用时，绑定类型
    let max = |&a,&b| -> i32{
        if a>b {
            a
        }else {
            b
        }
    };

    let mut a = 3;
    let mut b = 5;
    let max_v = max(&a, &b);
    println!("max = {}",max_v)
}
```

**采用结构体**

```rust
#![allow(unused)]
fn main() {
    // 可以省略返回类型和参数类型, 它会在第一次调用时，绑定类型
    let max = |a:String,b:String| -> String{
        if a.len() >b.len() {
            a
        }else {
            b
        }
    };

    let mut a = String::from("aa");
    let mut b = String::from("aabb");
    let max_v = Cacher::new(max).value(a,b);
    println!("max = {}",max_v);

}


struct Cacher<T>
where T:Fn(String, String) -> String, {
    max: T,
    value: Option<String>
}

impl <T> Cacher<T>
where T: Fn(String,String) -> String{
    fn new(max: T) -> Cacher<T>{
        return Cacher{
            max,
            value: None,
        }
    }

    fn value(mut self, arg1: String, arg2: String) -> String {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.max)(arg1, arg2);
                // self.value = Some(String::from(&v));
                self.value = Some(v.clone());
                return v;
            }
        }
    }
}
```

### iterator

能指定种子的随机生成器

```rust
#[allow(unused)]
use std::fmt::Debug;
use std::iter::Sum;
use rand::prelude::*;
use rand::prelude::StdRng;

fn main() {

    let mut  rng = RandIterator::new(14,10,20, 3);
    // println!("{}",rng.next().unwrap());
    // println!("{}",rng.next().unwrap());
    // println!("{}",rng.next().unwrap());

    // rng已经读取了三次，那么第三次应该为0.若去掉三次next，那么结果为上面的next之和
    println!("{}",rng.sum::<i32>());
}


// 随机数生成的迭代器,指定随机种子
struct RandIterator {
    low: i32,
    high: i32,
    times:i32,
    rng : StdRng
}
impl RandIterator{
    fn new(seed_value: u64, low:i32, high:i32, times:i32) -> RandIterator {
        let  rng = StdRng::seed_from_u64(seed_value);
        return RandIterator{low, high, times, rng};
    }
}

impl Iterator for RandIterator{
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.times -= 1;
        if self.times >= 0 {
            // let mut rng = s::seed_from_u64(self.seed_value);
            return Some(self.rng.gen_range(self.low,self.high));
        }else {
            return None
        }

    }

    fn sum<S>(self) -> S where Self: Sized, S: Sum<Self::Item> {
        return Sum::sum(self);
    }
}
```



### 智能指针

#### Box

	- 在编译时，某类型的大小无法确定，但使用该类型时，上下文却需要知道它的确切大小
	- 当有大量数据，想移交所有权，但需要确保在操作时数据不会被复制
	- 使用某个值时，你只是关心它是否实现了特定的trait，而不是关心它的具体类型

```rust
let b = Box::new(5) // 将会存在堆上
```

#### 解引用

```rust
use std::ops::Deref;

fn main() {
    let mut x = MyBox::new(5);
    assert_eq!(5, *x);
  	assert_eq!(5, *(x.deref()));

}

// 元组类型的结构体用()
struct MyBox<T> (T);

impl<T> MyBox<T>{
    fn new(x: T) -> MyBox<T>{
      return MyBox(x)
    }
}

impl<T> Deref for MyBox<T> {
    type Target = T;
  	// deref方法返回一个引用
    fn deref(&self) -> &Self::Target {
        // todo!()
        return &self.0
    }

}
```

#### Deref重载

可以使用DerefMut 这个trait来重载Deref

#### Drop

一个对象实现Drop trait时，不能显式调用drop，可以使用drop function

```rust
fn main() {
    let zack = Person{name:"zack".to_string(),age:24};
    // 运行后会自动调用drop，打印输出
    // drop(zack)
}

struct Person{
    name: String,
    age: usize
}
impl Drop for Person{
    fn drop(&mut self) {
        println!("drop function called...")
    }
}
```

#### Rc

`Rc<T>` 引用计数指针,

- 一个值有多个引用
- 只能用于单线程
- Rc::clone浅拷贝，T.clone深拷贝

```rust
use crate::Node::{Cons,Nil};
use std::rc::Rc;

fn main() {
    let a = Rc::new(
        Cons(
            5,Rc::new(
                Cons(10,Rc::new(Nil))
            )
        )
    );
    let b = Cons(3,Rc::clone(&a));
    println!("{}",Rc::strong_count(&a));// 2

    let c = Cons(4,Rc::clone(&a));
    println!("{}",Rc::strong_count(&a)) // 3


}
enum Node{
    Cons(i32, Rc<Node>),
    Nil
}
```



#### RefCell



|                  | Box`<T>`                     | Rc`<T>`                | RefCell`<T>`                 |
| ---------------- | ---------------------------- | ---------------------- | ---------------------------- |
| 同一数据所有者   | 一个                         | 多个                   | 一个                         |
| 可变性，借用检查 | 可变，不可变借用，编译时检查 | 不可变借用，编译时检查 | 可变，不可变借用，运行时检查 |

- `Refcell<T>`本身不可变，但仍能修改其中的数据

```rust
pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
    where
        T: Messenger,
{
    pub fn new(messenger: &'a T, max: usize) -> LimitTracker<'a, T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_of_max >= 0.9 {
            self.messenger
                .send("Urgent warning: You've used up over 90% of your quota!");
        } else if percentage_of_max >= 0.75 {
            self.messenger
                .send("Warning: You've used up over 75% of your quota!");
        }
    }
}

use std::cell::RefCell;
// use crate::::*;

fn main() {
    let mock_messenger = MockMessenger::new();
    let mut limit_tracker = LimitTracker::new(&mock_messenger, 100);

    limit_tracker.set_value(80);
		//上面的可变借用RefCell，现在只需要读操作用borrow，要修改用borrow_mut
    assert_eq!(mock_messenger.sent_messages.borrow().len(), 1);
}

struct MockMessenger {
    sent_messages: RefCell<Vec<String>>,// 定义这个变量的借用是可变的
}

impl MockMessenger {
    fn new() -> MockMessenger {
        MockMessenger {
            sent_messages: RefCell::new(vec![]),
        }
    }
}

impl Messenger for MockMessenger {
    fn send(&self, message: &str) {
      // 当使用上面的可变借用修改值的时候，这样
        self.sent_messages.borrow_mut().push(String::from(message));
    }
}

```



### thread

主线程结束后，子线程也会挂掉

spawn方法返回的是一个JoinHandle，它提供join方法阻塞线程，直到子线程完成

```rust
use std::thread;
use std::time::Duration;

fn main() {
    thread::spawn(|| {
        for i in 1..10 {
            println!("hi number {} from the spawned thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread!", i);
        thread::sleep(Duration::from_millis(1));
    }
}

// output ----------------------
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```

### Channel

recv每次只能接受到一个，并且会阻塞线程，直到获取一个值

try_recv: 不阻塞

```rust
use std::sync::mpsc;
use std::thread;
fn main() {
    let (tx, rx) = mpsc::channel();
    // let tx1 = tx.clone(); 和下面一句等价的
    let tx1 = mpsc::Sender::clone(&tx);
    thread::spawn(move || {
        let val1 = String::from("hi");
        tx.send(val1).unwrap();
        let val2 = String::from("world");

        tx.send(val2).unwrap();
    });
    let received1 = rx.recv().unwrap();
    println!("Got:{}", received1);
    let received2 = rx.recv().unwrap();
    println!("Got:{}", received2);
}
```

### 内存共享

```rust
use std::sync::{Arc, Mutex};
use std::thread;


fn main() {
    // Mutex是一个指针，指向内存中的一块地址，传入一个范型
    // Arc是一个智能指针，会自动deref,,提供clone方法可以让多个Arc指针指向一个地址
    // 这俩一起用，就可以让多个指针指向同一块地址，并且修改它的值
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter.lock().unwrap();

            *num += 1;
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Result: {}", *counter.lock().unwrap());
}
```



### 对象设计

```rust
pub mod blog{
    pub struct Post {
        state: Option<Box<dyn State>>,
        content: String,
    }

    impl Post {
        pub fn new() -> Post {
            Post {
                state: Some(Box::new(Draft {})),
                content: String::new(),
            }
        }
        pub fn add_text(&mut self, text: &str) {
            self.content.push_str(text);
        }

        pub fn request_review(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.request_review())
            }
        }
        pub fn approve(&mut self) {
            if let Some(s) = self.state.take() {
                self.state = Some(s.approve())
            }
        }
        pub fn content(&self) -> &str {
            self.state.as_ref().unwrap().content(self)
        }
    }

    trait State {
        fn request_review(self: Box<Self>) -> Box<dyn State>;
        fn approve(self: Box<Self>) -> Box<dyn State>;
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            ""
        }
    }

    struct Draft {}

    impl State for Draft {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            Box::new(PendingReview {})
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }
    }


    struct PendingReview {}

    impl State for PendingReview {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }
        fn approve(self: Box<Self>) -> Box<dyn State> {
            Box::new(Published {})
        }
    }

    struct Published {}

    impl State for Published {
        fn request_review(self: Box<Self>) -> Box<dyn State> {
            self
        }

        fn approve(self: Box<Self>) -> Box<dyn State> {
            self
        }

        // 这里的self代表trait，所以上面的content方法需要传入self给post获取content
        fn content<'a>(&self, _post: &'a Post) -> &'a str {
            &_post.content
        }
    }


}
// separator line--------------------------
use mylib::blog::Post;
fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
}
```

### pattern

#### 1. _

```rust
// 匹配剩下所以类型
let x = 1;
match x {
    1 => println!("one"),
    2 => println!("two"),
    3 => println!("three"),
    _ => println!("anything"),
}
```

```rust
// 被命名的变量，被覆盖
let x = Some(5);
let y = 10;

match x {
    Some(50) => println!("Got 50"),
    Some(y) => println!("Matched, y = {y}"),
    _ => println!("Default case, x = {:?}", x),
}

println!("at the end: x = {:?}, y = {y}", x);
```

#### 2. |

```rust
let x = 1;

match x {
    1 | 2 => println!("one or two"),
    3 => println!("three"),
    _ => println!("anything"),
}

```

#### 3. ..=

1..=5 代表一个范围的值，左右闭合

```rust
let x = 5;
match x {
    1..=5 => println!("one through five"),
    _ => println!("something else"),
}
// ---------------------------------------------------
let x = 'c';
match x {
    'a'..='j' => println!("early ASCII letter"),
    'k'..='z' => println!("late ASCII letter"),
    _ => println!("something else"),
}
```

#### 4. Destructing Structs

定义a，b变量，使它的值等于结构体中的字段

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

```rust
// 命名相同时的简写
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x, y } = p;
    assert_eq!(0, x);
    assert_eq!(7, y);
}
```

```rust
fn main() {
    let p = Point { x: 0, y: 7 };

    match p {
        Point { x, y: 0 } => println!("On the x axis at {x}"),
        Point { x: 0, y } => println!("On the y axis at {y}"),
        Point { x, y } => {
            println!("On neither axis: ({x}, {y})");
        }
    }
}
```

#### 5.Destructing Enums

```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let msg = Message::ChangeColor(0, 160, 255);
    match msg {
        Message::Quit => {
            println!("The Quit variant has no data to destructure.");
        }
        Message::Move { x, y } => {
            println!("Move in the x direction {x} and in the y direction {y}");
        }
        Message::Write(text) => {
            println!("Text message: {text}");
        }
        Message::ChangeColor(r, g, b) => {
            println!("Change the color to red {r}, green {g}, and blue {b}",)
        }
    }
}
```

#### 6. ignore

- `..`两个省略号省略多个
- `_`一个下划线省略一个

```rust
let numbers = (2, 4, 8, 16, 32);

match numbers {
    (first, .., last) => {
        println!("Some numbers: {first}, {last}");
    }
}
```

```rust
struct Point {
    x: i32,
    y: i32,
    z: i32,
}

let origin = Point { x: 0, y: 0, z: 0 };

match origin {
    Point { x, .. } => println!("x is {}", x),
}
```



## UnSafe

### 解引用原始指针

- 原始指针

  1. 可变的：`*mut T`
  2. 不可变的：`*const T`指针在解引用后不能对其直接复制

  **这里的*不是解引用符号，是类型名的一部分**

- 与引用不同，原始指针
  1. 允许通过同时具有不可变和可变指针或多个指向同一位置的可变指针来忽略借用规则
  2. 无法保证能指向合理的内存
  3. 允许为null
  4. 不实现任何清理
- 放弃保证的安全，换取更好的性能/与其他语言或硬件接口的能力

```rust
let mut num = 5;

// 有效的原始指针
let r1 = &num as *const i32;
let r2 = &mut num as *mut i32;

unsafe {
    *r2 = *r2 + 1;
    println!("{:?}", *r2)
}

//
let address = 0x123456usize;
let r3 = address as *const i32;
unsafe {println!("r3={:?}",r3)}
```

```rust
fn main() {
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];
    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

}
// 这个方法用于将一个切片分成两个，用到了unsafe
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    assert!(mid <= len);
    // error[E0499]: cannot borrow `*values` as mutable more than once at a time
    (&mut values[..mid], &mut values[mid..])
}

use std::slice;
// 用unsaf
fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = values.len();
    let ptr = values.as_mut_ptr();
    assert!(mid <= len);
    unsafe {
        (
            slice::from_raw_parts_mut(ptr, mid),
            slice::from_raw_parts_mut(ptr.add(mid), len - mid),
        )
    }
}
```



## trait

### 运算符重载

```rust
use std::ops::Add;

// #[derive(Debug)]
struct Millimeters(u32);
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let a = Millimeters(100);// 100 ms
    let b = Meters(1);// 1 m
    print!("{:?}", (a + b).0)
}
```

### 关联类型

与范型不同的是，范型可以为一个对象实现多次，而关联类型只能实现一次

```rust
struct Counter {
    count: u32,
}

impl Counter {
    fn new() -> Counter {
        Counter { count: 0 }
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        // --snip--
        self.count+=1;
        Some(self.count+1)
    }
}
// 范型-----------------------------------------------
trait fly<T> {
    fn fly();
}

impl fly<i32> for Counter {
    fn fly() {
        todo!()
    }
}
impl fly<u32> for Counter{
    fn fly() {
        todo!()
    }
}

impl<T> fly<T> for Counter{
    fn fly() {
        println!("hello world");
    }
}

```

### 完全限定语法

- 调用同名方法时使用

```rust
trait Animal {
    fn baby_name() -> String;
}

struct Dog;

impl Dog {
    fn baby_name() -> String {
        String::from("Spot")
    }
}

impl Animal for Dog {
    fn baby_name() -> String {
        String::from("puppy")
    }
}

fn main() {
    println!("A baby dog is called a {}", Dog::baby_name());
    // println!("{}", Animal::baby_name());// raise error
    println!("{}", <Dog as Animal>::baby_name())
}
```

### 继承

只有trait或者类型(Struct)定义在本地时才能为该类型实现这个trait

简而言之，不要去修改别人的代码，比如不能为Vec实现Display，因为都不是自己写的

```rust
use std::fmt;
use std::fmt::{Display, Formatter};

trait TestTrait: fmt::Display{
    fn test(&self){
        let output = self.to_string();
        println!("{}", output);
    }
}
struct Point(i32,i32);

impl Display for Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"x:{},y:{}", &self.0,&self.1)
    }
}

impl TestTrait for Point{

}

fn main() {
    let p1 = Point(1,2);
    p1.test()
}
```

### 函数指针

```rust
fn main() {

    let list_of_statuses: Vec<Status> = (0u32..20).map(Status::Value).collect();

    println!("{:?}", list_of_statuses)

}
#[derive(Debug)]
enum Status {
    Value(u32),
    Stop,
}
```

## macro宏

1. 本质上来讲，宏是用来编写可以生成其他代码的代码（元编程，metaprogramming）

2. 宏可以处理可变个数的参数
3. 编译器会在解释代码前展开宏
4. 某个文件调用宏时，必须提前定义宏或将宏导入当前作用域

- 使用macro_rules!构建的声明宏

- 3种过程宏

  - 自定义#[derive]宏，用于struct或enum，可以为其指定随derive属性添加的代码
  - 类似属性的宏，在任何条目上添加自定义属性
  - 类似函数的宏，看起来像函数调用，对其指定为参数的token进行操作

  ```rust
  ```





## web

1. 监听TCP

```rust
let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
for stream in listener.incoming() {
		handle(stream)// stream是每次发过来的消息
}
```

2.



## mysql

```rust
use chrono::NaiveDateTime;
use sqlx::mysql::MySqlPoolOptions;
use std::{env, io};
use dotenvy::from_filename;

#[derive(Debug)]
pub struct Course{
    pub id: i32,
    pub teacher_id: i32,
    pub name: String,
    pub time: Option<NaiveDateTime>,
}


#[actix_rt::main]
async fn main() -> io::Result<()>{
    from_filename("/Users/zackjchen/Desktop/RustProject/ws/db/resource/.env").ok();
    //如果项目根目录下有.env文件，则直接用这个,否者cargo sqlx prepare生成.sqlx
    // dotenvy::dotenv().ok();
    // env::set_var("DATABASE_URL", "mysql://root:root@localhost:3306/test");//这种直接设置的方法没有用
    let url = env::var("DATABASE_URL")
        .expect("DATABASE_URL not in .env configuration file");
    println!("{}",url);
    let db_pool = MySqlPoolOptions::new()
        .connect(&url).await.unwrap();

    let course_rows = sqlx::query!(r#"select * from course;"#)
        .fetch_all(&db_pool)
        .await.unwrap();

    let mut courses_list = vec![];
    for row in course_rows {
        courses_list.push(Course{
            id: row.id as i32,
            teacher_id: row.teacher_id,
            name: row.name,
            time: Some(chrono::NaiveDateTime::from(row.time.unwrap().naive_utc()))
        })
    }

    println!("Courses={:?}", courses_list[0]);
    Ok(())
}

```

## async

async返回一个Future，它需要一个执行者去执行

https://rust-lang.github.io/async-book/02_execution/02_future.html

```rust
use futures::executor::block_on;//执行者，会阻塞Future直到完成
fn main() {
    // block_on(lean_song());
    // block_on(sing_song(1));
    // block_on(dance());
    block_on(async_main())
}

async fn lean_song() -> i32{
    println!("lean song");
    return 1;
}


async fn sing_song(flag: i32){
    if flag==1 {
        println!("sing song")
    }
}

async fn dance(){
    println!("dance")
}

// -----------
async fn lean_and_sing(){
    let song = lean_song().await;
    sing_song(song).await
}

async fn async_main(){
    let f1 = lean_and_sing();
    let f2 = dance();
    futures::join!(f1,f2);
}
```
