local game = remodel.readPlaceFile("test-models/place-with-models.rbxlx")

local success, problem = pcall(function()
	remodel.writePlaceFile(game, "temp/new-place.rbxlx")
end)

assert(not success)
assert(tostring(problem):match("The two arguments are swapped") ~= nil)

success, problem = pcall(function()
	remodel.writeModelFile(game, "temp/new-model.rbxmx")
end)

assert(not success)
assert(tostring(problem):match("The two arguments are swapped") ~= nil)
