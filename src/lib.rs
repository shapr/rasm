use core::fmt;
use std::io;
use std::io::Write;
use time::PrimitiveDateTime as DateTime;
use unicode_segmentation::UnicodeSegmentation;
pub fn print_hello_world() {
    let _ = io::stdout().write_all(b"Hello, world!\n");
}

pub fn is_armstrong_number(num: u32) -> bool {
    let better_num = u64::from(num);
    // turn num into string
    // get length of string to find exponent
    let exponent: u32 = better_num.to_string().len().try_into().unwrap();
    // map that over each digit in the string
    // convert characters in strong to separate digits
    let digits: Vec<u64> = better_num
	.to_string()
	.chars()
	.map(|c| u64::from(c.to_digit(10).unwrap()))
	.collect();
    let arm_it: u64 = digits.iter().map(|n| n.pow(exponent)).sum::<u64>();
    better_num == arm_it
}

pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    for (i, v) in nums.iter().enumerate() {
	if let Some(found) = nums[i + 1..].iter().position(|&x| x == target - v) {
	    return vec![i.try_into().unwrap(), (1 + i + found).try_into().unwrap()];
	} else {
	    continue;
	}
    }
    return vec![];
}

// SUCCESS!
pub fn after(start: DateTime) -> DateTime {
    let gigasecond = time::Duration::new(1_000_000_000, 0);
    start.checked_add(gigasecond).unwrap()
}

pub fn string_rev(s: &str) -> String {
    s.graphemes(true).rev().collect()
}

#[derive(Debug, PartialEq)]
pub struct Clock {
    h: i32,
    m: i32,
}

impl Clock {
    pub fn new(hours: i32, minutes: i32) -> Self {
	let mut c = Clock {
	    h: hours,
	    m: minutes,
	};
	c.normalize_clock();
	c
    }

    pub fn add_minutes(mut self, minutes: i32) -> Self {
	self.m += minutes;
	self.normalize_clock();
	self
    }

    pub fn normalize_clock(&mut self) {
	// convert hours to minutes
	let mut total_minutes = self.h * 60;
	while total_minutes < 0 {
	    // if negative, subtract from 24 hours of minutes
	    total_minutes = (24 * 60) + total_minutes;
	}
	total_minutes += self.m;
	while total_minutes < 0 {
	    // if negative, subtract from 24 hours of minutes
	    total_minutes = (24 * 60) + total_minutes;
	}
	total_minutes = total_minutes % (24 * 60); // no more than 24 hours total

	self.h = total_minutes / 60; // update hours first
	self.m = total_minutes % 60; // mod minutes
    }
}
impl fmt::Display for Clock {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
	write!(f, "{:0>2}:{:0>2}", self.h, self.m)
    }
}

pub fn egg_count(display_value: u32) -> usize {
    todo!("count the eggs in {display_value}")
}

pub fn verse(n: u32) -> String {
    match n {
	0 => "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n".to_string(),
	1 => "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n".to_string(),
	2 => "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n".to_string(),
	3..99 => format!("{n} bottles of beer on the wall, {n} bottles of beer.\nTake one down and pass it around, {} bottles of beer on the wall.\n",n-1),
	_ => todo!()
    }
}

pub fn sing(start: u32, end: u32) -> String {
    (end..=start)
	.rev()
	.map(verse)
	.collect::<Vec<_>>()
	.join("\n")
}

pub fn raindrops(n: u32) -> String {
    let mut result = String::new();

    if n % 3 == 0 {
	result += "Pling";
    }
    if n % 5 == 0 {
	result += "Plang";
    }
    if n % 7 == 0 {
	result += "Plong";
    }
    if result.is_empty() {
	result = n.to_string();
    }
    result
}

