// fn returns_reference() -> &str {
//     // because this &my_string is belong to my_string,
//     // so &my_string can't life more my_string.
//     let my_string = String::from("I am a string");
//     &my_string // ⚠️
// }
//
// fn main() {}

// fn returns_str() -> &str {
//     let my_string = String::from("I am a string");
//     "I am a str" // ⚠️
// }
//
// fn main() {
//     let my_str = returns_str();
//     println!("{}", my_str);
// }

// fn returns_str() -> &'static str {
//     let my_string = String::from("I am a string");
//     "I am a str"
// }
//
// fn main() {
//     let my_str = returns_str();
//     println!("{}", my_str);
// }
//
// #[derive(Debug)]
// struct City {
//     name: &'static str, // ⚠️
//     date_founded: u32,
// }
//
// fn main() {
//     let my_city = City {
//         name: "Ichinomiya",
//         date_founded: 1921,
//     };
//
//     println!("{:?}", my_city);
// }

//
// #[derive(Debug)]
// struct City {
//     name: &'static str, // must live for the whole program
//     date_founded: u32,
// }
//
// fn main() {
//     let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()]; // city_names does not live for the whole program
//
//     let my_city = City {
//         name: &city_names[0], // ⚠️ This is a &str, but not a &'static str. It is a reference to a value inside city_names
//         date_founded: 1921,
//     };
//
//     println!("{} was founded in {}", my_city.name, my_city.date_founded);
// }

// #[derive(Debug)]
// struct City<'a> { // City has lifetime 'a,
//     // we can use anything to replace a, just like generics we usually use <T> and <U>
//     name: &'a str, // and name also has lifetime 'a.
//     date_founded: u32,
// }
//
// fn main() {
//     let city_names = vec!["Ichinomiya".to_string(), "Kurume".to_string()];
//
//     let my_city = City {
//         name: &city_names[0],
//         date_founded: 1921,
//     };
//
//     println!("{} was founded in {}", my_city.name, my_city.date_founded);
// }

// // ⚠️
// struct Adventurer<'a> {
//     name: &'a str,
//     hit_points: u32,
// }
//
// impl Adventurer<'_> {
//     fn take_damage(&mut self) {
//         self.hit_points -= 20;
//         println!("{} has {} hit points left!", self.name, self.hit_points);
//     }
// }
//
// fn main() {}

// //  ⚠️
// struct Adventurer {
//     name: &str,
//     hit_points: u32,
// }
//
// impl Adventurer {
//     fn take_damage(&mut self) {
//         self.hit_points -= 20;
//         println!("{} has {} hit points left!", self.name, self.hit_points);
//     }
// }
//
// impl std::fmt::Display for Adventurer {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         write!(f, "{} has {} hit points.", self.name, self.hit_points)
//     }
// }
//
// fn main() {}

// fn main() {
//     // // That's ok, because we change b -> a, and a not in {}, so that's ok.
//     // let a;
//     // {
//     //     let b = String::from("hello");
//     //     a = b;
//     // }
//     // println!("{a}");
//
//     // that'll bad, b will drop when place }.
//     let a;
//     {
//         let b = String::from("hello");
//         a = &b;
//     }
//     println!("{a}");
// }

// fn main() {
//     let num = 1;
//     let res = another_get_int_ref(&num);
//     println!("{res}")
// }
//
// // 所以这里有一个问题: 函数参数中并没有任何参数, 所以返回值只能是函数中创建的参数,
// // 但是对应的: 函数中创建的参数在函数返回的时候就会被销毁. 所以没办法返回(保存).
// // fn get_int_ref() -> &i32 {
// //     let a = 1;
// //     &a
// // }
//
//
// // That's ok, why?
// // 这意味着: 输入内存和输出内存都在同一个作用域中. (生命周期相同)
// fn another_get_int_ref<'a>(param: &'a i32) -> &'a i32 {
//     param
// }
//
//
// // // 当参数中含有很多的引用类型的时候: 编译器会自动分配不同的生命周期. 若是原始数据, 就会自动略过.
// // // 这都没有任何问题: 但是需要注意的是: 返回值这里的问题.
// // // 返回值若是一个原始数据, 那么并不存在生命周期的问题.
// // // 但是若是返回值是一个引用数据类型. 那么这就有一个问题了.
// fn some_function<'a, 'b>(param1: &'a str, param2: &'b str, param3: String) -> String {
//     param3
// }
//
// // // 返回值若是一个原始数据, 那么并不存在生命周期的问题.
// // // 但是若是返回值是一个引用数据类型. 那么这就有一个问题了.
// // // 编译器不知道这个引用应该与哪个输入参数的生命周期关联
// // // 或者不知道这个引用应该活多久
// fn another_some_function<'a, 'b>(param1: &'a str, param2: &'b str, param3: String) -> &str {
//     param3
// }
//
// // 关于 const 和 static 修饰的参数是不是可以引用的问题, 实际上他们的生命周期是: 'static.

