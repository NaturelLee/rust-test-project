#![allow(dead_code)]
pub fn run() {
    println!("===Error Handling===");
    // === Panic ===

    fn drink(beverage: &str) {
        if beverage == "lemonade" {
            // panic!("Aaaaaaa!!!!");
        }

        println!("Some refreshing {} is all I need.", beverage);
    }

    drink("water");
    drink("lemonade");

    // === Option & unwrap ===
    fn give_commoner(gift: Option<&str>) {
        match gift {
            Some("snake") => println!("Yuck! I'm putting this snake in the forest."),
            Some(inner) => println!("{}? How nice.", inner),
            None => println!("No gift? Oh well"),
        }
    }

    fn give_royal(gift: Option<&str>) {
        let inside = gift.unwrap();

        if inside == "snake" {
            // panic!("Aaaaa!");
        }

        println!("I love {}", inside);
    }

    let food = Some("cabbage");
    let snake = Some("snake");
    let void = None;

    give_commoner(food);
    give_commoner(snake);
    give_commoner(void);

    let bird = Some("robin");
    // let nothing = None;

    give_royal(bird);
    // give_royal(nothing);

    fn _next_birthday(current_age: Option<u8>) -> Option<String> {
        let next_age: u8 = current_age?;
        Some(format!("Next year I will be {}", next_age))
    }

    #[derive(Clone, Copy)]
    struct PhoneNumber {
        area_code: Option<u8>,
        number: u32,
    }

    #[derive(Clone, Copy)]
    struct Job {
        phone_number: Option<PhoneNumber>,
    }

    struct Person {
        job: Option<Job>,
    }

    impl Person {
        fn work_phone_area_code(&self) -> Option<u8> {
            self.job?.phone_number?.area_code
        }
    }

    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 2141231,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));

    #[derive(Debug)]
    enum Food {
        CordonBleu,
        Steak,
        Sushi,
    }

    #[derive(Debug)]
    enum Day {
        Monday,
        Tuesday,
        Wednesday,
    }

    fn have_ingredients(food: Food) -> Option<Food> {
        match food {
            Food::Sushi => None,
            _ => Some(food),
        }
    }

    fn have_recipe(food: Food) -> Option<Food> {
        match food {
            Food::CordonBleu => None,
            _ => Some(food),
        }
    }

    fn cookable_v1(food: Food) -> Option<Food> {
        match have_recipe(food) {
            None => None,
            Some(food) => match have_ingredients(food) {
                None => None,
                Some(food) => Some(food),
            },
        }
    }

    fn cookable_v2(food: Food) -> Option<Food> {
        have_recipe(food).and_then(have_ingredients)
    }

    fn eat(food: Food, day: Day) {
        match cookable_v2(food) {
            Some(food) => println!("Yay! on {:?} we get to eat: {:?}.", day, food),
            None => println!("Oh no it's {:?}", day),
        }
    }

    let (cordon_bleu, steak, sushi) = (Food::CordonBleu, Food::Steak, Food::Sushi);

    eat(cordon_bleu, Day::Monday);
    eat(steak, Day::Tuesday);
    eat(sushi, Day::Wednesday);

    // === Result ===
    use std::num::ParseIntError;
    type AliasedResult<T> = Result<T, ParseIntError>;

    fn mutiply(first_num_str: &str, second_num_str: &str) -> AliasedResult<i32> {
        first_num_str.parse::<i32>().and_then(|first_number| {
            second_num_str
                .parse::<i32>()
                .map(|second_number| first_number * second_number)
        })
    }

    fn _multiply(first_number_str: &str, second_number_str: &str) -> Result<i32, ParseIntError> {
        let first_number = first_number_str.parse::<i32>()?;
        let second_number = second_number_str.parse::<i32>()?;

        Ok(first_number * second_number)
    }

    fn print(result: AliasedResult<i32>) {
        match result {
            Ok(n) => println!("n is {}", n),
            Err(e) => println!("Error: {}", e),
        }
    }
    let twenty = mutiply("10", "2");
    print(twenty);
    let tt = mutiply("t", "2");
    print(tt);
    // Introducing ?

    // === Multiple error types ===
    // === Iterating over Results ===
}
