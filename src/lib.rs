use core::fmt;
use std::collections::HashSet;
use std::io;
use std::io::Write;
use time::PrimitiveDateTime as DateTime;
pub fn print_hello_world() {
    let _ = io::stdout().write_all(b"Hello, world!\n");
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
    dbg!(s.chars().rev().collect())
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
        dbg!(total_minutes);
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

// pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&str]) -> HashSet<&'a str> {
//     todo!("For the '{word}' word find anagrams among the following words: {possible_anagrams:?}");
// }

#[cfg(test)]
mod tests {
    use super::*;
    // #[test]

    // fn no_matches() {
    //     let word = "diaper";
    //     let inputs = &["hello", "world", "zombies", "pants"];
    //     let output = anagrams_for(word, inputs);
    //     let expected = HashSet::from_iter([]);
    //     assert_eq!(output, expected);
    // }

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
