-- One use for Remodel is to pull data out of a place file and produce multiple
-- model files from it. This can be a hold-over for two-way sync in Rojo or to
-- sync models between multiple places!
--
-- Remodel can read place files with remodel.readPlaceFile. This returns a
-- DataModel instance, also known as game in Roblox!
local game = remodel.readPlaceFile("test-models/place-with-models.rbxlx")

-- In this example, we have a bunch of models stored in
-- ReplicatedStorage.Models. We want to put them into a folder named models,
-- maybe for a tool like Rojo.
local Models = game.ReplicatedStorage.Models
remodel.createDirAll("temp/models")

for _, model in ipairs(Models:GetChildren()) do
	remodel.writeModelFile("temp/models/" .. model.Name .. ".rbxmx", model)
end

-- And that's it!