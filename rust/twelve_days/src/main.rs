fn main() {

    println!("\nTwelve Days Of Christmas\n");
    for day in 1..13 {
        println!("On the {day}th of Christmas my true love sent to me");
            for moment in (1..day+1).rev() {
                match moment {
                    1 => {println!("{moment} partridge")},
                    2 => {println!("{moment} turtledoves")},
                    3 => {println!("{moment} French hens")},
                    4 => {println!("{moment} calling birds")},
                    5 => {println!("{moment} golden rings")},
                    6 => {println!("{moment} geese a-laying")},
                    7 => {println!("{moment} swans A-swimming")},
                    8 => {println!("{moment} maids a-milking")},
                    9 => {println!("{moment} ladies dancing")},
                    10 => {println!("{moment} lords a-leaping")},
                    11 => {println!("{moment} Pipers piping")},
                    12 => {println!("{moment} drummers drumming")},
                    a => {println!("{a} unkown number");}
            }
        }
        println!("In a pear tree\n");
    }
}
