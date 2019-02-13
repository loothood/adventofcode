use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};

///
/// Read file by lines. Return collection of lines.
///
/// # Arguments
///
/// * `file_name` - a path to the file with input data.
///
fn read_input(file_name: &str) -> Lines<BufReader<File>> {
    BufReader::new(
        File::open(file_name)
            .unwrap_or_else(|e| panic!("Cannot read file '{}. The error is {:?}'.", file_name, e)),
    )
    .lines()
}

//-------------------------------------------- day 1 --------------------------------------------//

///
/// Transform vector of strings to vector of i32
///
/// # Arguments
///
/// * `file_name` - a path to the file with input data. Data example:
///
///  +13
///
/// -7
///
/// -17
///
/// +12
///
/// -11
///
/// +19
///
/// +18
///
/// +19
///
fn get_vec_int(file_name: &str) -> Vec<i32> {
    read_input(file_name)
        .map(|line| {
            line.expect("Line reading error!'")
                .parse::<i32>()
                .expect("Line parsing error!")
        })
        .collect()
}

///
/// # The task explanation
/// The device displays frequency changes of +1, -2, +3, +1,
/// then starting from a frequency of zero, the following changes would occur:
///
///    Current frequency  0, change of +1; resulting frequency  1.
///
///    Current frequency  1, change of -2; resulting frequency -1.
///
///    Current frequency -1, change of +3; resulting frequency  2.
///
///    Current frequency  2, change of +1; resulting frequency  3.
///
///
/// In this example, the resulting frequency is 3.
/// Here are other example situations:
///
///    +1, +1, +1 results in  3
///
///    +1, +1, -2 results in  0
///
///    -1, -2, -3 results in -6
///
///
/// Starting with a frequency of zero, what is the resulting frequency after all of the changes in frequency have been applied?
///
///
/// # Arguments
///
/// * `file_name` - a path to the file with input data. Data example:
///
/// +13
///
/// -7
///
/// -17
///
/// +12
///
/// -11
///
/// +19
///
/// +18
///
/// +19
///
pub fn day1_task1(file_name: &str) -> i32 {
    get_vec_int(file_name).iter().sum()
}

///
/// # The task explanation
/// The device would loop as follows:
///
/// Current frequency  0, change of +1; resulting frequency  1.
///
/// Current frequency  1, change of -2; resulting frequency -1.
///
/// Current frequency -1, change of +3; resulting frequency  2.
///
/// Current frequency  2, change of +1; resulting frequency  3.
///
/// (At this point, the device continues from the start of the list.)
///
/// Current frequency  3, change of +1; resulting frequency  4.
///
/// Current frequency  4, change of -2; resulting frequency  2, which has already been seen.
///
/// In this example, the first frequency reached twice is 2.
/// Note that your device might need to repeat its list of frequency changes many times before
/// a duplicate frequency is found, and that duplicates might be found while in the middle
/// of processing the list.
///
/// Here are other examples:
///
/// +1, -1 first reaches 0 twice.
///
/// +3, +3, +4, -2, -4 first reaches 10 twice.
///
/// -6, +3, +8, +5, -6 first reaches 5 twice.
///
/// +7, +7, -2, -7, -4 first reaches 14 twice.
///
/// What is the first frequency your device reaches twice?
///
/// # Arguments
///
/// * `file_name` - a path to the file with input data. Data example:
///
///  +13
///
/// -7
///
/// -17
///
/// +12
///
/// -11
///
/// +19
///
/// +18
///
/// +19
///
pub fn day1_task2(file_name: &str) -> i32 {
    let mut current_value = 0i32;
    let mut history = HashSet::new();
    for number in get_vec_int(file_name).iter().cycle() {
        current_value += number;
        if !history.insert(current_value) {
            break;
        }
    }
    current_value
}

