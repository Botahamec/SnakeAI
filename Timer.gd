extends Label

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var time : float

# Called when the node enters the scene tree for the first time.
func _ready():
	time = 0.0

# Called every frame. 'delta' is the elapsed time since the previous frame.
func _process(delta):
	if not get_parent().get_child(0).dead:
		time += delta
		text = "Time: " + str(time) + " secs"