// fn main() {}

// 这里我们并不需要显式的写出 param2 的生命周期, 因为 param2 永远不会被返回.
// 但是编译器还是为 param2 分配了一个生命周期, 只是并没有显式定义出来而已.
// fn get_int_ref<'a>(param1: &'a i32, param2: &i32) -> &'a i32 {
//     println!("{param2}");
//     param1
// }

// 这就是显式定义出来的.
// fn get_int_ref<'a, 'b>(param1: &'a i32, param2: &'b i32) -> &'a i32 {
//     println!("{param2}");
//     param1
// }

// 这里有一个新的问题: 若是返回 param2 和 param1 都有可能怎么办呢?
// 若是返回 param1 还好说, 因为生命周期是相同的.
// 若是返回 param2 就会报错.
// 一般最常用的解决办法是: 同意生命周期.
// fn get_int_ref<'a, 'b>(param1: &'a i32, param2: &'b i32) -> &'a i32 {
//     if param1 > param2 {
//         param1
//     } else {
//         param2
//     }
// }
// fn get_int_ref<'a>(param1: &'a i32, param2: &'a i32) -> &'a i32 {
//     if param1 > param2 {
//         param1
//     } else {
//         param2
//     }
// }

// // 这里我们显式声明了所有的参数都是同一个生命周期, (因为不同的引用, 编译器会分配不同的生命周期.)
// fn get_str_ref<'a>(param1: &'a str, param2: &'a str) -> &'a str {
//     if param1 > param2 { param1 } else { param2 }
// }

// 没有任何问题, 而且生命周期也不会加入其中, 因为并没有任何引用.
// fn test_1(param1: Vec<f64>) -> Vec<f64> {
//     param1
// }

// 这同样没有任何问题, 因为我们并没有引用作为返回值.
// fn test_2(param1: &Vec<f64>) -> Vec<f64> {
//     param1.clone()
// }
// fn test_2<'a>(param1: &'a Vec<f64>) -> Vec<f64> { // 可以省略. 显式声明了.
//     param1.clone()
// }

// 这里会报错, 因为输入并没有引用, 并且生命周期也不会自动添加. 返回值也是会报错.
// fn test_3<'a>(param1: Vec<f64>) -> &'a Vec<f64> {
//     &param1
// }

// 这里的问题在于: Rust编译器默认为两个引用类型的数据分配了两个生命周期(因为这里有两个不同的引用类型数据)
// 那么这里的问题是: 最后的返回值的生命周期不知道选择哪一个.
// fn test_1(param_1: i32, param_2: &str, param_3: &str, param_4: f64) -> &str {
//     if param_1 == 7 && param_4 > 10_f64 {
//         param_2
//     } else {
//         param_3
//     }
// }

// 下面是显式子声明两个生命周期.
// 那么这就回到了原始的问题: 最后的返回值应该是哪一个生命周期呢?
// fn test_1<'a, 'b>(param_1: i32, param_2: &'a str, param_3: &'b str, param_4: f64) -> &str {
//     if param_1 == 7 && param_4 > 10_f64 {
//         param_2
//     } else {
//         param_3
//     }
// }

