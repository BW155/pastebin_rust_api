# Source of values.txt: 'https://pastebin.com/api/'

values = []
with open('values.txt', 'r') as myfile:
    data = myfile.read()
    data = data.split("\n")
    for d in data:
        result = d.split(" = ")
        values.append(result[0].replace(" ", ""))

# rust_formats.txt is the list of the Enum present in src/paster/format.rs
with open('rust_formats.txt', 'r') as myfile:
    data = myfile.read()
    data = data.replace("\n", "").replace(" ", "")
    data = data.split(",")
    i = 0
    for d in data:
        print("&Format::" + d + " => \"" + values[i] + "\",")
        i += 1
