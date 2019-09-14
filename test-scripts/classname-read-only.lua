local model = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

local ok, err = pcall(function()
	model.ClassName = "Nah"
end)

assert(not ok)
assert(tostring(err):find("ClassName") ~= nil)