fn main() {
    let water = Some("water");
    let lemonade = Some("lemonade");
    let inner = Some("1");
    let void:Option<&str> = None;
    let coffee = Some("Coffee");
    let coffee2 = Some("Coffee");
    // give_adult(water);
    // give_adult(lemonade);
    // give_adult(inner);
    // give_adult(void);
    // drink(lemonade);
    
    // drink_unwrap(lemonade);
    // drink_unwrap(inner);

    // drink_expect(coffee);
    drink_expect(void);
}

//Implicitamente falando
fn drink_unwrap(drink: Option<&str>){
    let inside = drink.unwrap();
    if inside != "lemonade" {panic!("Deu merda, veio {} 🤷‍♂️", inside);}
    println!("Deu bom, veio {} 🙌", inside);
}

fn drink_expect(drink: Option<&str>){
    let inside = drink.expect("Deu ruim, veio outra coisa 😵");
    println!("{}", inside);
}

//Explicitamente falando
fn give_adult(drink: Option<&str>){
    match drink {
        Some("lemonade")  => println!("cold lemonade"),
        Some("water")     => println!("cold water"),
        Some(inner) => println!("{}?. How nice", inner),
        None              => println!("No drink? Oh man"),
    }
}
