extends RigidBody2D

var being_carried = false

func _physics_process(_delta):
	if position.y >= 240:
		gravity_scale = 0
