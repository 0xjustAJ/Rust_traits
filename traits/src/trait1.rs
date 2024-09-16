//how we cannot use trait in rust
#[derive(std::ops:: Add)]
struct ThingsToAdd {
    first_thing: i32,
    second_thing: f32,
}

fn main(){
    let my_thing = ThingsToAdd {
        first_thing: 7,
        second_thing: 7.8,
    };
}