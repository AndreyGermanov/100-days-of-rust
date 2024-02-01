use std::collections::HashMap;

#[derive(PartialEq)]
enum TimeType { HOUR, MINUTE }

fn time_in_words<'a>(hour:u8, minute:u8) -> Result<String,&'a str> {
    if hour == 0 || hour > 12 { return Err("Incorrect hour specification") };
    if minute > 59 { return Err("Incorrect minute specification") };
    if minute == 0 {
        if hour == 0 { return Ok("midnight".to_string()); }
        return Ok(format!("{} o' clock",get_number_as_words(hour, TimeType::HOUR)));
    }
    if minute <= 30 {
        return Ok(
            format!("{} past {}",get_number_as_words(minute,TimeType::MINUTE),
                get_number_as_words(hour,TimeType::HOUR)
            )
        );
    }
    Ok(
        format!("{} to {}",get_number_as_words(60-minute,TimeType::MINUTE),
           get_number_as_words(hour+1,TimeType::HOUR)
        )
    )
}
#[test]
fn test_time_in_words() {
    assert_eq!(time_in_words(3, 00).unwrap(), "three o' clock");
    assert_eq!(time_in_words(5, 01).unwrap(), "one minute past five");
    assert_eq!(time_in_words(7, 15).unwrap(), "quarter past seven");
    assert_eq!(time_in_words(5, 28).unwrap(), "twenty eight minutes past five");
    assert_eq!(time_in_words(5, 47).unwrap(), "thirteen minutes to six");
    assert_eq!(time_in_words(5, 30).unwrap(), "half past five");
    assert_eq!(time_in_words(8, 45).unwrap(), "quarter to nine");
    assert!(time_in_words(0, 30).is_err());
    assert!(time_in_words(5, 65).is_err());
}

fn get_number_as_words(number: u8, time_type:TimeType) -> String  {
    if number == 0 { return "".to_string() }
    if number == 15 { return "quarter".to_string() };
    if number == 30 { return "half".to_string() }
    let numbers:HashMap<u8,&str> =HashMap::from([
        (1, "one"), (2, "two"), (3, "three"), (4, "four"), (5, "five"), (6, "six"), (7, "seven"),
        (8, "eight"), (9, "nine"), (10, "ten"), (11, "eleven"), (12, "twelve"), (13, "thirteen"),
        (14, "fourteen"), (15, "fifteen"), (16, "sixteen"), (17, "seventeen"), (18, "eighteen"),
        (19, "nineteen"), (20, "twenty"), (30, "thirty"), (40, "forty"), (50, "fifty"),
        (60, "sixty"), (70, "seventy"), (80, "eighty"), (90, "ninety")
    ]);
    if time_type == TimeType::HOUR { return format!("{}",numbers[&number]) };
    if number == 1 { return format!("{} minute",numbers[&number]) }
    if number < 21 { return format!("{} minutes",numbers[&number]) }
    let tens = (number / 10) * 10;
    let ones = number - tens;
    if ones == 0 {  return format!("{} minutes",numbers[&tens]) }
    if ones == 1 { return format!("{} {} minute", numbers[&tens], numbers[&ones]) }
    format!("{} {} minutes", numbers[&tens], numbers[&ones])
}

#[test]
fn test_number_as_words() {
    assert_eq!(get_number_as_words(0,TimeType::MINUTE),"");
    assert_eq!(get_number_as_words(1,TimeType::MINUTE),"one minute");
    assert_eq!(get_number_as_words(3,TimeType::MINUTE),"three minutes");
    assert_eq!(get_number_as_words(5,TimeType::MINUTE),"five minutes");
    assert_eq!(get_number_as_words(10,TimeType::MINUTE),"ten minutes");
    assert_eq!(get_number_as_words(12,TimeType::MINUTE),"twelve minutes");
    assert_eq!(get_number_as_words(15,TimeType::MINUTE),"quarter");
    assert_eq!(get_number_as_words(30, TimeType::MINUTE),"half");
    assert_eq!(get_number_as_words(32, TimeType::MINUTE),"thirty two minutes");
    assert_eq!(get_number_as_words(45, TimeType::MINUTE),"forty five minutes");
    assert_eq!(get_number_as_words(59, TimeType::MINUTE),"fifty nine minutes");
}


