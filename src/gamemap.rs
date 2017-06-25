use std::cmp::*;
use std::ops::{Add, Sub, Index, IndexMut};
#[derive(Copy, Clone, Eq, Debug)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}
impl Coord {
    pub fn new(x_: i32, y_: i32) -> Coord {
        Coord { x: x_, y: y_ }
    }
}
impl Add for Coord {
    type Output = Coord; // Coord * Coord -> Coord
    fn add(self, other: Coord) -> Coord {
        Coord::new(self.x + other.x, self.y + other.y)
    }
}
impl Sub for Coord {
    type Output = Coord;
    fn sub(self, other: Coord) -> Coord {
        Coord::new(self.x - other.x, self.y - other.y)
    }
}
impl Ord for Coord {
    fn cmp(&self, other: &Coord) -> Ordering {
        let xcmp = self.x.cmp(&other.x);
        match xcmp {
            Ordering::Equal => self.y.cmp(&other.y),
            _ => xcmp,
        }
    }
}
impl PartialEq for Coord {
    fn eq(&self, other: &Coord) -> bool {
        self.x == other.x && self.y == other.y
    }
}
impl PartialOrd for Coord {
    fn partial_cmp(&self, other: &Coord) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// ゲームマップ用の汎用型
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct Field<T> {
    data: Vec<Vec<T>>,
    height: usize,
    width: usize,
}

impl<T: Copy> Field<T> {
    pub fn new(init_num: T, h: usize, w: usize) -> Field<T> {
        let res = vec![vec![init_num; w]; h];
        Field {
            data: res,
            height: h,
            width: w,
        }
    }
}

impl<T> Field<T> {
    pub fn id_ok(&self, id: &Coord) -> bool {
        id.x >= 0 && id.y >= 0 && id.x < self.width as i32 && id.y < self.height as i32
    }
}

impl<T> Index<Coord> for Field<T> {
    type Output = T;
    fn index(&self, id: Coord) -> &T {
        assert!(self.id_ok(&id), "Index Error {:?}", id);
        &self.data[id.y as usize][id.x as usize]
    }
}

impl<T> IndexMut<Coord> for Field<T> {
    fn index_mut(&mut self, id: Coord) -> &mut T {
        assert!(self.id_ok(&id), "Index Error {:?}", id);
        &mut self.data[id.y as usize][id.x as usize]
    }
}

impl<T> Index<(usize, usize)> for Field<T> {
    type Output = T;
    fn index(&self, id: (usize, usize)) -> &T {
        assert!(id.0 < self.width && id.1 < self.height,
                "Index Error {:?}",
                id);
        &self.data[id.1 as usize][id.0 as usize]
    }
}

impl<T> IndexMut<(usize, usize)> for Field<T> {
    fn index_mut(&mut self, id: (usize, usize)) -> &mut T {
        assert!(id.0 < self.width && id.1 < self.height,
                "Index Error {:?}",
                id);
        &mut self.data[id.1 as usize][id.0 as usize]
    }
}
