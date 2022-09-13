extends Node2D

enum SelectedFaction {NONE, ANT, BEETLE}

var selected_faction = SelectedFaction.NONE

var rng = RandomNumberGenerator.new()

func _ready():
	rng.randomize()

func _input(event):
	if event is InputEventKey and event.pressed:
		if event.scancode == KEY_A:
			selected_faction = SelectedFaction.ANT
		if event.scancode == KEY_B:
			selected_faction = SelectedFaction.BEETLE
		
		if selected_faction == SelectedFaction.ANT:
			if event.scancode == KEY_1:
				var ants = get_tree().get_nodes_in_group("Ant")
				for ant in ants:
					ant.area_position = $Area1.position
					ant.state = ant.State.GOING_TO_AREA
			if event.scancode == KEY_2:
				var ants = get_tree().get_nodes_in_group("Ant")
				for ant in ants:
					ant.area_position = $Area2.position
					ant.state = ant.State.GOING_TO_AREA
			if event.scancode == KEY_3:
				var ants = get_tree().get_nodes_in_group("Ant")
				for ant in ants:
					ant.area_position = $Area3.position
					ant.state = ant.State.GOING_TO_AREA
			if event.scancode == KEY_4:
				var ants = get_tree().get_nodes_in_group("Ant")
				for ant in ants:
					ant.area_position = $Area4.position
					ant.state = ant.State.GOING_TO_AREA
					
		if selected_faction == SelectedFaction.BEETLE:
			if event.scancode == KEY_1:
				var beetles = get_tree().get_nodes_in_group("Beetle")
				for beetle in beetles:
					beetle.area_position = $Area1.position
					beetle.state = beetle.State.GOING_TO_AREA
			if event.scancode == KEY_2:
				var beetles = get_tree().get_nodes_in_group("Beetle")
				for beetle in beetles:
					beetle.area_position = $Area2.position
					beetle.state = beetle.State.GOING_TO_AREA
			if event.scancode == KEY_3:
				var beetles = get_tree().get_nodes_in_group("Beetle")
				for beetle in beetles:
					beetle.area_position = $Area3.position
					beetle.state = beetle.State.GOING_TO_AREA
			if event.scancode == KEY_4:
				var beetles = get_tree().get_nodes_in_group("Beetle")
				for beetle in beetles:
					beetle.area_position = $Area4.position
					beetle.state = beetle.State.GOING_TO_AREA

func _on_AntSpawnTimer_timeout():
	var ants = get_tree().get_nodes_in_group("Ant")
	
	if ants.size() >= 10:
		return

	var new_ant = load("res://Scenes/Ant.tscn").instance()
	add_child(new_ant)
	new_ant.position = Vector2(320, 360)

	for ant in ants:
		new_ant.position = ant.position
		new_ant.state = ant.state
		new_ant.area_position = ant.area_position
		break

func _on_BeetleSpawnTimer_timeout():
	var beetles = get_tree().get_nodes_in_group("Beetle")
	
	if beetles.size() >= 5:
		return

	var new_beetle = load("res://Scenes/Beetle.tscn").instance()
	add_child(new_beetle)
	new_beetle.position = Vector2(320, 360)

	for beetle in beetles:
		new_beetle.position = beetle.position
		new_beetle.state = beetle.state
		new_beetle.area_position = beetle.area_position
		break

func _on_WasteTimer_timeout():
	var wastes = get_tree().get_nodes_in_group("Waste")
	
	if wastes.size() >= 5:
		return

	var waste = load("res://Scenes/Waste.tscn").instance()
	add_child(waste)
	waste.position = Vector2(rng.randf_range(0, 640), 0)
