local game = remodel.readPlaceFile("test-models/place-with-models.rbxlx")
assert(game.Workspace.Camera ~= nil)

remodel.writePlaceFile(game, "temp/new-place.rbxlx")

local game2 = remodel.readPlaceFile("temp/new-place.rbxlx")
assert(game2.Workspace.Camera ~= nil)