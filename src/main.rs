fn main() {
    let gifts = ["a partridge in a pear tree", "two turtle doves", "three French hens","four calling birds", "five gold rings", "six geese a-laying", "seven swans a-swimming", "eight maids a-milking", "nine ladies dancing", "ten lords a-leaping", "eleven pipers piping", "twelve drummers drumming"];
    let days = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    for day in 0..12{
        let day_c = days[day];
        println!("On the {day_c} day of Christmas my true love gave to me");
        let gift_range = (0..=day).rev();
        for gift in gift_range{
            let gift_add = gifts[gift];
             println!("{gift_add}!")
        }
    }
}
