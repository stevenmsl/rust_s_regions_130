#[derive(Debug, PartialEq)]

pub struct Solution {}

impl Solution {
    /* key concepts
       - mark candidates
       - flood fill from the edge elements
    */

    pub fn solve(board: &mut Vec<Vec<char>>) {
        let row = board.len() - 1;
        let col = board[0].len() - 1;
        println!("{:?}", board);
        /*
          - mark all the candidates from 'O' to '-'
          - start from the edge elements of the candidates
            and "flood-fill" (by changing '-' back to 'O')
            as much as you can
            - if the element is on the edge, it won't be
              surrounded by 'X', so are its adjacent
              elements, and the adjacent elements of
              its adjacent elements so on and so forth
            - pay attention to how you determine if an
              element is on the edge
          - change the remaining '-' to 'X'
        */

        /* mark candidates */
        Self::replace(board, 'O', '-');

        println!("{:?}", board);

        /* flood-fill from the edge elements */
        for row_index in 0..=row {
            for col_index in 0..=col {
                let edge_element =
                    row_index == 0 || row_index == row || col_index == 0 || col_index == col;
                if !edge_element {
                    continue;
                }
                if board[row_index][col_index] == '-' {
                    Self::flood_fill(board, row_index, col_index);
                }
            }
        }

        println!("{:?}", board);

        /* change the remaining '-' to 'X' */
        Self::replace(board, '-', 'X');

        println!("{:?}", board);
    }

    pub fn replace(board: &mut Vec<Vec<char>>, found: char, replace_with: char) {
        let row = board.len();
        let col = board[0].len();
        for row_index in 0..row {
            for col_index in 0..col {
                if board[row_index][col_index] == found {
                    board[row_index][col_index] = replace_with;
                }
            }
        }
    }

    pub fn flood_fill(board: &mut Vec<Vec<char>>, row_index: usize, col_index: usize) {
        let row = board.len() - 1;
        let col = board[0].len() - 1;
        if board[row_index][col_index] == 'X' {
            return;
        }
        board[row_index][col_index] = 'O';

        // visit left
        if col_index > 0 {
            Self::flood_fill(board, row_index, col_index - 1);
        }
        // visit up
        if row_index > 0 {
            Self::flood_fill(board, row_index - 1, col_index);
        }
        // visit right
        if col_index < col {
            Self::flood_fill(board, row_index, col_index + 1);
        }
        // visit down
        if row_index < row {
            Self::flood_fill(board, row_index + 1, col_index);
        }
    }

    pub fn text_fixture_1() -> Vec<Vec<char>> {
        vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'O', 'X'],
            vec!['X', 'X', 'O', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ]
    }

    pub fn text_fixture_2() -> Vec<Vec<char>> {
        vec![vec!['X']]
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sample_1() {
        let target = vec![
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'X', 'X', 'X'],
            vec!['X', 'O', 'X', 'X'],
        ];

        let mut board = Solution::text_fixture_1();
        Solution::solve(&mut board);
        assert_eq!(board, target);
    }

    #[test]
    fn sample_2() {
        let target = vec![vec!['X']];
        let mut board = Solution::text_fixture_2();
        Solution::solve(&mut board);
        assert_eq!(board, target);
    }
}
