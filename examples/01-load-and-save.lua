-- We can load models from a file with remodel.readModelFile.
-- Model files can contain multiple top-level instances so it returns a list.
local root = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

-- Remodel is a subset of Roblox's API with stricter semantics.
-- You should feel right at home if you work on Roblox!
local stringValue = root:FindFirstChild("String")

print("Found string value: ", stringValue.Name)

-- We can save models back to disk with remodel.writeModelFile
remodel.writeModelFile("temp/just-a-stringvalue.rbxmx", stringValue)