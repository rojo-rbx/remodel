local source = [[
	{
		"x": 5,
		"y": "Hello, world!",
		"z": [1, 2, 3],
		"w": {
			"0": "cool",
			"wack": "yo"
		}
	}
]]

local decoded = json.fromString(source)

assert(decoded.x == 5)
assert(decoded.y == "Hello, world!")

assert(#decoded.z == 3)
for i = 1, 3 do
	assert(decoded.z[i] == i)
end

assert(type(decoded.w) == "table")
assert(decoded.w["0"] == "cool")
assert(decoded.w.wack == "yo")