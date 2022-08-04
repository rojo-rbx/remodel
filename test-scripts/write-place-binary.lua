local game = remodel.readPlaceFile("test-models/place-with-models-binary.rbxl")
assert(game.Workspace.Camera ~= nil)

remodel.writePlaceFile("temp/new-place-binary.rbxl", game)

local game2 = remodel.readPlaceFile("temp/new-place-binary.rbxl")
assert(game2.Workspace.Camera ~= nil)