#![allow(non_snake_case)]
#![allow(clippy::not_unsafe_ptr_arg_deref)]
#![allow(clippy::transmute_ptr_to_ptr)]

extern crate gdnative;

use gdnative::{godot_error, Variant, VariantArray, FromVariant, GodotString, godot_gdnative_init, godot_gdnative_terminate, godot_nativescript_init, godot_print, godot_wrap_method, godot_wrap_method_inner, godot_wrap_method_parameter_count, methods, user_data, NativeClass, Node, Vector2, Vector2Array};

const BOARD_X: i32 = 8;
const BOARD_Y: i32 = 4;

fn ham_cycle(snake: Vector2Array) -> Option<Vector2Array> {

	fn util(path: &mut Vector2Array, pos: i32, snake: &Vector2Array) -> bool {

		fn safe(v: Vector2, snake: &Vector2Array, path: &Vector2Array, pos: i32) -> bool {
			if 0.0 > v.x || v.x >= BOARD_X as f32 {return false;}
			if 0.0 > v.y || v.y >= BOARD_Y as f32 {return false;}
			for i in 0..snake.len() {
				if v == snake.get(i) {
					return false;
				}
			}
			for i in 0..path.len() {
				if i > pos {break;}
				if v == path.get(i) {return false;}
			}
			true
		}

		let vertices : i32 = BOARD_X * BOARD_Y - 1;

		if pos == vertices {
			let tail = snake.get(snake.len() - 1);
			let end = path.get(pos - 1);
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
			let past = path.get(pos - 1);
			let mut test = past + Vector2::new(1.0, 0.0);
			if safe(test, snake, &path, pos) {
				path.set(pos, &test);
				if util(path, pos + 1, snake) {return true;}
			}
			test = past + Vector2::new(-1.0, 0.0);
			if safe(test, snake, &path, pos) {
				path.set(pos, &test);
				if util(path, pos + 1, snake) {return true;}
			}
			test = past + Vector2::new(0.0, 1.0);
			if safe(test, snake, &path, pos) {
				path.set(pos, &test);
				if util(path, pos + 1, snake) {return true;}
			}
			test = past + Vector2::new(0.0, -1.0);
			if safe(test, snake, &path, pos) {
				path.set(pos, &test);
				if util(path, pos + 1, snake) {return true;}
			}
			false
		}
	}

	let mut path : Vector2Array = Vector2Array::new();
	path.resize(BOARD_X * BOARD_Y - 1);
	path.set(0, &snake.get(0));
	if util(&mut path, 1, &snake) {
		let mut new = Vector2Array::new();
		new.push_array(&snake);
		new.remove(0);
		path.push_array(&new);
		Some(path)
	} else { None }
}

fn find_index(snake: &Vector2Array, path: &Vector2Array) -> i32 {
	for i in 0..path.len() {
		for j in 0..snake.len() {
			if snake.get(j) == path.get(i) {return i;}
		}
	}
	return -1;
}

/// The HelloWorld "class"
#[derive(NativeClass)]
#[inherit(Node)]
#[user_data(user_data::MutexData<AI>)]
pub struct AI {
	path: Vector2Array,
	index: i32
}

// __One__ `impl` block can have the `#[methods]` attribute, which will generate
// code to automatically bind any exported methods to Godot.
#[methods]
impl AI {

	/// The "constructor" of the class.
	fn _init(_owner: Node) -> Self {
		AI{path: Vector2Array::new(), index: 0}
	}

	// In order to make a method known to Godot, the #[export] attribute has to be used.
	// In Godot script-classes do not actually inherit the parent class.
	// Instead they are"attached" to the parent object, called the "owner".
	// The owner is passed to every single exposed method.
	#[export]
	unsafe fn _ready(&mut self, owner: Node) {
		// The `godot_print!` macro works like `println!` but prints to the Godot-editor
		// output tab as well.
		let mut snake = owner.get_parent().unwrap().get_child(0).unwrap();
		let body = Vector2Array::from_variant(&snake.call(GodotString::from("get_body"), &[])).unwrap();
		let ham = ham_cycle(body);
		match ham {
			Some(p) => self.path = p,
			None => godot_print!("body: {}", Vector2Array::from_variant(&owner.get_parent().unwrap().get_child(0).unwrap().call(GodotString::from("get_body"), &[])).unwrap().get(0))
		};
		match snake.connect(GodotString::from("moved"), Some(owner.to_object()), GodotString::from("move_snake"), VariantArray::new(), 0) {
			Ok(_n) => godot_print!("AI connected"),
			Err (_e) => godot_print!("{}", _e as u8)
		};
	}

	#[export]
	unsafe fn move_snake(&mut self, owner: Node) {
		// changes the snake's direction whenever the snake moves
		let mut snake = owner.get_parent().unwrap().get_child(0).unwrap();
		//let body = Vector2Array::from_variant(&snake.call(GodotString::from("get_body"), &[])).unwrap();
		//let index = find_index(&body, &self.path);
		//let current = self.path.get(index);
		//let next = self.path.get(index + 1);
		self.index += 1;
		self.index %= self.path.len();
		let current = self.path.get(self.index);
		let mut next_index = self.index + 1;
		next_index %= self.path.len();
		let next = self.path.get(next_index);

		let mut test = current + Vector2::new(1.0, 0.0);
		if test == next {
			snake.call(GodotString::from("change_direction"), &[Variant::from_i64(1)]);
		}
		test = current + Vector2::new(-1.0, 0.0);
		if test == next {
			snake.call(GodotString::from("change_direction"), &[Variant::from_i64(0)]);
		}
		test = current + Vector2::new(0.0, 1.0);
		if test == next {
			snake.call(GodotString::from("change_direction"), &[Variant::from_i64(3)]);
		}
		test = current + Vector2::new(0.0, -1.0);
		if test == next {
			snake.call(GodotString::from("change_direction"), &[Variant::from_i64(2)]);
		}
	}
}

// Function that registers all exposed classes to Godot
fn init(handle: gdnative::init::InitHandle) {
	handle.add_class::<AI>();
}

// macros that create the entry-points of the dynamic library.
godot_gdnative_init!();
godot_nativescript_init!(init);
godot_gdnative_terminate!();