local game = remodel.load("test-models/folder-and-value.rbxmx")

local Root = game.Root
assert(Root ~= nil)
assert(Root.Name == "Root")

local String = Root.String
assert(String ~= nil)
assert(String.Name == "String")

local ok, err = pcall(function()
	return String.DoesNotExist
end)

assert(not ok)
assert(tostring(err):find("DoesNotExist") ~= nil)