local game = remodel.load("test-models/folder-and-value.rbxmx")

local ok, err = pcall(function()
	game.ClassName = "Nah"
end)

assert(not ok)
assert(tostring(err):find("ClassName") ~= nil)