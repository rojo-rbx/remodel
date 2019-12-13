assert(remodel.isDir("test-scripts"))
assert(remodel.isFile("test-scripts/is-file-dir.lua"))

local ok = pcall(function() remodel.isDir("does-not-exist") end)
assert(not ok)

local ok = pcall(function() remodel.isFile("does-not-exist") end)
assert(not ok)