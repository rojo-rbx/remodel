local baseline = {
	x = 5,
	y = "Hello, world!",
	z = {1, 2, 3},
	w = {
		[0] = "cool",
		wack = "yo",
	},
}

local encodedBaseline = json.toString(baseline)
assert(encodedBaseline == [[{"w":{"0":"cool","wack":"yo"},"x":5.0,"y":"Hello, world!","z":[1.0,2.0,3.0]}]])

local sparse = {1, 2, nil, 4}
local encodedSparse = json.toString(sparse)
assert(encodedSparse == "[1.0,2.0,null,4.0]")

local sparser = {1, 2, 3, [1000] = 6}
local encodedSparser = json.toString(sparser)
assert(encodedSparser == [[{"1":1.0,"1000":6.0,"2":2.0,"3":3.0}]])

local encodedPretty = json.toStringPretty(baseline)
assert(encodedPretty == [[{
  "w": {
    "0": "cool",
    "wack": "yo"
  },
  "x": 5.0,
  "y": "Hello, world!",
  "z": [
    1.0,
    2.0,
    3.0
  ]
}]])

local encodedPrettyWithIndent = json.toStringPretty(baseline, "\t")
assert(encodedPrettyWithIndent == [[{
	"w": {
		"0": "cool",
		"wack": "yo"
	},
	"x": 5.0,
	"y": "Hello, world!",
	"z": [
		1.0,
		2.0,
		3.0
	]
}]])
