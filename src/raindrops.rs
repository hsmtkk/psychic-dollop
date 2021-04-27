pub fn rain_drops(num: i64) -> String {
    let mut result = String::new();
    if num % 3 == 0 {
        result.push_str("Pling");
    }
    if num % 5 == 0 {
        result.push_str("Plang");
    }
    if num % 7 == 0 {
        result.push_str("Plong");
    }
    if result == "" {
        return num.to_string();
    }
    return result;
}

#[test]
fn test_rain_drops(){
    assert_eq!("Plong", rain_drops(28));
    assert_eq!("PlingPlang", rain_drops(30));
    assert_eq!("34", rain_drops(34));
}