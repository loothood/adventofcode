extern crate adventofcodelib;

#[test]
fn check_day1_task1() {
    assert_eq!(
        520,
        adventofcodelib::day1_task1(adventofcodelib::FILE_NAME_DAY1)
    );
}

#[test]
fn check_day1_task2() {
    assert_eq!(
        394,
        adventofcodelib::day1_task2(adventofcodelib::FILE_NAME_DAY1)
    );
}

#[test]
fn check_day2_task1() {
    assert_eq!(
        7904,
        adventofcodelib::day2_task1(adventofcodelib::FILE_NAME_DAY2)
    );
}

#[test]
fn check_day2_task2() {
    assert_eq!(
        "wugbihckpoymcpaxefotvdzns",
        adventofcodelib::day2_task2(adventofcodelib::FILE_NAME_DAY2)
    );
}

#[test]
fn check_day3_task1() {
    assert_eq!(
        113966,
        adventofcodelib::day3_task1(adventofcodelib::FILE_NAME_DAY3)
    );
}

#[test]
fn check_day3_task2() {
    assert_eq!(
        235,
        adventofcodelib::day3_task2(adventofcodelib::FILE_NAME_DAY3)
    );
}

#[test]
fn check_parse_inch_valid() {
    let input = "#1318 @ 428,284: 25x21";
    let inch = adventofcodelib::parse_inch(input);
    let expected = adventofcodelib::Inch::new(1318, 428, 284, 25, 21);
    assert_eq!(expected, inch);
}

#[test]
fn check_parse_inch_invalid() {
    let input = "#1318 @ 428,284: 25x21";
    let inch = adventofcodelib::parse_inch(input);
    let expected = adventofcodelib::Inch::new(1320, 428, 284, 25, 21);
    assert_ne!(expected, inch);
}

#[test]
#[should_panic(expected = "Cannot parse string '#AD @ 428,284: 25x21")]
fn check_parse_inch_must_fail() {
    let input = "#AD @ 428,284: 25x21";
    adventofcodelib::parse_inch(input);
}

#[test]
#[should_panic(expected = "DAY3: Cannot parse id value!")]
fn check_parse_inch_must_fail_id() {
    let input = "#4294967296 @ 428,284: 25x21";
    adventofcodelib::parse_inch(input);
}

#[test]
#[should_panic(expected = "DAY3: Cannot parse left value!")]
fn check_parse_inch_must_fail_left() {
    let input = "#1 @ 4294967299,284: 25x21";
    adventofcodelib::parse_inch(input);
}

#[test]
#[should_panic(expected = "DAY3: Cannot parse top value!")]
fn check_parse_inch_must_fail_top() {
    let input = "#1 @ 42,4294967300: 25x21";
    adventofcodelib::parse_inch(input);
}

#[test]
#[should_panic(expected = "DAY3: Cannot parse wide value!")]
fn check_parse_inch_must_fail_wide() {
    let input = "#1 @ 42,11: 4294967405x21";
    adventofcodelib::parse_inch(input);
}

#[test]
#[should_panic(expected = "DAY3: Cannot parse toll value!")]
fn check_parse_inch_must_fail_toll() {
    let input = "#1 @ 42,11: 55x4294967500";
    adventofcodelib::parse_inch(input);
}
