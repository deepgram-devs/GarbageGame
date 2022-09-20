extends KinematicBody2D

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

var velocity = Vector2.ZERO
var speed = 100

var rng = RandomNumberGenerator.new()

func _ready():
	rng.randomize()
	speed *= rng.randf_range(0.8, 1.2)

func _physics_process(_delta):
	if is_instance_valid(waste) and state != State.COLLECTING_GARBAGE:
		if waste.being_collected:
			print("Why is this happening?")
	
	if state == State.GOING_TO_AREA:
		if global_position.distance_to(area_position) < 10.0:
			state = State.IDLE
			
			velocity = Vector2(0, 0)
		else:
			velocity = (area_position - global_position).normalized() * speed
	elif state == State.COLLECTING_GARBAGE:
		# logically, waste cannot be null if the state is COLLECTING_GARBAGE
		if waste != null:
			velocity = (waste.global_position - global_position).normalized() * speed
		if waste == null:
			print("Why is this happening?")
			state = State.IDLE
			
			velocity = Vector2.ZERO
	elif state == State.GOING_TO_MUSHROOM:
		if waste != null and waste.global_position.distance_to(mushroom_position) < 50.0:
			# if the ant is GOING_TO_MUSHROOM, it must be carrying waste
			# if the ant gets close to the MUSHROOM, make it idle,
			# destroy the pin joint with the waste, and destroy the waste
			state = State.IDLE
			if pin_joint_2d != null:
				pin_joint_2d.queue_free()
			if waste != null:
				waste.queue_free()
			
			velocity = Vector2.ZERO
		else:
			velocity = (mushroom_position - global_position).normalized() * speed

	var returned_velocity = move_and_slide(velocity, Vector2.ZERO, false, 4, 0, false)
	
	if returned_velocity != Vector2.ZERO:
		rotation = returned_velocity.angle()
	
	for index in get_slide_count():
		var collision = get_slide_collision(index)
		if collision.collider.is_in_group("Waste"):
			if collision.collider == waste:
				carrying_waste = true
				waste.being_carried = true
				waste.being_collected = false
			
				# this is kind of a hack for the AI, so that ants don't get stuck colliding into waste being carried by other ants
				waste.set_collision_layer_bit(3, false)
						
				state = State.GOING_TO_MUSHROOM
			
				pin_joint_2d = PinJoint2D.new()
				pin_joint_2d.add_to_group("PinJoint2D")

				pin_joint_2d.set_node_a(self.get_path())
				pin_joint_2d.set_node_b(waste.get_path())

				add_child(pin_joint_2d)