//-------------------------------------------- day 2 --------------------------------------------//
///
/// # The task explanation
/// You see the following box IDs:
///
/// abcdef contains no letters that appear exactly two or three times.
///
/// bababc contains two a and three b, so it counts for both.
///
/// abbcde contains two b, but no letter appears exactly three times.
///
/// abcccd contains three c, but no letter appears exactly two times.
///
/// aabcdd contains two a and two d, but it only counts once.
///
/// abcdee contains two e.
///
/// ababab contains three a and three b, but it only counts once.
///
/// Of these box IDs, four of them contain a letter which appears exactly twice, and three of them contain a letter which appears exactly three times. Multiplying these together produces a checksum of 4 * 3 = 12.
///
/// What is the checksum for your list of box IDs?
///
/// # Arguments
///
/// * `file_name` - a path to the file with input data. Data example:
///
/// qwubbihrkplymcraxefntvdzns
///
/// qwugbihrkplyzcjahefttvdzns
///
/// qwugbihrkplymcjoxrsotvdzns
///
pub fn day2_task1(file_name: &str) -> u32 {
    let mut twice = 0u32;
    let mut thrice = 0u32;
    let input = read_input(file_name);
    for line in input {
        let l = line.expect("Line reading error!'");
        let mut is_twice_inserted = false;
        let mut is_thrice_inserted = false;
        for c in l.chars() {
            match l.matches(c).count() {
                2 => {
                    if !is_twice_inserted {
                        is_twice_inserted = true;
                        twice += 1;
                    }
                }
                3 => {
                    if !is_thrice_inserted {
                        is_thrice_inserted = true;
                        thrice += 1;
                    }
                }
                _ => {}
            };
        }
    }
    twice * thrice
}

///
/// # The task explanation
/// The boxes will have IDs which differ by exactly one character at the same position in
/// both strings. For example, given the following box IDs:
///
/// abcde
///
/// fghij
///
/// klmno
///
/// pqrst
///
/// fguij
///
/// axcye
///
/// wvxyz
///
/// The IDs abcde and axcye are close, but they differ by two characters (the second and fourth).
/// However, the IDs fghij and fguij differ by exactly one character, the third (h and u).
/// Those must be the correct boxes.
///
/// What letters are common between the two correct box IDs? (In the example above, this is
/// found by removing the differing character from either ID, producing fgij.)
///
/// # Arguments
///
/// * `file_name` - a path to the file with input data. Data example:
///
/// qwubbihrkplymcraxefntvdzns
///
/// qwugbihrkplyzcjahefttvdzns
///
/// qwugbihrkplymcjoxrsotvdzns
///
pub fn day2_task2(file_name: &str) -> String {
    let input = read_input(file_name)
        .map(|s| s.expect("Line reading error!'"))
        .collect::<Vec<String>>();
    let mut max_similar_count = 0u8;
    let mut similar_pair = (String::new(), String::new());
    for line1 in input.clone() {
        for line2 in input.clone() {
            if line1.eq(&line2) {
                continue;
            }
            let mut similar_count = 0u8;
            for (char1, char2) in line1.chars().zip(line2.chars()) {
                if char1.eq(&char2) {
                    similar_count += 1u8;
                }
            }
            if similar_count > max_similar_count {
                max_similar_count = similar_count;
                similar_pair = (line1.clone(), line2.clone());
            }
        }
    }
    let mut result = String::new();
    for (char1, char2) in similar_pair.0.chars().zip(similar_pair.1.chars()) {
        if char1.eq(&char2) {
            result.push(char1);
        }
    }
    result
}

//-------------------------------------------- day 3 --------------------------------------------//
#[macro_use]
extern crate lazy_static;
extern crate regex;
use regex::Regex;

const RECT_SIZE: usize = 1000;

lazy_static! {
    static ref RE_DAY3: Regex =
        Regex::new(r"^#(?P<id>\d+)\s@\s(?P<left>\d+),(?P<top>\d+):\s*(?P<wide>\d+)x(?P<toll>\d+)$")
            .unwrap();
}

