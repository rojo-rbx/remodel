-- One use for Remodel is to move the terrain of one place into another place.
local inputGame = remodel.readPlaceFile("input-place.rbxlx")
local outputGame = remodel.readPlaceFile("output-place.rbxlx");

-- This isn't possible inside Roblox, but works just fine in Remodel!
outputGame.Terrain:Destroy()
inputGame.Terrain.Parent = outputGame

remodel.writePlaceFile(outputGame, "output-place-updated.rbxlx")