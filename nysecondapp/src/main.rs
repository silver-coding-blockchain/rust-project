
mod back_of_house{
    pub struct Breakfast{
        pub toast:String,
        seasonal_fruit:String,
    }
    impl Breakfast{
        pub fn summer(toast:&str)->Breakfast{
            Breakfast{
                toast:String::from(toast),
                seasonal_fruit:String::from("peaches"),
            }
        }
    }
}

pub fn eat_at_restaurant(){
    let mut meal=back_of_house::Breakfast::summer("Rye");
    meal.toast=String::from("wheat");
    println!("I'd like {} toast please", meal.toast);
}
fn pro_common_collections(){
    let mut v=vec![100,32,57];
    for i in &mut v {
        *i+=50;
        println!("{i}");
    }
    for i in &mut v {
        println!("{i}");
    }
}
fn pro_using_enum_to_store_multipleTypes(){
    #[derive(Debug)]
    enum SpreadSheetCell{
        Int(i32),
        Float(f64),
        Text(String),
    }
    let low=vec![
        SpreadSheetCell::Int(3),
        SpreadSheetCell::Float(10.10),
        SpreadSheetCell::Text(String::from("blue")),
    ];
    println!("{:?},{:?},{:?}",low[0],low[1],low[2]);
}
fn pro1(){
    let s1=String::from("hello, ");
    let s2=String::from("world!");
    let s3=s1.clone()+&s2;
    let s4=s1+&s2;
    println!("{s3}  ");
    println!("{s4}");
}
fn main(){
    //pro_using_enum_to_store_multipleTypes();
    pro1();
}