use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file_path);
    fs::remove_file(path).unwrap();
    println!("File {} has been deleted", file_path);
}

fn list_files(directory_path: &str) {
    let path = Path::new(directory_path);
    let entries = fs::read_dir(path).unwrap();

    println!("Files in directory {}:", directory_path);
    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.

assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}

use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}

        use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file_path);
    fs::remove_file(path).unwrap();
    println!("File {} has been deleted", file_path);
}

fn list_files(directory_path: &str) {
    let path = Path::new(directory_path);
    let entries = fs::read_dir(path).unwrap();

    println!("Files in directory {}:", directory_path);
    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.

assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}

use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file_path);
    fs::remove_file(path).unwrap();
    println!("File {} has been deleted", file_path);
}

fn list_files(directory_path: &str) {
    let path = Path::new(directory_path);
    let entries = fs::read_dir(path).unwrap();

    println!("Files in directory {}:", directory_path);
    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.

assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}

use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file_path);
    fs::remove_file(path).unwrap();
    println!("File {} has been deleted", file_path);
}

fn list_files(directory_path: &str) {
    let path = Path::new(directory_path);
    let entries = fs::read_dir(path).unwrap();

    println!("Files in directory {}:", directory_path);
    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.

assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}

use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file_path);
    fs::remove_file(path).unwrap();
    println!("File {} has been deleted", file_path);
}

fn list_files(directory_path: &str) {
    let path = Path::new(directory_path);
    let entries = fs::read_dir(path).unwrap();

    println!("Files in directory {}:", directory_path);
    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.

assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}

use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file_path);
    fs::remove_file(path).unwrap();
    println!("File {} has been deleted", file_path);
}

fn list_files(directory_path: &str) {
    let path = Path::new(directory_path);
    let entries = fs::read_dir(path).unwrap();

    println!("Files in directory {}:", directory_path);
    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.

assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}

use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}
use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file_path);
    fs::remove_file(path).unwrap();
    println!("File {} has been deleted", file_path);
}

fn list_files(directory_path: &str) {
    let path = Path::new(directory_path);
    let entries = fs::read_dir(path).unwrap();

    println!("Files in directory {}:", directory_path);
    for entry in entries {
        let entry = entry.unwrap();
        let file_name = entry.file_name();
        println!("{}", file_name.to_string_lossy());
    }
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.

assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}

use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}


use chrono::prelude::*;
use chrono::offset::LocalResult;

let dt = Utc.with_ymd_and_hms(2014, 7, 8, 9, 10, 11).unwrap(); // `2014-07-08T09:10:11Z`
// July 8 is 188th day of the year 2014 (`o` for "ordinal")
assert_eq!(dt, Utc.yo(2014, 189).and_hms_opt(9, 10, 11).unwrap());
// July 8 is Tuesday in ISO week 28 of the year 2014.
assert_eq!(dt, Utc.isoywd(2014, 28, Weekday::Tue).and_hms_opt(9, 10, 11).unwrap());

let dt = NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap().and_local_timezone(Utc).unwrap(); // `2014-07-08T09:10:11.012Z`
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_micro_opt(9, 10, 11, 12_000).unwrap().and_local_timezone(Utc).unwrap());
assert_eq!(dt, NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_nano_opt(9, 10, 11, 12_000_000).unwrap().and_local_timezone(Utc).unwrap());

// dynamic verification
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(21, 15, 33),
           LocalResult::Single(Utc.with_ymd_and_hms(2014, 7, 8, 21, 15, 33).unwrap()));
assert_eq!(Utc.ymd_opt(2014, 7, 8).and_hms_opt(80, 15, 33), LocalResult::None);
assert_eq!(Utc.ymd_opt(2014, 7, 38).and_hms_opt(21, 15, 33), LocalResult::None);

