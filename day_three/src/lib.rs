struct NumberBox {
    x_min: usize,
    x_max: usize,
    y: usize,
    number: u32
}

struct Symbol {
    x: usize,
    y: usize,
    symbol: char
}

pub fn part_one(input: &str) -> u32 {
    let mut number_boxes: Vec<NumberBox> = Vec::new();
    let mut symbols: Vec<Symbol> = Vec::new();
    for (y, line) in input.lines().enumerate() {
        let mut current_number: u32 = 0;
        let mut parsing_number = false;
        let mut x_min = 0;
        for (x, character) in line.chars().enumerate() {
            if character.is_ascii_digit() {
                if !parsing_number {
                    x_min = x;
                    parsing_number = true;
                    current_number = 0;
                }
                current_number = current_number * 10 + character.to_digit(10).unwrap();
            } else {
                if parsing_number {
                    number_boxes.push(NumberBox{x_min, x_max: x-1, y, number: current_number});
                    parsing_number = false;
                }
                if character != '.' {
                    symbols.push(Symbol { x, y, symbol: character })
                }
            }
        }
        if parsing_number {
            number_boxes.push(NumberBox{x_min, x_max: line.len()-1, y, number: current_number});
            parsing_number = false;
        }
    }
    number_boxes.iter()
    .map(|num_box| {
        if symbols.iter().any(|symbol| {
            if symbol.x > num_box.x_max + 1 {
                return false;
            }
            if num_box.x_min > 0 && symbol.x < num_box.x_min - 1 {
                return false;
            }
            if symbol.y > num_box.y + 1 {
                return false;
            }
            if num_box.y > 0 && symbol.y < num_box.y - 1 {
                return false;
            }
            return true;
        }) {
            return num_box.number;
        }
        0
    })
    .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use indoc::indoc;

    fn test_input() -> &'static str {
        indoc! {"467..114..
                ...*......
                ..35..633.
                ......#...
                617*......
                .....+.58.
                ..592.....
                ......755.
                ...$.*....
                .664.598.."}
    }

    #[test]
    fn it_calculates_the_sum_of_part_numbers() {
        let result = part_one(test_input());
        assert_eq!(result, 4361);
    }
}
