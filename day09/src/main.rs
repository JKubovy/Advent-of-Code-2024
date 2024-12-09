use std::iter;
use utils::StrParser;

type FileEmpty = (usize, usize);
#[derive(Clone, Copy)]
struct File {
    pub size: usize,
    pub index: usize,
}

fn parse_input(input: &str) -> Vec<FileEmpty> {
    let mut result = Vec::new();
    let mut chars = input.chars();
    while let Some(c) = chars.next() {
        let first = c.parse_usize();
        let second = if let Some(next_char) = chars.next() {
            next_char
                .to_digit(10)
                .expect("Invalid character, not a digit") as usize
        } else {
            0
        };
        result.push((first, second));
    }

    result
}

fn expand_and_defragment(data: &[FileEmpty]) -> Vec<usize> {
    let mut result = vec![];
    if data.len() <= 1 {
        return result;
    }
    let mut index = 0;
    let mut last_index = data.len() - 1;
    let (mut files, mut empty) = data.get(index).unwrap();
    let (mut files_last, _) = data.get(last_index).unwrap();
    while index < last_index {
        result.extend(iter::repeat_n(index, files));
        files = 0;
        if files_last == 0 {
            last_index -= 1;
            (files_last, _) = *data.get(last_index).unwrap();
            continue;
        }
        if empty > files_last {
            result.extend(iter::repeat_n(last_index, files_last));
            empty -= files_last;
            last_index -= 1;
            (files_last, _) = *data.get(last_index).unwrap();
            continue;
        } else {
            result.extend(iter::repeat_n(last_index, empty));
            files_last -= empty;
            index += 1;
            (files, empty) = *data.get(index).unwrap()
        }
    }
    if files_last > 0 {
        result.extend(iter::repeat_n(last_index, files_last));
    }

    result
}

fn expand_and_move_files(data: &[FileEmpty]) -> Vec<usize> {
    let mut data = data
        .iter()
        .enumerate()
        .map(|(index, file_empty)| {
            (
                0,
                vec![File {
                    size: file_empty.0,
                    index,
                }],
                file_empty.1,
            )
        })
        .collect::<Vec<_>>();
    if data.len() <= 1 {
        return vec![];
    }
    let mut last_index = data.len() - 1;
    while last_index > 0 {
        let (_, files, _) = data.get(last_index).unwrap().clone();
        let file = *files.first().unwrap();
        let mut index = 0;
        let mut success = false;
        while index < last_index {
            let (begin_empty, mut begin_files, end_empty) = data.get(index).unwrap().clone();
            if end_empty >= file.size {
                begin_files.push(file);
                success = true;
                data[index] = (begin_empty, begin_files, end_empty - file.size);
                break;
            }
            index += 1;
        }
        if success {
            let tmp = data.get_mut(last_index).unwrap();
            tmp.1.remove(0);
            data[last_index] = (tmp.0 + file.size, tmp.1.clone(), tmp.2);
        }
        last_index -= 1;
    }
    data.iter()
        .fold(vec![], |mut data, (begin_empty, files, end_empty)| {
            data.extend(iter::repeat_n(0, *begin_empty));
            for file in files {
                data.extend(iter::repeat_n(file.index, file.size));
            }
            data.extend(iter::repeat_n(0, *end_empty));
            data
        })
}

fn first_part(input: &str) -> usize {
    let data = parse_input(input);
    let data = expand_and_defragment(&data);
    data.iter()
        .enumerate()
        .fold(0, |sum, (file_index, block_index)| {
            sum + file_index * *block_index
        })
}

fn second_part(input: &str) -> usize {
    let data = parse_input(input);
    let data = expand_and_move_files(&data);
    data.iter()
        .enumerate()
        .fold(0, |sum, (file_index, block_index)| {
            sum + file_index * *block_index
        })
}

fn main() {
    let input = include_str!("../inputs/input.txt");
    let first_part = first_part(input);
    println!("First part: {}", first_part);
    let second_part = second_part(input);
    println!("Second part: {}", second_part);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_part() {
        let data = include_str!("../inputs/test.txt");
        let result = first_part(data);
        assert_eq!(result, 1928);
    }

    #[test]
    fn input_first_part() {
        let data = include_str!("../inputs/input.txt");
        let result = first_part(data);
        assert_eq!(result, 6435922584968);
    }

    #[test]
    fn test_second_part() {
        let data = include_str!("../inputs/test.txt");
        let result = second_part(data);
        assert_eq!(result, 2858);
    }

    #[test]
    fn input_second_part() {
        let data = include_str!("../inputs/input.txt");
        let result = second_part(data);
        assert_eq!(result, 6469636832766);
    }
}
