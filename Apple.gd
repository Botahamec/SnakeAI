extends Sprite

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var pos: Vector2 = Vector2(0, 0)

const board_dimensions : Vector2 = Vector2(8, 4)
const block_size : int = 50

# Called when the node enters the scene tree for the first time.
func _ready():
	rand_position()

func check_position(new_pos: Vector2):
	for cell in get_parent().get_child(0).body:
		if cell == new_pos: return true
	return false

# puts the apple in a random spot on the board
func rand_position():
	var snake_body = get_parent().get_child(0).body
	if snake_body.size() == board_dimensions.x * board_dimensions.y:
		move_local_x(4000)
		move_local_y(4000)
		return
	randomize()
	var new_x : int
	var new_y : int
	while true:
		new_x = randi() % int(board_dimensions.x)
		new_y = randi() % int(board_dimensions.y)
		var new_pos = Vector2(new_x, new_y)
		if check_position(new_pos): continue
		break
	var delta_x = new_x - pos.x
	var delta_y = new_y - pos.y
	move_local_x(delta_x * block_size)
	move_local_y(delta_y * block_size)
	pos.x = new_x
	pos.y = new_y