// //a summary trait that consists of the same behaviour provided by a summarize method
// pub trait Summary {
//     fn summarize (&self) -> String; //the method signature that describe the behaviors of the types that implement the trait
// }

// pub struct NewsArticle { //type tweet
//     pub headline: String,
//     pub location: String,
//     pub author: String,
//     pub content: String,
// }

// impl Summary for NewsArticle { //implemeting the Summary trait for type NewsArticle
//     fn summarize(&self) -> String {
//         format!("{}, by {} ({})", self.headline, self.author, self.location)
//     }
// }

// pub struct Tweet { //type new article
//     pub username: String,
//     pub content: String,
//     pub reply: bool,
//     pub retweet: bool,
// }

// impl Summary for Tweet { //implementing the Summary triat for type Tweet
//     fn summarize(&self) -> String {
//         format!("{}: {}", self.username, self.content)
//     }
// } 

// //use aggregator::{Summary, Tweet};
// fn main(){
//     let tweet = Tweet {
//         username: String::from("hors_ebooks"),
//         content: String::from(
//             "As you have already know people, people change alot",
//         ),
//         reply: false,
//         retweet: false
//     };
//     println!("1 new tweet: {}", tweet.summarize())
// }


//////////////////////////////////////////////////////////////////////////////////////////////
//easy rust
//how we cannot use trait in rust
//#[derive(std::ops::Add)] //this will not work because rust doesnt know what to do at compile time like do we want to change the i32 to f32 and add them to g
// a float, or do we want to round it up to an integer, or do we want to turn into a string. so we have to tell rust exactly how to do it.in order to make it work, we have 
// to implement a trait Add

// struct ThingsToAdd {
//     first_thing: i32,
//     second_thing: f32,
// }

// impl std::ops::Add for ThingsToAdd{
    
// }

// fn main(){
//     let my_thing = ThingsToAdd {
//         first_thing: 7,
//         second_thing: 7.8,
//     };
// }

//////////////////////////////////////////////////////////////////////////////////////////////////

// struct Animal {
//     name: String,
// }

// trait Canine {
//      //canine is like an adjective for dogs which will make the struct into a canine which gives it the ability to bark
//      fn bark(&self) {
//         println!("Woof woof");
//      } // dog can bark, and this has given canine the ability to bark also
// // what else can dogs do ?
// //Dogs can run 
//  fn run(&self) {
//     println!("The dog is running");
//  }

// /*the above function are two super simple functions which doesnot require us to tell rust what type it is.so this is very easy to implement. */

// }

// impl Canine for Animal { /* the animal struct that we created */
//       // we can also implement our own run fuction 
//       fn run(&self){
//         let my_number = 5; 
//         println!("Dog number {} is running!", my_number);
//         //we are trying to over write the run function that is in the trait canine, and we only do that for our struct Animal
//       }

// }
// fn main(){
//     let rover = Animal{
//         name: "Rover".to_string(),
//     };
//     /*after calling the instance of the Animal, then we call the methods below */
//     rover.bark(); //dogs can bark
//     rover.run(); //dogs can run
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// //easy rust part 2
// trait DoThings{//we call it triat do things ecause verbs are good, or verb related things are good for the name of a trait.
//     fn do_one_thing(&self);
//     fn do_different_things(&self);
// }

// struct Human {
//     name: String,
// }

// // next we implement do things for man
// impl DoThings for Human {}
// fn main() {

// }

// use core::fmt;

// ////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// //part 3
// #[derive(Debug)] 
// struct Cat {
//     name: String,
//     age:u8,
// }

// impl std::fmt::Display for Cat {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> std::fmt::Result {
//         //we can also have our logic here 
//        let cat_oldness= match self.age {
//            0..=2 => "kitten",
//            3..=10 => "adult cat",
//            _ => "old_cat"
//        };
//         write!(f, "{} is a cat who is {} years old, and therefore an {}", self.name, self.age, cat_oldness)
//     }
// }

// fn main() {
//     let Mr_silly = Cat{
//         name: "Billy".to_string(),
//         age: 3,
//     };
//     println!("Billy is a {}", Mr_silly); //with debug 
    
// }

// ///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// /// part 4
// struct Monster {
//     health: i32,

// }

// struct Wizard {}
// struct Ranger {}

// /* next we think of a trait that will have th fight character */
// trait FightClose {
//     //that is if u have the trait fight close, you can attack with a sword which will take a reference to self
//     fn attack_with_sword(&self, opponent: &mut Monster) {
//         opponent.health -=10; // we make it decrease
//         println!("you attacked with youe sword. Your opponent has {} health left.", opponent.health);
//     }

//     fn attack_with_hand(&self, opponent: &mut Monster) {
//         opponent.health -= 2;
//         println!("you attacked your opponent with your hand. Your opponent now has {} health left.", opponent.health);
//     }
// }

// /* Next, we now give the ightClose trait to wizard and ranger with the below code*/
// impl FightClose for Wizard {}
// impl FightClose for Ranger {}

// /*next we write another trait FightFromDistance */
// trait FightFromDistance {
//     fn attack_with_bow(&self, opponent: &mut Monster, distance: u32) {
//         if distance < 10 {
//             opponent.health -= 10;
//             println!("You attacked with your bow. Your opponent has {} health left", opponent.health)
//         }
//     }
    