#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    /* first guess is that I check one element against all the elements in the other set, if one is equal, this item does not contribute.
    if there isn't one that's equal, this set could be a super set
    I guess I need a "for each value, is this value in the other set" sort of result. */
    // start with the shortest list
    // check to see if
    if _first_list == _second_list {
	return Comparison::Equal;
    }
    let first: &[T];
    let second: &[T];
    let mut swapped = false;
    if _first_list.len() <= _second_list.len() {
	first = _first_list;
	second = _second_list;
    } else {
	first = _second_list;
	second = _first_list;
	swapped = true;
    }
    let mut matches = vec![];
    for item in first {
	matches.push(second.iter().any(|i2| i2 == item));
    }

    let shorter_is_sublist = matches.iter().all(|f| *f);
    if shorter_is_sublist {
	if swapped {
	    return Comparison::Superlist;
	} else {
	    return Comparison::Sublist;
	}
    } else {
	return Comparison::Unequal;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_lists() {
	let list_one: &[i32] = &[];
	let list_two: &[i32] = &[];
	let output = sublist(list_one, list_two);
	let expected = Comparison::Equal;
	assert_eq!(output, expected);
    }

    #[test]
    fn empty_list_within_non_empty_list() {
	let list_one: &[i32] = &[];
	let list_two: &[i32] = &[1, 2, 3];
	let output = sublist(list_one, list_two);
	let expected = Comparison::Sublist;
	assert_eq!(output, expected);
    }

    #[test]
    fn non_empty_list_contains_empty_list() {
	let list_one: &[i32] = &[1, 2, 3];
	let list_two: &[i32] = &[];
	let output = sublist(list_one, list_two);
	let expected = Comparison::Superlist;
	assert_eq!(output, expected);
    }

    #[test]
    fn list_equals_itself() {
	let list_one: &[i32] = &[1, 2, 3];
	let list_two: &[i32] = &[1, 2, 3];
	let output = sublist(list_one, list_two);
	let expected = Comparison::Equal;
	assert_eq!(output, expected);
    }

    #[test]
    fn different_lists() {
	let list_one: &[i32] = &[1, 2, 3];
	let list_two: &[i32] = &[2, 3, 4];
	let output = sublist(list_one, list_two);
	let expected = Comparison::Unequal;
	assert_eq!(output, expected);
    }

    #[test]
    fn first_list_missing_element_from_second_list() {
	let list_one: &[i32] = &[1, 3];
	let list_two: &[i32] = &[1, 2, 3];
	let output = sublist(list_one, list_two);
	let expected = Comparison::Unequal;
	assert_eq!(output, expected);
    }
    #[test]
    fn test_the_sound_for_1_is_1() {
	let input = 1;
	let output = raindrops(input);
	let expected = "1";
	assert_eq!(output, expected);
    }

    #[test]
    fn test_the_sound_for_3_is_pling() {
	let input = 3;
	let output = raindrops(input);
	let expected = "Pling";
	assert_eq!(output, expected);
    }

    #[test]
    fn test_the_sound_for_5_is_plang() {
	let input = 5;
	let output = raindrops(input);
	let expected = "Plang";
	assert_eq!(output, expected);
    }

    #[test]
    fn verse_0() {
	assert_eq!(verse(0), "No more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }

    #[test]
    fn verse_1() {
	assert_eq!(verse(1), "1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n");
    }

    #[test]
    fn verse_2() {
	assert_eq!(verse(2), "2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n");
    }

    #[test]
    fn verse_8() {
	assert_eq!(verse(8), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n");
    }
    #[test]
    fn song_8_6() {
	assert_eq!(sing(8, 6), "8 bottles of beer on the wall, 8 bottles of beer.\nTake one down and pass it around, 7 bottles of beer on the wall.\n\n7 bottles of beer on the wall, 7 bottles of beer.\nTake one down and pass it around, 6 bottles of beer on the wall.\n\n6 bottles of beer on the wall, 6 bottles of beer.\nTake one down and pass it around, 5 bottles of beer on the wall.\n");
    }

    #[test]
    fn song_3_0() {
	assert_eq!(sing(3, 0), "3 bottles of beer on the wall, 3 bottles of beer.\nTake one down and pass it around, 2 bottles of beer on the wall.\n\n2 bottles of beer on the wall, 2 bottles of beer.\nTake one down and pass it around, 1 bottle of beer on the wall.\n\n1 bottle of beer on the wall, 1 bottle of beer.\nTake it down and pass it around, no more bottles of beer on the wall.\n\nNo more bottles of beer on the wall, no more bottles of beer.\nGo to the store and buy some more, 99 bottles of beer on the wall.\n");
    }

    #[test]
    fn zero_is_an_armstrong_number() {
	assert!(is_armstrong_number(0))
    }

    #[test]
    fn single_digit_numbers_are_armstrong_numbers() {
	assert!(is_armstrong_number(5))
    }

    #[test]
    fn test_rev_string() {
	assert_eq!(string_rev("uüu"), "uüu");
    }

    #[test]
    fn foo() {
	assert_eq!(two_sum(vec![1, 2], 3), vec![0, 1]);
	assert_eq!(two_sum(vec![3, 3], 6), vec![0, 1]);
    }
    #[test]
    fn rev_string() {
	assert_eq!(string_rev("racecar"), "racecar");
	assert_eq!(string_rev("desserts"), "stressed");
    }
    #[test]
    fn clock_checks() {
	assert_eq!(Clock::new(1, 1), Clock { h: 1, m: 1 });
	assert_eq!(Clock::new(0, 61), Clock { h: 1, m: 1 });
	assert_eq!(Clock::new(-2, 40), Clock { h: 22, m: 40 });
	assert_eq!(Clock::new(-16, -28), Clock { h: 7, m: 32 });
	assert_eq!(Clock::new(-6, -57), Clock { h: 17, m: 3 });
	assert_eq!(Clock::new(-10, -11), Clock { h: 13, m: 49 });
	assert_eq!(Clock::new(0, -22), Clock { h: 23, m: 38 });
	assert_eq!(Clock::new(-5, -53), Clock { h: 18, m: 7 });
	assert_eq!(Clock::new(-23, -40), Clock { h: 00, m: 20 });
	assert_eq!(Clock::new(-7, -20), Clock { h: 16, m: 40 });
	assert_eq!(Clock::new(-19, 00), Clock { h: 5, m: 00 });
	assert_eq!(Clock::new(-17, -45), Clock { h: 6, m: 15 });
    }

    #[test]

    fn on_the_hour() {
	assert_eq!(Clock::new(8, 0).to_string(), "08:00");
    }

    #[test]

    fn past_the_hour() {
	assert_eq!(Clock::new(11, 9).to_string(), "11:09");
    }

    #[test]

    fn midnight_is_zero_hours() {
	assert_eq!(Clock::new(24, 0).to_string(), "00:00");
    }

    #[test]

    fn hour_rolls_over() {
	assert_eq!(Clock::new(25, 0).to_string(), "01:00");
    }

    #[test]

    fn hour_rolls_over_continuously() {
	assert_eq!(Clock::new(100, 0).to_string(), "04:00");
    }

    #[test]

    fn sixty_minutes_is_next_hour() {
	assert_eq!(Clock::new(1, 60).to_string(), "02:00");
    }

    #[test]

    fn minutes_roll_over() {
	assert_eq!(Clock::new(0, 160).to_string(), "02:40");
    }

    #[test]

    fn minutes_roll_over_continuously() {
	assert_eq!(Clock::new(0, 1723).to_string(), "04:43");
    }

    #[test]

    fn hours_and_minutes_roll_over() {
	assert_eq!(Clock::new(25, 160).to_string(), "03:40");
    }

    #[test]

    fn hours_and_minutes_roll_over_continuously() {
	assert_eq!(Clock::new(201, 3001).to_string(), "11:01");
    }

    #[test]

    fn hours_and_minutes_roll_over_to_exactly_midnight() {
	assert_eq!(Clock::new(72, 8640).to_string(), "00:00");
    }

    #[test]

    fn negative_hour() {
	assert_eq!(Clock::new(-1, 15).to_string(), "23:15");
    }

    #[test]

    fn negative_hour_roll_over() {
	assert_eq!(Clock::new(-25, 00).to_string(), "23:00");
    }

    #[test]

    fn negative_hour_roll_over_continuously() {
	assert_eq!(Clock::new(-91, 00).to_string(), "05:00");
    }

    #[test]

    fn negative_minutes() {
	assert_eq!(Clock::new(1, -40).to_string(), "00:20");
    }

    #[test]

    fn negative_minutes_roll_over() {
	assert_eq!(Clock::new(1, -160).to_string(), "22:20");
    }

    #[test]

    fn negative_minutes_roll_over_continuously() {
	assert_eq!(Clock::new(1, -4820).to_string(), "16:40");
    }

    #[test]

    fn negative_sixty_minutes_is_prev_hour() {
	assert_eq!(Clock::new(2, -60).to_string(), "01:00");
    }

    #[test]

    fn negative_one_twenty_minutes_is_two_prev_hours() {
	assert_eq!(Clock::new(1, -120).to_string(), "23:00");
    }

    #[test]

    fn negative_hour_and_minutes_both_roll_over() {
	assert_eq!(Clock::new(-25, -160).to_string(), "20:20");
    }

    #[test]

    fn negative_hour_and_minutes_both_roll_over_continuously() {
	assert_eq!(Clock::new(-121, -5810).to_string(), "22:10");
    }

    #[test]

    fn zero_hour_and_negative_minutes() {
	assert_eq!(Clock::new(0, -22).to_string(), "23:38");
    }

    //

    // Clock Math

    //

    #[test]

    fn add_minutes() {
	let clock = Clock::new(10, 0).add_minutes(3);

	assert_eq!(clock.to_string(), "10:03");
    }

    #[test]

    fn add_no_minutes() {
	let clock = Clock::new(6, 41).add_minutes(0);

	assert_eq!(clock.to_string(), "06:41");
    }

    #[test]

    fn add_to_next_hour() {
	let clock = Clock::new(0, 45).add_minutes(40);

	assert_eq!(clock.to_string(), "01:25");
    }

    #[test]

    fn add_more_than_one_hour() {
	let clock = Clock::new(10, 0).add_minutes(61);

	assert_eq!(clock.to_string(), "11:01");
    }

    #[test]

    fn add_more_than_two_hours_with_carry() {
	let clock = Clock::new(0, 45).add_minutes(160);

	assert_eq!(clock.to_string(), "03:25");
    }

    #[test]

    fn add_across_midnight() {
	let clock = Clock::new(23, 59).add_minutes(2);

	assert_eq!(clock.to_string(), "00:01");
    }

    #[test]

    fn add_more_than_one_day() {
	let clock = Clock::new(5, 32).add_minutes(1500);

	assert_eq!(clock.to_string(), "06:32");
    }

    #[test]

    fn add_more_than_two_days() {
	let clock = Clock::new(1, 1).add_minutes(3500);

	assert_eq!(clock.to_string(), "11:21");
    }

    #[test]

    fn subtract_minutes() {
	let clock = Clock::new(10, 3).add_minutes(-3);

	assert_eq!(clock.to_string(), "10:00");
    }

    #[test]

    fn subtract_to_previous_hour() {
	let clock = Clock::new(10, 3).add_minutes(-30);

	assert_eq!(clock.to_string(), "09:33");
    }

    #[test]

    fn subtract_more_than_an_hour() {
	let clock = Clock::new(10, 3).add_minutes(-70);

	assert_eq!(clock.to_string(), "08:53");
    }

    #[test]

    fn subtract_across_midnight() {
	let clock = Clock::new(0, 3).add_minutes(-4);

	assert_eq!(clock.to_string(), "23:59");
    }

    #[test]

    fn subtract_more_than_two_hours() {
	let clock = Clock::new(0, 0).add_minutes(-160);

	assert_eq!(clock.to_string(), "21:20");
    }

    #[test]

    fn subtract_more_than_two_hours_with_borrow() {
	let clock = Clock::new(6, 15).add_minutes(-160);

	assert_eq!(clock.to_string(), "03:35");
    }

    #[test]

    fn subtract_more_than_one_day() {
	let clock = Clock::new(5, 32).add_minutes(-1500);

	assert_eq!(clock.to_string(), "04:32");
    }

    #[test]

    fn subtract_more_than_two_days() {
	let clock = Clock::new(2, 20).add_minutes(-3000);

	assert_eq!(clock.to_string(), "00:20");
    }

    //

    // Test Equality

    //

    #[test]

    fn compare_clocks_for_equality() {
	assert_eq!(Clock::new(15, 37), Clock::new(15, 37));
    }

    #[test]

    fn compare_clocks_a_minute_apart() {
	assert_ne!(Clock::new(15, 36), Clock::new(15, 37));
    }

    #[test]

    fn compare_clocks_an_hour_apart() {
	assert_ne!(Clock::new(14, 37), Clock::new(15, 37));
    }

    #[test]

    fn compare_clocks_with_hour_overflow() {
	assert_eq!(Clock::new(10, 37), Clock::new(34, 37));
    }

    #[test]

    fn compare_clocks_with_hour_overflow_by_several_days() {
	assert_eq!(Clock::new(99, 11), Clock::new(3, 11));
    }

    #[test]

    fn compare_clocks_with_negative_hour() {
	assert_eq!(Clock::new(-2, 40), Clock::new(22, 40));
    }

    #[test]

    fn compare_clocks_with_negative_hour_that_wraps() {
	assert_eq!(Clock::new(-31, 3), Clock::new(17, 3));
    }

    #[test]

    fn compare_clocks_with_negative_hour_that_wraps_multiple_times() {
	assert_eq!(Clock::new(-83, 49), Clock::new(13, 49));
    }

    #[test]

    fn compare_clocks_with_minutes_overflow() {
	assert_eq!(Clock::new(0, 1441), Clock::new(0, 1));
    }

    #[test]

    fn compare_clocks_with_minutes_overflow_by_several_days() {
	assert_eq!(Clock::new(2, 4322), Clock::new(2, 2));
    }

    #[test]

    fn compare_clocks_with_negative_minute() {
	assert_eq!(Clock::new(3, -20), Clock::new(2, 40));
    }

    #[test]

    fn compare_clocks_with_negative_minute_that_wraps() {
	assert_eq!(Clock::new(5, -1490), Clock::new(4, 10));
    }

    #[test]

    fn compare_clocks_with_negative_minute_that_wraps_multiple() {
	assert_eq!(Clock::new(6, -4305), Clock::new(6, 15));
    }

    #[test]

    fn compare_clocks_with_negative_hours_and_minutes() {
	assert_eq!(Clock::new(-12, -268), Clock::new(7, 32));
    }

    #[test]

    fn compare_clocks_with_negative_hours_and_minutes_that_wrap() {
	assert_eq!(Clock::new(-54, -11_513), Clock::new(18, 7));
    }

    #[test]

    fn compare_full_clock_and_zeroed_clock() {
	assert_eq!(Clock::new(24, 0), Clock::new(0, 0));
    }
}
