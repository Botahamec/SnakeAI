#![feature(test)]

extern crate test;

#[derive(PartialEq, Default, Clone, Copy)]
struct Vector2 {
	x: f32,
	y: f32
}

impl Vector2 {
	fn new(x: f32, y: f32) -> Self {
		Vector2 {x: x, y: y}
	}
}

impl std::ops::Add for Vector2 {

	fn add(self, right: Vector2) -> Self {
		Vector2 {
			x: self.x + right.x,
			y: self.y + right.y
		}
	}

	type Output = Self;
}

type Vector2Array = Vec<Vector2>;

fn ham_cycle(snake: &mut Vector2Array, board_x: i32, board_y: i32) -> Option<Vector2Array> {

	fn util(path: &mut Vector2Array, pos: i32, snake: &Vector2Array, board_x: i32, board_y: i32) -> bool {

		fn safe(v: Vector2, snake: &Vector2Array, path: &Vector2Array, pos: i32, board_x: i32, board_y: i32) -> bool {
			if 0.0 > v.x || v.x >= board_x as f32 {return false;}
			if 0.0 > v.y || v.y >= board_y as f32 {return false;}
			for i in 0..snake.len() {
				if v == *snake.get(i).unwrap() {
					return false;
				}
			}
			for i in 0..path.len() {
				if i as i32 > pos {break;}
				if v == *path.get(i).unwrap() {return false;}
			}
			true
		}

		let vertices : i32 = board_x * board_y - snake.len() as i32 + 1;

		if pos == vertices {
			let tail = *snake.get(snake.len() - 1).unwrap();
			let end = *path.get((pos - 1) as usize).unwrap();
			let mut test = end + Vector2::new(1.0, 0.0);
			if test == tail {return true;}
			test = end + Vector2::new(-1.0, 0.0);
			if test == tail {return true;}
			test = end + Vector2::new(0.0, 1.0);
			if test == tail {return true;}
			test = end + Vector2::new(0.0, -1.0);
			if test == tail {return true;}
			false
		} else {
			let past = *path.get((pos - 1) as usize).unwrap();
			let mut test = past + Vector2::new(1.0, 0.0);
			if safe(test, snake, &path, pos, board_x, board_y) {
				path[pos as usize] = test;
				if util(path, pos + 1, snake, board_x, board_y) {return true;}
			}
			test = past + Vector2::new(-1.0, 0.0);
			if safe(test, snake, &path, pos, board_x, board_y) {
				path[pos as usize] = test;
				if util(path, pos + 1, snake, board_x, board_y) {return true;}
			}
			test = past + Vector2::new(0.0, 1.0);
			if safe(test, snake, &path, pos, board_x, board_y) {
				path[pos as usize] = test;
				if util(path, pos + 1, snake, board_x, board_y) {return true;}
			}
			test = past + Vector2::new(0.0, -1.0);
			if safe(test, snake, &path, pos, board_x, board_y) {
				path[pos as usize]  = test;
				if util(path, pos + 1, snake, board_x, board_y) {return true;}
			}
			false
		}
	}

	let mut path : Vector2Array = Vector2Array::new();
	path.resize((board_x * board_y - snake.len() as i32 + 1) as usize, Vector2::default());
	path[0] = snake[0];
	if util(&mut path, 1, &snake, board_x, board_y) {
		let mut new = Vector2Array::new();
		new.append(snake);
		new.remove(0);
		path.append(&mut new);
		Some(path)
	} else { None }
}

#[cfg(test)]
mod tests {

