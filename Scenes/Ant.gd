extends RigidBody2D

enum State {GOING_TO_AREA, COLLECTING_GARBAGE, GOING_TO_MUSHROOM, IDLE}

export var factor = 100
var state = State.IDLE
var area_position = Vector2(320, 240)
var mushroom_position = Vector2(320, 240)
var carrying_waste = false

func _physics_process(delta):
	if state == State.GOING_TO_AREA:
		if global_position.distance_to(area_position) < 10.0:
			state = State.IDLE
		else:
			apply_central_impulse((area_position - global_position).normalized() * factor * delta)

func _on_Ant_body_entered(body):
	if body.is_in_group("Waste"):
		if !carrying_waste and !body.being_carried:
			carrying_waste = true
			body.being_carried = true
			
			var pin_joint_2d = PinJoint2D.new()

			pin_joint_2d.set_node_a(self.get_path())
			pin_joint_2d.set_node_b(body.get_path())

			add_child(pin_joint_2d)
