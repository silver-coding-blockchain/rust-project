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
fn main(){
    pro_common_collections();
}