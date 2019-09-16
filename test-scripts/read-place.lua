local game = remodel.readPlaceFile("test-models/place-with-models.rbxlx")

assert(game.Parent == nil)
assert(game.ClassName == "DataModel")

assert(game.ReplicatedStorage.Name == "ReplicatedStorage")
assert(game.Workspace.Baseplate.Name == "Baseplate")
