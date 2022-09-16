extends RigidBody2D

# converting this code to Rust will be nice, because what I really want is to attach
# types to these enums: GoingToArea(Area2D), CollectingGarbage(Waste), GoingToMushroom(Mushroom), Idle
enum State {GOING_TO_AREA, COLLECTING_GARBAGE, GOING_TO_MUSHROOM, IDLE}

export var factor = 100
var state = State.IDLE

# instead of these "positions" maybe these should be nullable references to the objects like we do for waste?
var area_position = Vector2(320, 240)
var mushroom_position = Vector2(320, 240)
var waste = null
var pin_joint_2d = null
var carrying_waste = false

func _physics_process(delta):
	if is_instance_valid(waste) and state != State.COLLECTING_GARBAGE:
		if waste.being_collected:
			print("Why is this happening?")
	
	if state == State.GOING_TO_AREA:
		if global_position.distance_to(area_position) < 10.0:
			state = State.IDLE
		else:
			apply_central_impulse((area_position - global_position).normalized() * factor * delta)
	elif state == State.COLLECTING_GARBAGE:
		# logically, waste cannot be null if the state is COLLECTING_GARBAGE
		if waste != null:
			apply_central_impulse((waste.global_position - global_position).normalized() * factor * delta)
		if waste == null:
			print("Why is this happening?")
			state = State.IDLE
	elif state == State.GOING_TO_MUSHROOM:
		if global_position.distance_to(mushroom_position) < 10.0:
			# if the ant is GOING_TO_MUSHROOM, it must be carrying waste
			# if the ant gets close to the MUSHROOM, make it idle,
			# destroy the pin joint with the waste, and destroy the waste
			state = State.IDLE
			if pin_joint_2d != null:
				pin_joint_2d.queue_free()
			if waste != null:
				waste.queue_free()
		else:
			apply_central_impulse((mushroom_position - global_position).normalized() * factor * delta)

func _on_Ant_body_entered(body):
	if body.is_in_group("Waste"):
		if body == waste:
			carrying_waste = true
			body.being_carried = true
			body.being_collected = false
			
			# this is kind of a hack for the AI, so that ants don't get stuck colliding into waste being carried by other ants
			body.set_collision_layer_bit(0, false)
			body.set_collision_layer_bit(1, true)
						
			state = State.GOING_TO_MUSHROOM
			
			pin_joint_2d = PinJoint2D.new()
			pin_joint_2d.add_to_group("PinJoint2D")

			pin_joint_2d.set_node_a(self.get_path())
			pin_joint_2d.set_node_b(body.get_path())

			add_child(pin_joint_2d)