// 所以这里需要我们进行强制性的说明: 所有都是一个生命周期就行了.
// 当然: 任何永远不会作为输出返回的引用参数，使用 Rust 提供的默认生命周期即可。
// fn test_1<'a>(param_1: i32, param_2: &'a str, param_3: &'a str, param_4: f64) -> &'a str {
//     if param_1 == 7 && param_4 > 10_f64 {
//         param_2
//     } else {
//         param_3
//     }
// }

// 当然: 任何永远不会作为输出返回的引用参数，使用 Rust 提供的默认生命周期即可。
// fn test_1<'a>(param_1: i32, param_2: &'a str, param_3: &'a str, param_4: &f64) -> &'a str {
//     println!("{param_4}");
//     if param_1 == 7 {
//         param_2
//     } else {
//         param_3
//     }
// }

// 这里的输入和输出都是只有一个引用数据类型, 那么编译器会自动为他们设置相同的生命周期.
// 所以我们这里并不需要显式声明.
// fn test_2<'a>(param_1: &'a str) -> &'a str {
//     param_1
// }
// fn test_2(param_1: &str) -> &str {
//     param_1
// }

// fn main() {
//     let a: Vec<i32> = vec![1, 2, 3, 4, 5, 6];
//     let result = get_vec_slice(&a, &a);
//     println!("{:?}", result);
// }
//
// // 同样的: 还是只有一个输入引用, 一个输出引用, 而且是一个vec类型的.
// // 所有的都是同一个生命周期, 包括对应的子集.
// // fn get_vec_slice(param_1: &[i32]) -> &[i32] {
// //     &param_1[0..2]
// // }
//
// // 这里同样的问题: 分配了两个不同的生命周期, 所以不知道要返回的是哪一个.
// // 对应的: 我们显式声明.
// fn get_vec_slice<'a>(param_1: &'a [i32], param_2: &'a [i32]) -> &'a [i32] {
//     if param_1.len() > param_2.len() {
//         &param_1[0..2]
//     } else {
//         &param_2[0..2]
//     }
// }

// const SOME_CONST_A: &str = "I'm a constant!";
// const SOME_CONST_B: &str = "I'm a constant too!";
//
// fn main() {
//     let a = String::from("fdsakoasdf");
//     let greater = some_func(&a, &SOME_CONST_B);
//     print!("{greater}");
// }
//
// // fn some_func(param_1: &'static str, param_2: &'static str) -> &'static str {
// //     if param_1 > param_2 {
// //         param_1
// //     } else {
// //         param_2
// //     }
// // }
//
// // 因为 'static 会自动向下兼容 'a 所以这个没有问题.
// fn some_func<'a>(param_1: &'a str, param_2: &'a str) -> &'a str {
//     if param_1 > param_2 {
//         param_1
//     } else {
//         param_2
//     }
// }

// fn main() {
//     let some_float1 = 1.1;
//     let some_float2 = 2.2;
//     let ans1 = get_smaller(&some_float1, &some_float2);
//
//     let some_str1 = "a";
//     let some_str2 = "b";
//     let ans2 = get_smaller(&some_str1, &some_str2);
// }
//
// // 老生常谈的问题: 统一生命周期.
// fn get_smaller<'a, T: std::cmp::PartialOrd>(param_1: &'a T, param_2: &'a T) -> &'a T {
//     if param_1 < param_2 {
//         param_1
//     } else {
//         param_2
//     }
// }

// // 关于这个结构体的生命周期也是一样的.
// // 我们甚至可以进一步保证 b 的生命周期 不少于 a, 但是这个我们基本上用不到, 只要我们知道就行了.
// struct City<'a, 'b: 'a> {
//     name: String,
//     country: &'a Vec<String>,
//     all: &'b str,
// }
//
// fn main() {}

struct Adventurer<'a> {
    name: &'a str,
    hit_points: u32,
}

impl Adventurer<'_> {
    fn take_damage(&mut self) {
        self.hit_points -= 20;
        println!("{} has {} hit points left!", self.name, self.hit_points);
    }
}

impl std::fmt::Display for Adventurer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} has {} hit points.", self.name, self.hit_points)
    }
}

fn main() {
    let mut billy = Adventurer {
        name: "Billy",
        hit_points: 100_000,
    };
    println!("{}", billy);
    billy.take_damage();
}
