local game = remodel.readPlaceFile("test-models/place-with-models-binary.rbxl")
assert(game.Workspace.Camera ~= nil)

remodel.writePlaceFile(game, "temp/new-place-binary.rbxl")

local game2 = remodel.readPlaceFile("temp/new-place-binary.rbxl")
assert(game2.Workspace.Camera ~= nil)