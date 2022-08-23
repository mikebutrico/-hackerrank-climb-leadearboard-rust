use std::vec;

fn climbing_leaderboard(ranked: &Vec<usize>, player: &Vec<usize>) -> Vec<usize> {
    let mut res: Vec<usize> = vec![];
    let mut first_pass = true;
    let mut previous_rank = &ranked[0];
    let mut rank_set: Vec<&usize> = vec![];

    fn find_rank(array: &Vec<&usize>, search_value: usize) -> usize {
        let mut lower_bound = 0;
        let mut upper_bound = array.len() - 1;
        let mut mid_point_index = 0;
        let mut mid_point_value: &usize;

        while lower_bound <= upper_bound {
            mid_point_index = (upper_bound + lower_bound) / 2;
            mid_point_value = array[mid_point_index];

            if search_value == *mid_point_value {
                return mid_point_index + 1;
            } else if search_value > *mid_point_value && mid_point_index > 0 {
                upper_bound = mid_point_index - 1;
            } else if search_value < *mid_point_value {
                lower_bound = mid_point_index + 1
            } else if mid_point_index == 0 {
                break;
            }
        }
        if search_value < *array[mid_point_index] {
            return mid_point_index + 2;
        }

        return mid_point_index + 1;
    }

    for rank in ranked {
        if first_pass == true {
            first_pass = false;
            rank_set.push(rank);
        } else {
            if *rank != *previous_rank {
                rank_set.push(rank);
            }

            previous_rank = rank;
        }
    }

    for curr_player in player {
        res.push(find_rank(&rank_set, *curr_player));
    }

    return res;
}
fn main() {
    let ranks = vec![100, 100, 50, 40, 40, 20, 10];
    let player = vec![5, 25, 50, 120];

    println!("{:?}", climbing_leaderboard(&ranks, &player));
}
