local model = remodel.readModelFile("test-models/folder-and-value.rbxmx")[1]

assert(model.Name == "Root")

model.Name = "Foo"

assert(model.Name == "Foo")