///
/// Struct contains the inch id and it's rect x, y coordinates
///
#[derive(Debug, Clone, PartialEq)]
pub struct Inch {
    id: u32,
    min_x: u32,
    max_x: u32,
    min_y: u32,
    max_y: u32,
}

impl Inch {
    pub fn new(id: u32, left: u32, top: u32, wide: u32, toll: u32) -> Self {
        Inch {
            id,
            min_x: left,
            max_x: left + wide,
            min_y: top,
            max_y: top + toll,
        }
    }

    ///
    /// calculates overlap of two inches. returns boll value - does inches overlap or not.
    /// returns coordinates of the overlap rect
    ///
    fn get_overlap(&self, other: Inch) -> (bool, u32, u32, u32, u32) {
        let max_min_x = self.min_x.max(other.min_x);
        let min_max_x = self.max_x.min(other.max_x);
        let x = min_max_x as i32 - max_min_x as i32;
        let max_min_y = self.min_y.max(other.min_y);
        let min_max_y = self.max_y.min(other.max_y);
        let y = min_max_y as i32 - max_min_y as i32;
        (x > 0 && y > 0, max_min_x, min_max_x, max_min_y, min_max_y)
    }
}

///
/// Parse string to an Inch.
///
/// # Arguments
///
/// * `input` - a string for a parsing. Input data example: "#1318 @ 428,284: 25x21"
///
pub fn parse_inch(input: &str) -> Inch {
    let mut id = 0u32;
    let mut left = 0u32;
    let mut top = 0u32;
    let mut wide = 0u32;
    let mut toll = 0u32;
    if !RE_DAY3.is_match(input) {
        panic!("Cannot parse string '{}'", input);
    }
    for cap in RE_DAY3.captures_iter(input) {
        id = cap["id"]
            .parse::<u32>()
            .expect("DAY3: Cannot parse id value!");
        left = cap["left"]
            .parse::<u32>()
            .expect("DAY3: Cannot parse left value!");
        top = cap["top"]
            .parse::<u32>()
            .expect("DAY3: Cannot parse top value!");
        wide = cap["wide"]
            .parse::<u32>()
            .expect("DAY3: Cannot parse wide value!");
        toll = cap["toll"]
            .parse::<u32>()
            .expect("DAY3: Cannot parse toll value!");
    }
    Inch::new(id, left, top, wide, toll)
}

///
/// Parse data from file to a Vec of an Inches.
///
/// # Arguments
///
/// * `file_name` - a path to the file with input data. Data example:
///
/// #1 @ 249,597: 20x15
///
/// #2 @ 192,174: 10x21
///
/// #3 @ 734,527: 23x10
///
fn parse_inches(file_name: &str) -> Vec<Inch> {
    read_input(file_name)
        .map(|s| parse_inch(&s.expect("Line reading error!'")))
        .collect::<Vec<Inch>>()
}

///
/// Get count of true values in a vector of vectors
///
/// # Arguments
///
/// * `input` - a vector of vectors. Contains bool types only
///
fn collect_squares_count(input: &[[bool; RECT_SIZE]; RECT_SIZE]) -> u32 {
    let mut true_count = 0u32;
    for i in input.iter() {
        for &value in i.iter() {
            if value {
                true_count += 1;
            }
        }
    }
    true_count
}

