extends RigidBody2D

enum State {GOING_TO_AREA, BREAKING_GARBAGE, IDLE}

export var factor = 100
var state = State.IDLE
var area_position = Vector2(320, 240)

func _physics_process(delta):
	if state == State.GOING_TO_AREA:
		if global_position.distance_to(area_position) < 10.0:
			state = State.IDLE
		else:
			apply_central_impulse((area_position - global_position).normalized() * factor * delta)
