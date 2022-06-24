local game = remodel.readPlaceFile("test-models/place-with-models.rbxlx")
assert(game.Workspace.Camera ~= nil)

remodel.writePlaceFile("temp/new-place.rbxlx", game)

local game2 = remodel.readPlaceFile("temp/new-place.rbxlx")
assert(game2.Workspace.Camera ~= nil)