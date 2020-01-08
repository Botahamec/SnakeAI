extends Label

# Declare member variables here. Examples:
# var a = 2
# var b = "text"
var score : int

# Called when the node enters the scene tree for the first time.
func _ready():
	score = get_parent().get_child(0).body.size()
	text = "Score: " + str(score) + " pts"

func increment():
	score += 1
	text = "Score: " + str(score) + " pts"