///
/// # The task explanation
/// The problem is that many of the claims overlap, causing two or more claims to cover part of
/// the same areas. For example, consider the following claims:
///
/// #1 @ 1,3: 4x4
///
/// #2 @ 3,1: 4x4
///
/// #3 @ 5,5: 2x2
///
/// Visually, these claim the following areas:
///
/// ........
///
/// ...2222.
///
/// ...2222.
///
/// .11XX22.
///
/// .11XX22.
///
/// .111133.
///
/// .111133.
///
/// ........
///
/// The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to
/// the others, does not overlap either of them.)
///
/// If the Elves all proceed with their own plans, none of them will have enough fabric.
/// How many square inches of fabric are within two or more claims?
///
/// # Arguments
///
/// * `file_name` - a path to the file with input data. Data example:
///
/// #1 @ 249,597: 20x15
///
/// #2 @ 192,174: 10x21
///
/// #3 @ 734,527: 23x10
///
pub fn day3_task1(file_name: &str) -> u32 {
    let mut result = [[false; RECT_SIZE]; RECT_SIZE];
    let inches = parse_inches(file_name);
    for inch1 in inches.clone() {
        for inch2 in inches.clone() {
            if inch1 == inch2 {
                continue;
            }
            let (is_overlap, max_min_x, min_max_x, max_min_y, min_max_y) = inch1.get_overlap(inch2);
            if is_overlap {
                for x in max_min_x..min_max_x {
                    for y in max_min_y..min_max_y {
                        result[x as usize][y as usize] = true;
                    }
                }
            }
        }
    }
    collect_squares_count(&result)
}

///
/// Struct contains the vector of inches and the tag - is inch claimed or not
///
#[derive(Clone, PartialEq)]
struct InchClaimed {
    inch: Inch,
    claimed: bool,
}

impl InchClaimed {
    fn new(inch: Inch, claimed: bool) -> Self {
        InchClaimed { inch, claimed }
    }
}

///
/// # The task explanation
/// The problem is that many of the claims overlap, causing two or more claims to cover part of
/// the same areas. For example, consider the following claims:
///
/// #1 @ 1,3: 4x4
///
/// #2 @ 3,1: 4x4
///
/// #3 @ 5,5: 2x2
///
/// Visually, these claim the following areas:
///
/// ........
///
/// ...2222.
///
/// ...2222.
///
/// .11XX22.
///
/// .11XX22.
///
/// .111133.
///
/// .111133.
///
/// ........
///
/// The four square inches marked with X are claimed by both 1 and 2. (Claim 3, while adjacent to
/// the others, does not overlap either of them.)
///
/// What is the ID of the only claim that doesn't overlap?
///
/// # Arguments
///
/// * `file_name` - a path to the file with input data. Data example:
///
/// #1 @ 249,597: 20x15
///
/// #2 @ 192,174: 10x21
///
/// #3 @ 734,527: 23x10
///
pub fn day3_task2(file_name: &str) -> u32 {
    let inches = parse_inches(file_name);
    let mut list_of_inches = Vec::new();
    for inch in inches.iter() {
        let inch_claimed = InchClaimed::new(inch.clone(), false);
        list_of_inches.push(inch_claimed);
    }
    for i in 0..list_of_inches.len() {
        for y in 0..list_of_inches.len() {
            if list_of_inches[i].inch == list_of_inches[y].inch {
                continue;
            }
            let (is_overlap, _, _, _, _) = list_of_inches[i]
                .inch
                .get_overlap(list_of_inches[y].inch.clone());
            if is_overlap {
                list_of_inches[i].claimed = true;
                list_of_inches[y].claimed = true;
            }
        }
    }
    let mut id = 0;
    for inchcl in list_of_inches.iter() {
        if !inchcl.claimed {
            id = inchcl.inch.id
        }
    }
    id
}

//-------------------------------------------- day 4 --------------------------------------------//
//#[macro_use]
//extern crate lazy_static;
//extern crate regex;
//use regex::Regex;
extern crate chrono;

use chrono::{DateTime, FixedOffset, Timelike};

lazy_static! {
    static ref RE_DAY4: Regex =
        Regex::new(r"^\[(?P<year>\d+)-(?P<month>\d+)-(?P<day>\d+)\s+(?P<hour>\d+):(?P<minute>\d+)\]\s+(?P<action>.*)")
            .unwrap();
}

const MINUTE_BOOL: [bool; 60] = [false; 60];

#[derive(Debug)]
struct Message {
    year: u16,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    action: String,
    date_time: DateTime<FixedOffset>,
}

