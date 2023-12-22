use itertools::Itertools;
use std::collections::HashMap;

fn main() {
    println!("Part 1: {}", part1(include_str!("../input.txt")));
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct Brick {
    start_pos: [usize; 3],
    end_pos: [usize; 3],
}

impl Brick {
    fn parse(input: &str) -> Self {
        let (start, end) = input.split_once('~').unwrap();
        let start_pos = start
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        let end_pos = end
            .split(',')
            .map(|num| num.parse::<usize>().unwrap())
            .collect::<Vec<_>>()
            .try_into()
            .unwrap();

        Self { start_pos, end_pos }
    }

    fn lower_by(&mut self, amount: usize) {
        self.start_pos[2] -= amount;
        self.end_pos[2] -= amount;
    }

    fn project_down(&self) -> impl Iterator<Item = (usize, usize)> {
        let x_positions = self.start_pos[0]..=self.end_pos[0];
        let y_positions = self.start_pos[1]..=self.end_pos[1];

        x_positions.cartesian_product(y_positions)
    }

    fn height_of_top(&self) -> usize {
        self.start_pos[2].max(self.end_pos[2])
    }

    fn height_from_ground(&self) -> usize {
        self.start_pos[2].min(self.end_pos[2])
    }

    fn contains_point(&self, point: [usize; 3]) -> bool {
        let min_x = self.start_pos[0].min(self.end_pos[0]);
        let max_x = self.start_pos[0].max(self.end_pos[0]);

        let min_y = self.start_pos[1].min(self.end_pos[1]);
        let max_y = self.start_pos[1].max(self.end_pos[1]);

        let min_z = self.start_pos[2].min(self.end_pos[2]);
        let max_z = self.start_pos[2].max(self.end_pos[2]);

        (min_x..=max_x).contains(&point[0])
            && (min_y..=max_y).contains(&point[1])
            && (min_z..=max_z).contains(&point[2])
    }
}

struct Sky {
    bricks: Vec<Brick>,
}

impl Sky {
    fn parse(input: &str) -> Self {
        let bricks = input.lines().map(Brick::parse).collect();

        Self { bricks }
    }

    fn drop_everything(&mut self) {
        self.bricks.sort_by_key(|brick| brick.height_from_ground());

        let mut floor_heights: HashMap<(usize, usize), usize> = HashMap::new();

        for brick in &mut self.bricks {
            let mut drop_height = brick.height_from_ground();

            for pos in brick.project_down() {
                if let Some(&height) = floor_heights.get(&pos) {
                    drop_height = (brick.height_from_ground() - height).min(drop_height);
                }
            }

            brick.lower_by(drop_height);

            for pos in brick.project_down() {
                floor_heights.insert(pos, brick.height_of_top() + 1);
            }
        }
    }

    fn disintegratable_blocks(&mut self) -> usize {
        let mut count = 0;
        self.bricks.sort_by_key(|brick| brick.height_from_ground());

        'outer: for (i, brick_to_remove) in self.bricks.iter().enumerate() {
            for (test_brick_index, test_brick) in self.bricks.iter().enumerate().skip(i) {
                // check that test_brick can still stand somewhere

                // only need to actually check it if it touches the removed one
                if test_brick.height_from_ground() == brick_to_remove.height_of_top() + 1 {
                    // there is a chance this one could fall, check if something else could hold it
                    let z = test_brick.height_from_ground() - 1;

                    if !test_brick.project_down().any(|(x, y)| {
                        self.bricks[..test_brick_index].iter().enumerate().any(
                            |(check_index, brick_to_check)| {
                                check_index != i && brick_to_check.contains_point([x, y, z])
                            },
                        )
                    }) {
                        continue 'outer;
                    }
                }
            }

            count += 1;
        }

        count
    }
}

fn part1(input: &str) -> usize {
    let mut sky = Sky::parse(input);

    sky.drop_everything();

    sky.disintegratable_blocks()
}

#[test]
fn part1_given_input() {
    assert_eq!(
        part1(
            "1,0,1~1,2,1
0,0,2~2,0,2
0,2,3~2,2,3
0,0,4~0,2,4
2,0,5~2,2,5
0,1,6~2,1,6
1,1,8~1,1,9"
        ),
        5
    );
}
