local a = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]
local b = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

assert(a.Name == "Root")
assert(a ~= b)

local stringA = a.String

a.Parent = b

-- Make sure our reference to stringA wasn't invalidated
assert(stringA.Name == "String")

-- Make sure the root instance actually moved
assert(b.Root == a)