// other time zone objects can be used to construct a local datetime.
// obviously, `local_dt` is normally different from `dt`, but `fixed_dt` should be identical.
let local_dt = Local.from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(9, 10, 11, 12).unwrap()).unwrap();
let fixed_dt = FixedOffset::east_opt(9 * 3600).unwrap().from_local_datetime(&NaiveDate::from_ymd_opt(2014, 7, 8).unwrap().and_hms_milli_opt(18, 10, 11, 12).unwrap()).unwrap();
assert_eq!(dt, fixed_dt);


let re = Regex::new(r"\d+").unwrap();
let haystack = "a111b222c";

assert!(haystack.contains(&re));
assert_eq!(haystack.find(&re), Some(1));
assert_eq!(haystack.match_indices(&re).collect::<Vec<_>>(),
           vec![(1, "111"), (5, "222")]);
assert_eq!(haystack.split(&re).collect::<Vec<_>>(), vec!["a", "b", "c"]);


use http::{Request, Response, StatusCode};

fn respond_to(req: Request<()>) -> http::Result<Response<()>> {
    let mut builder = Response::builder()
        .header("Foo", "Bar")
        .status(StatusCode::OK);

    if req.headers().contains_key("Another-Header") {
        builder = builder.header("Another-Header", "Ack");
    }

    builder.body(())
}


use regex::Regex;
use chrono::{Utc};

fn main() {

    // let now = Utc::now().to_rfc3339();
    let now = Utc::now().to_string();
    println!("{:?}", now);

    let input_b64 = "data:image/jpg;base64,aaaaaaaaaaa=";
    let re = Regex::new(r"^data:image/(.*?);base64,").unwrap();

    let input_ext = re.captures(input_b64).unwrap().get(1).unwrap().as_str();

    println!("{:?}", input_ext);

    assert_eq!(
        re.replace(input_b64, "").into_owned(),
        String::from(re.replace(input_b64, "a")),
    )
}


struct Point {
    x: f64,
    y: f64,
}

// Implementation block, all `Point` associated functions & methods go in here
// 実装のためのブロック。`Point`の持つ関連関数とメソッドを全て定義する。
impl Point {
    // This is an "associated function" because this function is associated with
    // a particular type, that is, Point.
    // これは特定の型（すなわち Point）に関連した関数なので関連関数
    //
    // Associated functions don't need to be called with an instance.
    // These functions are generally used like constructors.
    // 関連関数はインスタンスからでなく呼び出すことができる。
    // 以下のようにコンストラクタとして使用されることが多い。
    fn origin() -> Point {
        Point { x: 0.0, y: 0.0 }
    }

    // Another associated function, taking two arguments:
    // もう一つ関連関数。引数を2つ取る。
    fn new(x: f64, y: f64) -> Point {
        Point { x: x, y: y }
    }
}

struct Rectangle {
    p1: Point,
    p2: Point,
}

impl Rectangle {
    // This is a method
    // `&self` is sugar for `self: &Self`, where `Self` is the type of the
    // caller object. In this case `Self` = `Rectangle`
    // こちらはメソッド。`&self`は`self: &Self`の糖衣構文。
    // `Self`は呼び出し元オブジェクトの型。この場合は`Rectangle`。
    fn area(&self) -> f64 {
        // `self` gives access to the struct fields via the dot operator
        // `self`はドット演算子によって構造体のfieldを参照できる。
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        // `abs` is a `f64` method that returns the absolute value of the
        // caller
        // `abs`は`f64`のメソッドで、呼び出し元の値の絶対値を返す。
        ((x1 - x2) * (y1 - y2)).abs()
    }

    fn perimeter(&self) -> f64 {
        let Point { x: x1, y: y1 } = self.p1;
        let Point { x: x2, y: y2 } = self.p2;

        2.0 * ((x1 - x2).abs() + (y1 - y2).abs())
    }

    // This method requires the caller object to be mutable
    // `&mut self` desugars to `self: &mut Self`
    // このメソッドは呼び出し元オブジェクトがミュータブルであることを
    // 必要とする。`&mut self`は`self: &mut Self`の糖衣構文である。
    fn translate(&mut self, x: f64, y: f64) {
        self.p1.x += x;
        self.p2.x += x;

        self.p1.y += y;
        self.p2.y += y;
    }
}