impl Message {
    fn new(year: u16, month: u8, day: u8, hour: u8, minute: u8, action: String) -> Self {
        // +0000 - is date time offset (FixedOffset)
        let date = format!(
            "{year}-{month}-{day} {hour}:{minute} +0000",
            year = year,
            month = month,
            day = day,
            hour = hour,
            minute = minute
        );
        let date_time = DateTime::parse_from_str(&date, "%Y-%m-%d %H:%M %z")
            .unwrap_or_else(|_| panic!("DAY4: Cannot parse date '{}'!", &date));
        Message {
            year,
            month,
            day,
            hour,
            minute,
            action,
            date_time,
        }
    }
}

#[derive(Debug, Clone)]
struct Guard {
    id: u16,
    sleep_duration: u16,
    start_sleep: u16,
    stop_sleep: u16,
    sleep_period: Vec<bool>,
}

impl Guard {
    fn new(id: u16, start_sleep: u32, stop_sleep: u32) -> Self {
        let mut sleep_period = MINUTE_BOOL;
        let sleep_duration = (stop_sleep - start_sleep) as u16;
        for i in 0..60 {
            if i >= start_sleep && i < stop_sleep {
                sleep_period[i as usize] = true;
            }
        }

        Guard {
            id,
            sleep_duration,
            start_sleep: start_sleep as u16,
            stop_sleep: (stop_sleep - 1u32) as u16,
            sleep_period: sleep_period.to_vec(),
        }
    }
}

pub fn day4_task1(file_name: &str) -> u32 {
    let mut messages = get_messages(file_name);
    messages.sort_by(|a, b| a.date_time.partial_cmp(&b.date_time).unwrap());
    let guards = get_guards_list(&messages);
    let mut result = HashMap::new();
    for guard in guards.clone() {
        let contains = result.contains_key(&guard.id);
        if contains {
            let duration = result[&guard.id];
            *result.get_mut(&guard.id).unwrap() = duration + guard.sleep_duration;
        } else {
            result.insert(guard.id, guard.sleep_duration);
        }
    }
    let (key, _) = result
        .iter()
        .max_by(|&(_, a), &(_, b)| a.partial_cmp(b).unwrap())
        .unwrap();
    let mut minute = [0; 60];
    for guard in guards {
        if guard.id.eq(key) {
            for (i, &item) in guard.sleep_period.iter().enumerate() {
                if item {
                    minute[i] += 1u32;
                }
            }
        }
    }
    let max_value = minute.iter().max().unwrap();
    let position = minute.iter().position(|el| el.eq(max_value)).unwrap();
    u32::from(*key) * (position as u32)
}

pub fn day4_task2(file_name: &str) -> u32 {
    let mut messages = get_messages(file_name);
    messages.sort_by(|a, b| a.date_time.partial_cmp(&b.date_time).unwrap());
    let guards = get_guards_list(&messages);
    let minutes = get_guards_minutes(&guards);
    let mut current_id = 0u16;
    let mut current_max = 0u16;
    let mut current_minute = 0usize;
    for (id, minute) in minutes {
        let max_value = *minute.iter().max().unwrap();
        if max_value > current_max {
            current_minute = minute.iter().position(|el| el.eq(&max_value)).unwrap();
            current_id = id;
            current_max = max_value;
        }
    }
    u32::from(current_id) * (current_minute as u32)
}

fn get_guards_minutes(guards: &[Guard]) -> HashMap<u16, Vec<u16>> {
    let mut minutes: HashMap<u16, Vec<u16>> = HashMap::new();
    for guard in guards {
        let mut minute = [0u16; 60];
        for (i, &item) in guard.sleep_period.clone().iter().enumerate() {
            if item {
                minute[i] += 1u16;
            }
        }
        let contains = minutes.contains_key(&guard.id);
        if contains {
            let res_mins: Vec<u16> = minutes[&guard.id]
                .iter()
                .zip(minute.iter())
                .map(|(x, y)| x + y)
                .collect();
            *minutes.get_mut(&guard.id).unwrap() = res_mins;
        } else {
            minutes.insert(guard.id, minute.to_vec());
        }
    }
    minutes
}

