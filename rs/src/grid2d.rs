// Leverage https://github.com/mlhoyt/aoc2020/blob/main/rs/src/bin/day11part2.rs "Layout"
// 2D grid abstraction.

#[derive(Debug, Clone)]
pub struct Grid2D<T: Copy> {
    grid: Vec<T>,
    width: usize,
    height: usize,
}

impl<T: Copy> Grid2D<T> {
    fn init() -> Self {
        Self {
            grid: vec![],
            width: 0,
            height: 0,
        }
    }

    pub fn new(rows: &[Vec<T>]) -> Result<Self, String> {
        let mut rv = Self::init();

        for (i, r) in rows.iter().enumerate() {
            if i == 0 {
                rv.width = r.len();
            } else if rv.width != r.len() {
                return Err(format!(
                    "row {} has length {} which does not match the previous length {}",
                    i,
                    r.len(),
                    rv.width
                ));
            }

            r.iter().for_each(|v| rv.grid.push(*v));

            rv.height += 1;
        }

        Ok(rv)
    }

    pub fn get_width(&self) -> usize {
        self.width
    }

    pub fn get_height(&self) -> usize {
        self.height
    }

    pub fn get_yx(&self, y: usize, x: usize) -> Option<T> {
        self.yx_to_index(y, x).map(|i| self.grid[i])
    }

    fn set_yx(&mut self, y: usize, x: usize, v: T) -> Option<usize> {
        match self.yx_to_index(y, x) {
            Some(i) => {
                self.grid[i] = v;
                Some(i)
            }
            _ => None,
        }
    }

    fn index_to_yx(&self, i: usize) -> (usize, usize) {
        let row = i / self.width;
        let col = i % self.width;

        (row, col)
    }

    fn yx_to_index(&self, y: usize, x: usize) -> Option<usize> {
        if y < (self.height) && x < (self.width) {
            let n = (y * self.width) + (x);
            Some(n)
        } else {
            None
        }
    }

    pub fn iter(&self) -> Grid2DIter<T> {
        Grid2DIter::<T> {
            grid: self,
            index: 0,
        }
    }
}

pub struct Grid2DIter<'a, T: Copy> {
    grid: &'a Grid2D<T>,
    index: usize,
}

impl<'a, T: Copy> Iterator for Grid2DIter<'a, T> {
    type Item = Grid2DPoint<T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.grid.grid.len() {
            None
        } else {
            let pos = self.grid.index_to_yx(self.index);
            self.index += 1;

            Some(Self::Item {
                x: pos.1,
                y: pos.0,
                value: self.grid.get_yx(pos.0, pos.1).unwrap(),
            })
        }
    }
}

impl<T: Copy + Default> std::iter::FromIterator<Grid2DPoint<T>> for Grid2D<T> {
    fn from_iter<I: std::iter::IntoIterator<Item = Grid2DPoint<T>>>(iter: I) -> Self {
        let mut max_x = 0;
        let mut max_y = 0;
        let mut ps = vec![];

        // determine width and height and capture points
        for i in iter {
            max_x = max_x.max(i.x);
            max_y = max_y.max(i.y);
            ps.push(i);
        }

        // initialize Grid2D with discovered width and height
        let mut rv = Grid2D::<T>::init();
        rv.width = max_x + 1;
        rv.height = max_y + 1;
        rv.grid = vec![T::default(); rv.width * rv.height];

        // populate Grid2D values with captured points
        ps.iter().for_each(|p| {
            rv.set_yx(p.y, p.x, p.value);
        });

        rv
    }
}

#[derive(Debug, Default, Hash, PartialEq, Eq, Clone)]
pub struct Grid2DPoint<T> {
    pub x: usize,
    pub y: usize,
    pub value: T,
}
