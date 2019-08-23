use ggez::mint::Point2;

#[derive(Clone)]
pub struct CellRect {
	pub pos: Point2<i32>,
	pub w: i32,
	pub h: i32,
}

impl CellRect {
	pub fn overlapping(&self, b: &CellRect) -> bool {
		if self.pos.x+self.w-1 < b.pos.x || self.pos.y+self.h-1 < b.pos.y || self.pos.x > b.pos.x + b.w-1 || self.pos.y > b.pos.y + b.h-1 {
			return false
		} else {
			return true
		}
	}
}

#[test]
fn testoverlapping() {
	let pos1 = CellRect {
		pos: Point2 {
			x: 2,
			y: 4
		},
		w: 2,
		h: 2,
	};
	let pos2 = CellRect {
		pos: Point2 {
			x: 4,
			y: 5
		},
		w: 2,
		h: 1,
	};

	assert!(pos1.overlapping(&pos2) == true);
}