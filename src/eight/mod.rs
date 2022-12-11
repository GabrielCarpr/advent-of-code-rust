use std::fs::read_to_string;

use math::round;

#[derive(Debug)]
struct Tree{
    index: usize,
    height: u32,
    visible: bool,
}

impl Tree {
    fn from(index: usize, height: u32) -> Self {
        Tree{
            index,
            height,
            visible: false,
        }
    }

    fn taller_than(&self, tree: &Tree) -> bool {
        return self.height > tree.height;
    }

    fn smaller_than(&self, tree: &Tree) -> bool {
        return self.height < tree.height;
    }

    fn view(&mut self) {
        self.visible = true;
    }
}

#[derive(Debug)]
struct Forest {
    trees: Vec<Tree>,
    width: usize,
    height: usize,
}

impl Forest {
    pub fn from(input: &str) -> Self {
        let mut lines = input.lines().peekable();
        let width = lines.peek().clone().unwrap().chars().count(); // Account for indexes
        let height = lines.count();

        let mut index = 0;
        let trees: Vec<Tree> = input
            .lines()
            .flat_map(|line| line.chars())
            .map(|tree| tree.to_digit(10).expect("Could not parse tree"))
            .map(|tree| {
                index += 1;
                Tree::from(index, tree)
            })
            .collect();

        Forest {
            trees,
            width,
            height,
        }
    }

    fn look_for_trees(&mut self) {
        self.walk_perimeter();
    }

    fn walk_perimeter(&mut self) {
        for i in 0..self.trees.len() {
            let row = (i / self.height) + 1;
            let col = (i % self.width) + 1;

            // Edge
            if row as usize == 1 || row == self.height || col == 1 || col == self.width {
                self.trees[i].view();
            }
        }
    }

    fn check_each_line(&mut self) {
        for row in 0..self.height {
            let trees = self.trees[row*self.width..((row+1)*self.width)-1];

            let mut highest = 0;
            for tree in trees {
                if tree.height < highest {
                    self.trees[tree.index].view();
                    highest = tree.height;
                }
            }
        }
    }

    fn visible_trees(&self) -> usize {
        return self.trees.iter().filter(|tree| tree.visible).count();
    }
}

pub fn eight() {
    println!("Eight");

    let mut forest = Forest::from(read_to_string("src/eight/input.txt").unwrap().as_str());
    forest.look_for_trees();

    println!("Forest trees: {:?}", forest.visible_trees());
}