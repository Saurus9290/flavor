enum Flavor {
    Sweet,
    Spicy,
    Fruity,
    Salty,

}

struct Drink {
    flavor: Flavor,
    fluid_ounce: f32
}

fn print_drink(drink:Drink) {
    match drink.flavor {
        Flavor::Sweet => println!("its Sweet"),
        Flavor::Spicy => println!("its Spicy"),
        Flavor::Fruity => println!("its Fruity"),
        Flavor::Salty => println!("its Salty")
    }
    println!("{}", drink.fluid_ounce);
}


fn main() {
    let drink = Drink {
        flavor:Flavor::Sweet,
        fluid_ounce: 3.2
    };
    print_drink(drink);
}