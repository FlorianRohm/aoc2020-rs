use std::convert::TryFrom;

pub fn solve() -> (usize, usize) {
    let input = include_str!("./input");

    let tree_hits_3_1 = count_tree_hits(&input, 3, 1);


    let tree_hits_1_1 = count_tree_hits(&input, 1, 1);
    let tree_hits_5_1 = count_tree_hits(&input, 5, 1);
    let tree_hits_7_1 = count_tree_hits(&input, 7, 1);
    let tree_hits_1_2 = count_tree_hits(&input, 1, 2);

    let tree_hit_score = tree_hits_1_1 * tree_hits_1_2 * tree_hits_3_1 * tree_hits_5_1 * tree_hits_7_1;

    (tree_hits_3_1, tree_hit_score)
}

#[derive(Eq, PartialEq, Debug)]
enum MapItem {
    Tree,
    Snow,
}

impl TryFrom<char> for MapItem {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(MapItem::Snow),
            '#' => Ok(MapItem::Tree),
            _ => Err(())
        }
    }
}

fn count_tree_hits(base_map: &str, right: usize, down: usize) -> usize {
    let map: Vec<Vec<MapItem>> = base_map.lines().into_iter()
        .map(|line| line.chars().into_iter()
            .map(|c| MapItem::try_from(c).expect(&format!("Could not parse {}", c)))
            .collect())
        .collect();
    let mut tree_counter = 0;

    let mut column_index = 0;
    for row in map.iter().filter(|r| !r.is_empty()).step_by(down) {
        column_index %= row.len();
        match row.get(column_index) {
            Some(tile) => { if *tile == MapItem::Tree { tree_counter += 1; } }
            None => {
                panic!("something went horribly wrong")
            }
        }
        column_index += right;
    }

    tree_counter
}

#[cfg(test)]
mod tests {
    use std::intrinsics::size_of;

    use super::*;

    #[test]
    fn should_solve() {
        assert_eq!(solve(), (244, 9406609920))
    }

    const TEST_INPUT: &str = "
..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#
";

    #[test]
    fn should_count_test_input_default_slope() {
        assert_eq!(count_tree_hits(TEST_INPUT, 3, 1), 7);
    }

    #[test]
    fn should_count_test_input_slope_1_1() {
        assert_eq!(count_tree_hits(TEST_INPUT, 1, 1), 2);
    }

    #[test]
    fn should_count_test_input_slope_5_1() {
        assert_eq!(count_tree_hits(TEST_INPUT, 5, 1), 3);
    }

    #[test]
    fn should_count_test_input_slope_7_1() {
        assert_eq!(count_tree_hits(TEST_INPUT, 7, 1), 4);
    }

    #[test]
    fn should_count_test_input_slope_1_2() {
        assert_eq!(count_tree_hits(TEST_INPUT, 1, 2), 2);
    }

    #[test]
    fn should_work_with_empty_lines() {
        assert_eq!(count_tree_hits(TEST_INPUT, 3, 1), 7);
    }
}