    use super::*;
	use test::{Bencher};

//	#[bench]
//	fn ham_cycle_8_4(b: &mut Bencher) {
//		let v1 : Vector2 = Vector2::new(3.0, 2.0);
//		let v2 : Vector2 = Vector2::new(4.0, 2.0);
//		let mut snake = Vector2Array::new();
//		snake.push(v1);
//		snake.push(v2);
//		b.iter(|| {
//			snake = Vector2Array::new();
//			snake.push(v1);
//			snake.push(v2);
//			ham_cycle(&mut snake, 8, 4);
//		})
//	}
//
	//#[bench]
	//fn ham_cycle_8_6(b: &mut Bencher) {
	//	let v1 : Vector2 = Vector2::new(3.0, 3.0);
	//	let v2 : Vector2 = Vector2::new(4.0, 3.0);
	//	let mut snake = Vector2Array::new();
	//	snake.push(v1);
	//	snake.push(v2);
	//	b.iter(|| {
	//		snake = Vector2Array::new();
	//		snake.push(v1);
	//		snake.push(v2);
	//		ham_cycle(&mut snake, 8, 6);
	//	})
	//}
//
	//#[bench]
	//fn ham_cycle_8_8(b: &mut Bencher) {
	//	let v1 : Vector2 = Vector2::new(3.0, 4.0);
	//	let v2 : Vector2 = Vector2::new(4.0, 4.0);
	//	let mut snake = Vector2Array::new();
	//	b.iter(|| {
	//		snake = Vector2Array::new();
	//		snake.push(v1);
	//		snake.push(v2);
	//		ham_cycle(&mut snake, 8, 8);
	//	})
	//}
//
	//#[bench]fn ham_cycle_16_4(b: &mut Bencher) {
	//	let v1 : Vector2 = Vector2::new(7.0, 2.0);
	//	let v2 : Vector2 = Vector2::new(8.0, 2.0);
	//	let mut snake = Vector2Array::new();
	//	b.iter(|| {
	//		snake = Vector2Array::new();
	//		snake.push(v1);
	//		snake.push(v2);
	//		ham_cycle(&mut snake, 16, 4);
	//	})
	//}
//
	//#[bench]
	//fn ham_cycle_12_6(b: &mut Bencher) {
	//	let v1 : Vector2 = Vector2::new(5.0, 3.0);
	//	let v2 : Vector2 = Vector2::new(6.0, 3.0);
	//	let mut snake = Vector2Array::new();
	//	b.iter(|| {
	//		snake = Vector2Array::new();
	//		snake.push(v1);
	//		snake.push(v2);
	//		ham_cycle(&mut snake, 12, 6);
	//	})
	//}
//
	//#[bench]
	//fn ham_cycle_10_8(b: &mut Bencher) {
	//	let v1 : Vector2 = Vector2::new(4.0, 4.0);
	//	let v2 : Vector2 = Vector2::new(5.0, 4.0);
	//	let mut snake = Vector2Array::new();
	//	b.iter(|| {
	//		snake = Vector2Array::new();
	//		snake.push(v1);
	//		snake.push(v2);
	//		ham_cycle(&mut snake, 10, 8);
	//	})
	//}
//
	//#[bench]
	//fn ham_cycle_12_10(b: &mut Bencher) {
	//	let v1 : Vector2 = Vector2::new(5.0, 5.0);
	//	let v2 : Vector2 = Vector2::new(6.0, 5.0);
	//	let mut snake = Vector2Array::new();
	//	b.iter(|| {
	//		snake = Vector2Array::new();
	//		snake.push(v1);
	//		snake.push(v2);
	//		ham_cycle(&mut snake, 12, 10);
	//	})
	//}
//
	//#[bench]
	//fn ham_cycle_14_14(b: &mut Bencher) {
	//	let v1 : Vector2 = Vector2::new(6.0, 7.0);
	//	let v2 : Vector2 = Vector2::new(7.0, 7.0);
	//	let mut snake = Vector2Array::new();
	//	b.iter(|| {
	//		snake = Vector2Array::new();
	//		snake.push(v1);
	//		snake.push(v2);
	//		ham_cycle(&mut snake, 14, 14);
	//	})
	//}
//
	//#[bench]
	//fn ham_cycle_16_16(b: &mut Bencher) {
	//	let v1 : Vector2 = Vector2::new(7.0, 8.0);
	//	let v2 : Vector2 = Vector2::new(8.0, 8.0);
	//	let mut snake = Vector2Array::new();
	//	b.iter(|| {
	//		snake = Vector2Array::new();
	//		snake.push(v1);
	//		snake.push(v2);
	//		ham_cycle(&mut snake, 16, 16);
	//	})
	//}
//
	//#[bench]
	//fn ham_cycle_18_18(b: &mut Bencher) {
	//	let v1 : Vector2 = Vector2::new(8.0, 9.0);
	//	let v2 : Vector2 = Vector2::new(9.0, 9.0);
	//	let mut snake = Vector2Array::new();
	//	b.iter(|| {
	//		snake = Vector2Array::new();
	//		snake.push(v1);
	//		snake.push(v2);
	//		ham_cycle(&mut snake, 18, 18);
	//	})
	//}

	#[bench]
	fn ham_cycle_20_20(b: &mut Bencher) {
		let v1 : Vector2 = Vector2::new(9.0, 10.0);
		let v2 : Vector2 = Vector2::new(10.0, 10.0);
		let mut snake = Vector2Array::new();
		b.iter(|| {
			snake = Vector2Array::new();
			snake.push(v1);
			snake.push(v2);
			ham_cycle(&mut snake, 20, 20);
		})
	}
}