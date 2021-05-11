local function assertString(result, expected)
    assert(result == expected, ('expected `%s` but got `%s`'):format(expected, result))
end

local FOLDER_A_NAME = "foo"
local FOLDER_B_NAME = "bar"

local folderA = Instance.new("Folder")
folderA.Name = FOLDER_A_NAME

assertString(folderA:GetFullName(), FOLDER_A_NAME)

local folderB = Instance.new("Folder")
folderB.Name = FOLDER_B_NAME
folderB.Parent = folderA

assertString(folderB:GetFullName(), FOLDER_A_NAME .. '.' .. FOLDER_B_NAME)

local game = remodel.readPlaceFile("test-models/place-with-models.rbxlx")

assertString(game.Workspace.Baseplate:GetFullName(), "Workspace.Baseplate")
