extends CanvasItem

enum Direction {Left, Right, Up, Down}

signal moved

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var body : PoolVector2Array = PoolVector2Array([Vector2(4, 2), Vector2(3, 2)])
var colors: PoolColorArray = PoolColorArray([Color(0, 1, 0)])
var time: float = 0
var direction
var dead: bool = false

const frame_length: float = 0.2 # controls speed of the game
const board_dimensions : Vector2 = Vector2(8, 4)
const block_size : int = 50

func _ready():
	direction = Direction.Right

# draws the snake
func _draw():
	for cell in body:
		var points = PoolVector2Array([])
		var x = cell.x * block_size
		var y = cell.y * block_size
		points.append(Vector2(x, y))
		points.append(Vector2(x + 40, y))
		points.append(Vector2(x + 40, y + 40))
		points.append(Vector2(x, y + 40))
		draw_polygon(points, colors)

# moves the snake in the current direction
func move():
	var new_pos = body[0]
	match direction:
		Direction.Right:
			new_pos.x += 1
		Direction.Left:
			new_pos.x -= 1
		Direction.Up:
			new_pos.y -= 1
		Direction.Down:
			new_pos.y += 1
	# warning-ignore:return_value_discarded
	body.insert(0, new_pos)
	if get_parent().check(): body.remove(body.size() - 1)

# kills the snake if necessary
func check_death():
	if body[0].x >= board_dimensions.x or body[0].x < 0: dead = true
	if body[0].y >= board_dimensions.y or body[0].y < 0: dead = true
	for i in range(1, body.size()):
		if body[i] == body[0]: dead = true

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	
	# checks to see if the frame should be advanced
	if dead: return
	time += delta
	if time < frame_length: return
	time = 0
	
	move()
	check_death()
	emit_signal("moved")
	
	update() # draws to the screen

func _input(input):
	if input is InputEventKey:
		if input.scancode == KEY_LEFT: direction = Direction.Left
		if input.scancode == KEY_RIGHT: direction = Direction.Right
		if input.scancode == KEY_UP: direction = Direction.Up
		if input.scancode == KEY_DOWN: direction = Direction.Down
	pass

func get_body() -> PoolVector2Array:
	return body

func change_direction(dir: int):
	if dir == 0:
		direction = Direction.Left
	if dir == 1:
		direction = Direction.Right
	if dir == 2:
		direction = Direction.Up
	if dir == 3:
		direction = Direction.Down