//     fn attack_with_rock(&self, opponent: &mut Monster, distance: u32) {
//         if distance < 3 {
//             opponent.health -= 4;
//         }
//         println!("You attacked with rock. Your opponent now has {} health left", opponent.health)
//     }

// }
// /* Can you guess who this trait is going to be attributed to? */
// /*the trait FightFromDistance will be attributed to Ranger. then we implement it for ranger*/
// impl FightFromDistance for Ranger {}
// //Ranger has implemeted two traits, while wizard only got one trait implemented
// fn main() {
//     let Gandalf = Wizard {};
//     let thorn = Ranger {};

//     let mut rendai = Monster {
//                                health:40
//     };
//     //then we call method on Gandalf to attack him
//     let distance = 88; 
//     Gandalf.attack_with_sword(&mut rendai); // we made the oppenent mutable here because we want to take his health down
//     thorn.attack_with_bow(&mut rendai,  distance)
// }

////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//trait bounds part 6   
// use std::fmt::Debug;

// struct Monster {
//     health: i32
// }

// #[derive(Debug)]
// struct Wizard {
//     health: i32, 
// }

// #[derive(Debug)]
// struct Ranger {
//     health: i32,
// }

// trait Magic {}
// trait FightClose {}
// trait FightFromDistance{}  // traits are like powers, thats why its good for roles in games 

// //next we need to think on who can fight close and who can fight from distance
// impl FightClose for Ranger {}
// impl FightClose for Wizard {}
// impl FightFromDistance for Ranger {}
// impl Magic for Wizard {}

// /* next, we are going with our generic fuction. Any type can use our generic which must implement the traits */

// fn attack_with_bow<T: FightFromDistance + Debug>(character: &T, opponent: &mut Monster, distance: u32){
//     if distance < 10 {
//         opponent.health -= 10;
//         println!("You attacked with your bow. Your opponent now has {} health left. You are now at: {:?}", opponent.health, character)
//     }
// }

// fn attack_with_sword<T: FightClose + Debug>(character: &T, opponent: &mut Monster){
//     opponent.health -= 10;
//     println!("You attacked with your sword. Your opponent now has {} health left. You are now at: {:?}", opponent.health, character);
// }

// fn fireball<T: Magic + Debug>(character: &T, opponent: &mut Monster, distance: u32){
//     if distance < 15 {
//         opponent.health -= 20;
//         println!("You raise your hands and cast a fireball! Your opponent now has {} health left. You are now at: {:?}", opponent.health, character);

//     }
// }

// fn main() {
//     let gandalf = Wizard{health: 70};
//     let thorn = Ranger{health: 70};

//     let mut rendai = Monster {health: 50};
//     attack_with_bow(&thorn, &mut rendai, 8);
//     attack_with_sword(&gandalf, &mut rendai);
//     fireball(&gandalf, &mut rendai, 10);

// }


//////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//Part 7: The From Trait

// use std::fmt::Display;
// fn print_vec<T: Display>(input:&Vec<T>) {
//     for item in input {
//         print!("{}", item);
//     }
//     println!();
// }

// fn main(){
//     let array_vector = Vec::from([8, 9, 10]);
//     print_vec(&array_vector);

//     let str_vector = Vec::from("What kind of vector will i be?");
//     print_vec(&str_vector);

//     let string_vector = Vec::from(String::from("Will i still be another form of vector?"));
//     print_vec(&string_vector);
// }
//////////////////////////////////////////////////////////////////////////////////////////////////////////////
/*part 8 */
/*implementing the From trait*/
// use std::convert::From;
// #[derive(Debug)]
// struct City {
//     name: String,
//     population: u32,
// }

// impl City {
//       fn new(name: &str, population: u32) -> Self {
//         Self {
//             name: name.to_string(),
//             population
//         }
//       }
// }

// #[derive(Debug)]
// struct Country {
//   cities: Vec<City>
// }

// impl From<Vec<City>> for Country {
//   fn from(cities: Vec<City>) -> Self {
//     Self {
//       cities
//     }
//   }
// }

// impl Country {
//   fn print_cities(&self) {
//     for city in &self.cities {
//       println!("{:?} has a population of {:?}", city.name, city.population);
//     }
//   }
// }





// fn main(){
//    let helsinki = City::new("Helsinki", 631_695);
//    let turku = City::new("Turku", 186_756);

//    let finland_cities = vec![helsinki, turku];
//    let finland = Country::from(finland_cities);

//    finland.print_cities()
// }

///////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//implemting the from trait again
// use std::convert::From;
// #[derive(Debug)]
// struct EvenoddVec(Vec<Vec<i32>>);

// impl From<Vec<i32>> for EvenoddVec {
//   fn from(input: Vec<i32>) -> Self {
//     let mut even_odd_vec  = vec![vec![], vec![]];

//     for item in input {
//       if item % 2 == 0 {
//         even_odd_vec[0].push(item);
//       } else {
//         even_odd_vec[1].push(item)
//       }
//     }
//     Self(even_odd_vec)
//   }
// }

// fn main(){
//   let bunch_of_numbers = vec![8, 7, -1, 3, 222, 9787, -47, 77, 0, 55, 700];
//   let new_vec = EvenoddVec::from(bunch_of_numbers);

//   print!("{:?}", new_vec);
// }


////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
//part 10
//AsRef



