// type alias
type Meters = i32;

const TOUCHDOWN_POINTS:i32 = 6;

#[allow(unused_doc_comments)]
fn main() {
    
    let apples:i64 = 59;
    let oranges:i64 = 59;
    let _fruits:i64 = apples + oranges;

    println!("My garden has {apples} apples & {oranges} oranges");

    //mutability

    let mut gym_reps:i64 = 10;
    gym_reps = 20;

    println!("Total gym reps = {gym_reps}");

    //variable shadowing
    #[allow(unused_variables)]
    let grams_of_protein = "100.35";
    let grams_of_protein = 32;
    {
        let grams_of_protein = 12.67;
        println!("{grams_of_protein}");
    }
    println!("{grams_of_protein}");


    /**
     * Scopes - 
     * It is a boundary in which a name is valid
     * Block is the area b/w opening curly braces & closing curly braces
     */
    

    let value = 12;
    {
        let value = "some value";
        println!("{value}"); //it is a string
    }
    println!("{value}"); //it is a i32

    // constants
    const TAX_RATE:f64 = 12.00;
    let total_tax = 10000.00 * TAX_RATE/100.00;

    println!("Total tax i have to pay {total_tax}");

    // Type Alias
    let distance:Meters = 23;
    println!("I have to travel {distance} M");

    // project
    let season:&str = "winters";
    let mut points_scored:i32 = 28;
    points_scored = 32; //hurray goal scored! value updated

}