fn get_guards_list(messages: &[Message]) -> Vec<Guard> {
    let mut guards: Vec<Guard> = Vec::new();
    let mut index = 0;
    while index < messages.len() - 1 {
        if messages[index].action.starts_with("Guard") {
            let guard_id = get_guard_id(&messages[index].action);
            index += 1;
            while !messages[index].action.starts_with("Guard") {
                let start_sleep_minute = get_start_sleep_minute(messages[index].date_time);
                index += 1;
                let stop_sleep_minute = get_stop_sleep_minute(messages[index].date_time);
                index += 1;
                let guard = Guard::new(guard_id, start_sleep_minute, stop_sleep_minute);
                guards.push(guard);
                if index > messages.len() - 1 {
                    break;
                }
            }
        } else {
            panic!("Parse error!");
        }
    }
    guards
}

fn get_start_sleep_minute(date_time: DateTime<FixedOffset>) -> u32 {
    date_time.minute()
}

fn get_stop_sleep_minute(date_time: DateTime<FixedOffset>) -> u32 {
    date_time.minute()
}

// input example "Guard #751 begins shift"
fn get_guard_id(message: &str) -> u16 {
    message
        .chars()
        .filter(|x| x.is_numeric())
        .collect::<String>()
        .parse::<u16>()
        .expect("DAY4: Cannot parse guard id!")
}

fn get_messages(file_name: &str) -> Vec<Message> {
    read_input(file_name)
        .map(|s| parse_message(&s.expect("Line reading error!'")))
        .collect::<Vec<Message>>()
}

fn parse_message(input: &str) -> Message {
    let mut year: u16 = 0;
    let mut month: u8 = 0;
    let mut day: u8 = 0;
    let mut hour: u8 = 0;
    let mut minute: u8 = 0;
    let mut action: String = String::new();
    if !RE_DAY4.is_match(input) {
        panic!("Cannot parse string '{}'", input);
    }
    for cap in RE_DAY4.captures_iter(input) {
        year = cap["year"]
            .parse::<u16>()
            .expect("DAY4: Cannot parse year value!");
        month = cap["month"]
            .parse::<u8>()
            .expect("DAY4: Cannot parse month value!");
        day = cap["day"]
            .parse::<u8>()
            .expect("DAY4: Cannot parse day value!");
        hour = cap["hour"]
            .parse::<u8>()
            .expect("DAY4: Cannot parse hours value!");
        minute = cap["minute"]
            .parse::<u8>()
            .expect("DAY4: Cannot parse minute value!");
        action = cap["action"]
            .parse::<String>()
            .expect("DAY4: Cannot parse action value!");
    }
    Message::new(year, month, day, hour, minute, action)
}

//-------------------------------------------- day 5 --------------------------------------------//
pub fn day5_task1(file_name: &str) -> usize {
    let word = get_word(file_name);
    let bytes = word.as_bytes();
    remove_double(bytes)
}

pub fn day5_task2(file_name: &str) -> usize {
    let word = get_word(file_name);
    let alphabet = "abcdefghijklmnopqrstuvwxyz".chars();
    let mut result = std::usize::MAX;
    for letter in alphabet {
        let upper = letter.to_ascii_uppercase();
        let bytes: Vec<u8> = word
            .chars()
            .filter(|x| !(letter.eq(x) || upper.eq(x)))
            .map(|x| x as u8)
            .collect();
        let len = remove_double(bytes.as_slice());
        if len < result {
            result = len;
        }
    }
    result
}

fn remove_double(bytes: &[u8]) -> usize {
    let mut result: Vec<u8> = Vec::new();
    for &byte in bytes {
        let must_add = match result.last() {
            None => true,
            Some(&last) => {
                !(last != byte && last.to_ascii_lowercase() == byte.to_ascii_lowercase())
            }
        };

        if must_add {
            result.push(byte);
        } else {
            result.pop();
        }
    }
    result.len()
}

fn get_word(file_name: &str) -> String {
    read_input(file_name).next().unwrap().unwrap()
}
