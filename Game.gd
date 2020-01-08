extends Node2D

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
const board_dimensions : Vector2 = Vector2(8, 4)
const block_size : int = 50
const colors: PoolColorArray = PoolColorArray([Color(.25, .25, .25)])

func check():
	var snake_body = get_child(0).body
	var snake_pos = snake_body[0]
	var apple_pos = get_child(1).pos
	if snake_pos == apple_pos:
		get_child(1).rand_position()
		get_child(3).increment()
		return false
	else: return true

#draws the board
func _draw():
	var points = PoolVector2Array([])
	points.append(Vector2(0, 0))
	points.append(Vector2(board_dimensions.x * block_size, 0))
	points.append(Vector2(board_dimensions.x * block_size, board_dimensions.y * block_size))
	points.append(Vector2(0, board_dimensions.y * block_size))
	draw_polygon(points, colors)