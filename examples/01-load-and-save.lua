-- We can load models from a file with remodel.load
local game = remodel.load("test-models/folder-and-value.rbxmx")

-- game is a DataModel -- remodel.load always wraps its return value in a
-- DataModel instance because models and places can contain multiple top-level
-- instances.
print("game is a ", game.ClassName)

-- Remodel is a subset of Roblox's API with stricter semantics.
-- You should feel right at home if you work on Roblox!
local Root = game:FindFirstChild("Root")
local stringValue = Root:FindFirstChild("String")

print("Found string value: ", stringValue.Name)

-- We can save models back to disk with remodel.save
remodel.save(stringValue, "temp/just-a-stringvalue.rbxmx")