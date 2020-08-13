use failure::Error;

use std::collections::HashSet;
use std::fs;

#[derive(Debug)]
pub struct Flags {
    case_sensitive: bool,
    collect_matches: bool,
    only_matching_files: bool,
    only_whole_lines: bool,
    print_line_numbers: bool,
}

impl Flags {
    pub fn new(flags: &[&str]) -> Self {
        let mut case_sensitive = true;
        let mut collect_matches = true;
        let mut only_matching_files = false;
        let mut only_whole_lines = false;
        let mut print_line_numbers = false;

        for flag in flags {
            match *flag {
                "-i" => case_sensitive = false,
                "-l" => only_matching_files = true,
                "-n" => print_line_numbers = true,
                "-v" => collect_matches = false,
                "-x" => only_whole_lines = true,
                _ => (),
            }
        }

        Flags {
            case_sensitive,
            collect_matches,
            only_matching_files,
            only_whole_lines,
            print_line_numbers,
        }
    }
}

pub fn grep(pattern: &str, flags: &Flags, files: &[&str]) -> Result<Vec<String>, Error> {
    let mut matched_lines: Vec<String> = Vec::new();
    let mut matched_files: HashSet<String> = HashSet::new();
    let mut files_order: Vec<String> = Vec::new();

    for file_name in files {
        for (line_number, line) in fs::read_to_string(file_name)?.lines().enumerate() {
            let is_match =
                is_line_match(line, pattern, flags.case_sensitive, flags.only_whole_lines);

            if (flags.collect_matches && is_match) || (!flags.collect_matches && !is_match) {
                if matched_files.insert(file_name.to_string()) {
                    files_order.push(file_name.to_string());
                }
                let matched_line = display_matched_line(
                    line,
                    line_number + 1,
                    match files.len() {
                        n if n > 1 => Some(file_name),
                        _ => None,
                    },
                    flags.print_line_numbers,
                );

                matched_lines.push(matched_line)
            }
        }
    }

    match flags.only_matching_files {
        true => Ok(files_order),
        false => Ok(matched_lines),
    }
}

fn is_line_match(line: &str, pattern: &str, case_sensitive: bool, only_whole_lines: bool) -> bool {
    let match_line: String = match case_sensitive {
        true => line.to_string(),
        false => line.to_lowercase(),
    };

    let pattern: String = match case_sensitive {
        true => pattern.to_string(),
        false => pattern.to_lowercase(),
    };

    match only_whole_lines {
        true => match_line == pattern,
        false => match_line.contains(pattern.as_str()),
    }
}

fn display_matched_line(
    line: &str,
    line_number: usize,
    file_name: Option<&str>,
    print_line_numbers: bool,
) -> String {
    let mut matched_line: String = String::new();

    if let Some(file_name) = file_name {
        matched_line.push_str(file_name);
        matched_line.push_str(":");
    }

    if print_line_numbers {
        matched_line.push_str(line_number.to_string().as_str());
        matched_line.push_str(":");
    }

    matched_line.push_str(line);

    matched_line
}
