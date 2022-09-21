extends Node2D

enum SelectedFaction {NONE, ANT, BEETLE}

var selected_faction = SelectedFaction.NONE

var rng = RandomNumberGenerator.new()

func _ready():
	rng.randomize()

func _input(event):
	if event is InputEventKey and event.pressed:			
		if event.scancode == KEY_A:
			$CanvasLayer/MarginContainerAnts/AntsButton.emit_signal("pressed")
		if event.scancode == KEY_B:
			$CanvasLayer/MarginContainerBeetles/BeetlesButton.emit_signal("pressed")

		if event.scancode == KEY_N:
			$CanvasLayer/MarginContainerNorth/NorthButton.emit_signal("pressed")
		if event.scancode == KEY_S:
			$CanvasLayer/MarginContainerSouth/SouthButton.emit_signal("pressed")
		if event.scancode == KEY_E:
			$CanvasLayer/MarginContainerEast/EastButton.emit_signal("pressed")
		if event.scancode == KEY_W:
			$CanvasLayer/MarginContainerWest/WestButton.emit_signal("pressed")

func _on_AntSpawnTimer_timeout():
	var ants = get_tree().get_nodes_in_group("Ant")
	
	if ants.size() >= 10:
		return

	var new_ant = load("res://Scenes/Ant.tscn").instance()
	add_child(new_ant)
	new_ant.position = Vector2(200, 360)

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
	new_beetle.position = Vector2(440, 360)

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
	waste.position = Vector2(rng.randf_range(112, 528), 0)

func _process(_delta):
	var ants = get_tree().get_nodes_in_group("Ant")
	var wastes = get_tree().get_nodes_in_group("Waste")

	for ant in ants:
		for waste in wastes:
			if !waste.being_collected and !waste.being_carried and (ant.state == ant.State.GOING_TO_AREA or ant.state == ant.State.IDLE):
				# if this ant isn't collecting waste or moving it to the mushroom,
				# and this waste isn't being carried or collected,
				# and this ant is close to this waste, set this ant to collect this waste
				if ant.global_position.distance_to(waste.global_position) < 100.0:
					ant.waste = waste
					ant.state = ant.State.COLLECTING_GARBAGE
					waste.being_collected = true


func _on_NorthButton_pressed():
	if selected_faction == SelectedFaction.ANT:
		var ants = get_tree().get_nodes_in_group("Ant")
		for ant in ants:
			if ant.state == ant.State.GOING_TO_AREA or ant.state == ant.State.IDLE:
				ant.area_position = $AreaN.position + Vector2(rng.randf_range(-50.0, 50.0), rng.randf_range(-10.0, 50.0))
				ant.state = ant.State.GOING_TO_AREA

	if selected_faction == SelectedFaction.BEETLE:
		var beetles = get_tree().get_nodes_in_group("Beetle")
		for beetle in beetles:
			beetle.area_position = $AreaN.position
			beetle.state = beetle.State.GOING_TO_AREA

func _on_SouthButton_pressed():
	if selected_faction == SelectedFaction.ANT:
		var ants = get_tree().get_nodes_in_group("Ant")
		for ant in ants:
			if ant.state == ant.State.GOING_TO_AREA or ant.state == ant.State.IDLE:
				ant.area_position = $AreaS.position + Vector2(rng.randf_range(-50.0, 50.0), rng.randf_range(-50.0, 10.0))
				ant.state = ant.State.GOING_TO_AREA

	if selected_faction == SelectedFaction.BEETLE:
		var beetles = get_tree().get_nodes_in_group("Beetle")
		for beetle in beetles:
			beetle.area_position = $AreaS.position
			beetle.state = beetle.State.GOING_TO_AREA

func _on_EastButton_pressed():
	if selected_faction == SelectedFaction.ANT:
		var ants = get_tree().get_nodes_in_group("Ant")
		for ant in ants:
			if ant.state == ant.State.GOING_TO_AREA or ant.state == ant.State.IDLE:
				ant.area_position = $AreaE.position + Vector2(rng.randf_range(-50.0, 10.0), rng.randf_range(-50.0, 50.0))
				ant.state = ant.State.GOING_TO_AREA

	if selected_faction == SelectedFaction.BEETLE:
		var beetles = get_tree().get_nodes_in_group("Beetle")
		for beetle in beetles:
			beetle.area_position = $AreaE.position
			beetle.state = beetle.State.GOING_TO_AREA

func _on_WestButton_pressed():
	if selected_faction == SelectedFaction.ANT:
		var ants = get_tree().get_nodes_in_group("Ant")
		for ant in ants:
			if ant.state == ant.State.GOING_TO_AREA or ant.state == ant.State.IDLE:
				ant.area_position = $AreaW.position + Vector2(rng.randf_range(-10.0, 50.0), rng.randf_range(-50.0, 50.0))
				ant.state = ant.State.GOING_TO_AREA

	if selected_faction == SelectedFaction.BEETLE:
		var beetles = get_tree().get_nodes_in_group("Beetle")
		for beetle in beetles:
			beetle.area_position = $AreaW.position
			beetle.state = beetle.State.GOING_TO_AREA

func _on_AntsButton_pressed():
	selected_faction = SelectedFaction.ANT

func _on_BeetlesButton_pressed():
	selected_faction = SelectedFaction.BEETLE

func _on_FlowerTimer_timeout():
	var flower = load("res://Scenes/Flower.tscn").instance()
	add_child(flower)
	flower.position = Vector2(rng.randf_range(112, 528), rng.randf_range(150, 330))
