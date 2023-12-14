struct NumberBox {
    x_min: usize,
    x_max: usize,
    y: usize,
    number: u32
}

impl NumberBox {
    fn is_adjacent_to(&self, x: usize, y: usize) -> bool {
        if x > self.x_max + 1 {
            return false;
        }
        if self.x_min > 0 && x < self.x_min - 1 {
            return false;
        }
        if y > self.y + 1 {
            return false;
        }
        if self.y > 0 && y < self.y - 1 {
            return false;
        }
        true
    }
}

struct Symbol {
    x: usize,
    y: usize,
    symbol: char
}

pub fn part_one(input: &str) -> u32 {
    let (number_boxes, symbols) = parse_input(input);
    number_boxes.iter()
    .map(|num_box| {
        if symbols.iter().any(|symbol| {
            num_box.is_adjacent_to(symbol.x, symbol.y)
        }) {
            return num_box.number;
        }
        0
    })
    .sum()
}

pub fn part_two(input: &str) -> u32 {
    let (number_boxes, symbols) = parse_input(input);
    symbols.iter()
    .map(|symbol| {
        if symbol.symbol != '*' {
            return 0;
        }
        let adjacent_boxes: Vec<_> = number_boxes.iter().filter(|num_box| {
            num_box.is_adjacent_to(symbol.x, symbol.y)
        }).collect();
        if adjacent_boxes.len() != 2 {
            return 0;
        }
        adjacent_boxes[0].number * adjacent_boxes[1].number
    })
    .sum()
}

fn parse_input(input: &str) -> (Vec<NumberBox>, Vec<Symbol>) {
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
        }
    }
    (number_boxes, symbols)
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

    #[test]
    fn it_calculates_the_sum_of_adjacent_gear_products() {
        let result = part_two(test_input());
        assert_eq!(result, 467835);
    }
}
