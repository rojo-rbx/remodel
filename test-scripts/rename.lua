local game = remodel.load("test-models/folder-and-value.rbxmx")

assert(game.Name == "DataModel")

game.Name = "Foo"

assert(game.Name == "Foo")