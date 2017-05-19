# Source of values.txt: 'https://pastebin.com/api/'
# I handled the output myself to get a Rust compatible Enum

with open('values.txt', 'r') as myfile:
    data = myfile.read()
    data = data.split("\n")
    for d in data:
        result = d.split(" = ")
        print(result[1] + ",")