// `Pair` owns resources: two heap allocated integers
// `Pair`はヒープ上の整数を2つ保持する。
struct Pair(Box<i32>, Box<i32>);

impl Pair {
    // This method "consumes" the resources of the caller object
    // `self` desugars to `self: Self`
    // このメソッドは呼び出し元オブジェクトの持つ要素を「消費」する。
    // `self`は`self: Self`の糖衣構文である。
    fn destroy(self) {
        // Destructure `self`
        // `self`をデストラクト
        let Pair(first, second) = self;

        println!("Destroying Pair({}, {})", first, second);

        // `first` and `second` go out of scope and get freed
        // `first`、`second`はスコープから抜け出すと同時に、解放される。
    }
}

use std::env;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: file_operations <command> <file_path> [other_arguments...]");
        return;
    }

    let command = &args[1];
    let file_path = &args[2];

    match command.as_str() {
        "read" => read_file(file_path),
        "write" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations write <file_path> <text>");
                return;
            }
            let text = &args[3];
            write_file(file_path, text);
        }
        "copy" => {
            if args.len() < 4 {
                eprintln!("Usage: file_operations copy <source_file_path> <destination_file_path>");
                return;
            }
            let destination_path = &args[3];
            copy_file(file_path, destination_path);
        }
        "delete" => delete_file(file_path),
        "list" => list_files(file_path),
        _ => eprintln!("Invalid command. Available commands are: read, write, copy, delete, list."),
    }
}

fn read_file(file_path: &str) {
    let path = Path::new(file_path);
    let file = File::open(&path).unwrap();
    let reader = BufReader::new(file);

    for (index, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        println!("Line {}: {}", index + 1, line);
    }
}

fn write_file(file_path: &str, text: &str) {
    let path = Path::new(file_path);
    let mut file = File::create(&path).unwrap();

    file.write_all(text.as_bytes()).unwrap();
    println!("Text has been written to {}", file_path);
}

fn copy_file(source_path: &str, destination_path: &str) {
    let contents = fs::read(source_path).unwrap();
    fs::write(destination_path, contents).unwrap();
    println!("File has been copied from {} to {}", source_path, destination_path);
}

fn delete_file(file_path: &str) {
    let path = Path::new(file


fn main() {
    let rectangle = Rectangle {
        // Associated functions are called using double colons
        // 関連関数はコロンを2つ挟んで呼び出される。
        p1: Point::origin(),
        p2: Point::new(3.0, 4.0),
    };

    // Methods are called using the dot operator
    // Note that the first argument `&self` is implicitly passed, i.e.
    // メソッドはドット演算子を用いて呼び出される。
    // 最初の引数`&self`は明示せずに受け渡されていることに注目。つまり
    // `rectangle.perimeter()` === `Rectangle::perimeter(&rectangle)`
    println!("Rectangle perimeter: {}", rectangle.perimeter());
    println!("Rectangle area: {}", rectangle.area());

    let mut square = Rectangle {
        p1: Point::origin(),
        p2: Point::new(1.0, 1.0),
    };

    // Error! `rectangle` is immutable, but this method requires a mutable
    // object
    // エラー！`rectangle`はイミュータブルだがこのメソッドはミュータブルなオブジェクトを
    // 必要とする。
    //rectangle.translate(1.0, 0.0);
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。

    // Okay! Mutable objects can call mutable methods
    // OK! ミュータブルなオブジェクトはミュータブルなメソッドを呼び出せる。
    square.translate(1.0, 1.0);

    let pair = Pair(Box::new(1), Box::new(2));

    pair.destroy();

    // Error! Previous `destroy` call "consumed" `pair`
    // エラー！先ほどの`destroy`で`pair`はすでに消費されてしまっている。
    //pair.destroy();
    // TODO ^ Try uncommenting this line
    // TODO ^ この行をアンコメントしてみましょう。
}
