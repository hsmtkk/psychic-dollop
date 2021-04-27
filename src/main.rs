mod raindrops;

fn main() {
    for num in 10..30 {
        let msg = raindrops::rain_drops(num);
        println!("{}", msg);
    }
}
