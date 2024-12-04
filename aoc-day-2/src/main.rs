//! Couple thoughts here
//! (1) A colleague recommended me to try chain iterators more, which I did. No surprises here. Works as I would expect!
//! (2) Also wanted to define some first functions in Rust. No major surprises there either.
//! (3) I'm not a big fan of the docstring style yet. Probably will take some getting used to.
//! (4) Quick iteration can be a little tricky because Rust doesn't let you write scrappy code.
//!     Having to type my variables before I can quickly print them, especially with iterators, can be annoying.
//!

use std::env;
use std::fs;

/// Returns the sign of a number, either -1, 0, or 1.
/// I'm sure there's a crate for this, but I wanted to write my own functions for this one.
///
/// # Arguments
///
///  * `num` - The number to return the sign of.
///
fn get_sign(num: i8) -> i8 {
    if num < 0 {
        return -1;
    } else if num > 0 {
        return 1;
    } else {
        return 0;
    }
}

/// Initial call to check the report.
/// Will initialize the backtracking algorithm twice, once for each direction.
///
/// # Arguments
///
/// * `report` - The report to check.
///
fn check_report(report: &Vec<i8>) -> bool {
    return check_report_pairs(report, 0, 1, true, -1) || check_report_pairs(report, 0, 1, true, 1);
}

/// Backtracking algorithm to check if a report is safe, allowing a single entry to be skipped.
/// Recursively checks pairs from the report.
/// 
/// Took me a few tries to get this right.
///
/// # Arguments
///
/// * `report` - The report to check.
/// * `idx_1` - The first index of the pair.
/// * `idx_2` - The second index of the pair.
/// * `can_skip` - Whether we can skip still skip an entry.
/// * `direction` - The direction of the report, either -1 or 1.
///
fn check_report_pairs(
    report: &Vec<i8>,
    idx_1: usize,
    idx_2: usize,
    can_skip: bool,
    direction: i8,
) -> bool {
    if idx_2 == report.len() {
        return true; // Termination: Successfully reached the end of the report! This report is valid.
    }

    // Check the validity of the pair
    // A pair should have a difference from 1 to 3.
    // The sign should be consistent with the direction.
    let diff = report[idx_2] - report[idx_1];
    let direction_ok = get_sign(diff) == direction;
    let difference_ok = diff != 0 && diff.abs() <= 3;

    if !direction_ok || !difference_ok {
        if !can_skip {
            return false; // Termination: We can't skip anymore and this item is invalid.
        }

        if idx_1 == 0 {
            // Skipping the first item looks a little different than any subsequent item. We handle that here.
            // First try skipping the first item in the pair, then try skipping the second item in the pair.
            return check_report_pairs(report, idx_2, idx_2 + 1, false, direction)
                || check_report_pairs(report, idx_1, idx_2 + 1, false, direction);
        }

        // First try skipping the first item in the pair, then try skipping the second item in the pair.
        return check_report_pairs(report, idx_1 - 1, idx_2, false, direction)
            || check_report_pairs(report, idx_1, idx_2 + 1, false, direction);
    }

    // Don't skip anything and move on to the next pair
    return check_report_pairs(report, idx_2, idx_2 + 1, can_skip, direction);
}

/// Main function
/// Entrypoint of the application
fn main() {
    // Parse the input file
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let reports = contents.split("\n").map(|line| {
        let split = line.split_whitespace();
        return split.map(|part| {
            part.parse::<i8>()
                .expect("The part should be an unsigned integer")
        });
    });

    // ===== Part I =====

    // Find the differences between all neighboring pairs.
    let differences_between_consequent_pairs = reports.clone().map(|report| {
        return report
            .clone()
            .skip(1)
            .zip(report.clone())
            .map(|(a, b)| a - b);
    });

    let report_safety = differences_between_consequent_pairs.map(|mut differences| {
        let montonic_decrease = differences.clone().all(|diff| diff < 0);
        let montonic_increase = differences.clone().all(|diff| diff > 0);
        let monotonic = montonic_decrease || montonic_increase;

        let safe_differences = differences.all(|diff| diff.abs() <= 3);
        return (safe_differences && monotonic) as u16;
    });

    let count: u16 = report_safety.clone().sum();
    println!("The number of safe reports is: {count}");

    // ===== Part II =====

    // We'll skip all reports that are already safe.
    let potentially_safe_reports = reports.zip(report_safety);
    let potentially_safe_reports = potentially_safe_reports.filter(|(_, is_safe)| *is_safe == 0);
    let potentially_safe_reports = potentially_safe_reports.map(|(report, _)| report);

    let total_to_check = potentially_safe_reports.clone().count();
    println!("The number of reports to check again {total_to_check}");

    // Backtracking algorithm to check the reports again.
    let refine_report_safety = potentially_safe_reports.filter(|report| {
        let report_data: Vec<i8> = report.clone().collect();
        return check_report(&report_data);
    });

    // Final result
    let refined_count = refine_report_safety.count();
    let total_count = refined_count + count as usize;
    println!("The refined number of safe reports is: {count} + {refined_count} = {total_count}");
}
