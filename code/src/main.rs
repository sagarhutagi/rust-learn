#[allow(dead_code)]

enum Coin{
    Penny, 
    Nickel,
    Dime, 
    Quarter,
}

fn value_of_coin(coin: Coin) -> u8{
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 2,
        Coin::Dime => 3,
        Coin::Quarter => 4,
    }
}

fn main(){
    let coin : Coin = Coin::Penny;
    let value = value_of_coin(coin);
    println!("{value}");
}