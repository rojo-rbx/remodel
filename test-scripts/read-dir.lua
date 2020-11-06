local testScripts = remodel.readDir("test-scripts")

local fileCount = 0
local foundSelf = false
for _, file in ipairs(testScripts) do
	if file == "read-dir.lua" then
		foundSelf = true
	end

	fileCount = fileCount + 1
end

assert(foundSelf)
assert(fileCount > 5)