from os import linesep


binjgb_trace_log = open(
    "/Users/siddharth/Downloads/binjgb-v0.1.11/bin/test.txt")
gb_trace_log = open("/Users/siddharth/Code/rust/gameboy_emulator/test.txt")
blarggs_trace_log = open(
    '/Users/siddharth/Downloads/Gameboy-logs-master/Blargg5.txt')

binjgb_list = []
gb_list = []
blarggs_list = []

for line in gb_trace_log:
    line_array = line.split(" ")
    gb_list.append({"A": int(line_array[1], 16), "F": line_array[3], "BC": int(line_array[5], 16), "DE": int(
        line_array[7], 16), "HL": int(line_array[9], 16), "Opcode": int(line_array[15].strip(), 16), "PC": int(line_array[13], 16)})
    # gb_list.append({"A": int(line_array[1], 16), "F": int(line_array[3], 16), "B": int(
    #     line_array[5], 16), "C": int(line_array[7], 16), "D": int(line_array[9], 16), "E": int(line_array[11], 16), "H": int(line_array[13], 16), "L": int(line_array[15], 16), "SP": int(line_array[17], 16), "PC": int(line_array[19], 16), "Opcode": int(line_array[20], 16)})


# for line in blarggs_trace_log:
#     line_array = line.split(" ")
#     blarggs_list.append({"A": int(line_array[1], 16), "F": int(line_array[3], 16), "B": int(
#         line_array[5], 16), "C": int(line_array[7], 16), "D": int(line_array[9], 16), "E": int(line_array[11], 16), "H": int(line_array[13], 16), "L": int(line_array[15], 16), "SP": int(line_array[17], 16), "PC": int(line_array[19].split(":")[1], 16), "Opcode": int(line_array[20].split("(")[1], 16)})


for line in binjgb_trace_log:
    line_array = line.split(" ")
    binjgb_list.append({"A": int(line_array[0].split(":")[1], 16), "F": line_array[1].split(":")[1], "BC": int(line_array[2].split(
        ":")[1], 16), "DE": int(line_array[3].split(":")[1], 16), "HL": int(line_array[4].split(":")[1], 16), "Opcode": int(line_array[11], 16), "PC": int(line_array[6].split(":")[1], 16)})

# for i in range(len(blarggs_list)):
#     for key in blarggs_list[i]:
#         if (key in gb_list[i] and blarggs_list[i][key] != gb_list[i][key]):
#             print("Error on line " + str(i))
#             print(gb_list[i])
#             print(blarggs_list[i])
#             exit()

for i in range(len(gb_list)):
    for key in binjgb_list[i]:
        if (key in gb_list[i] and binjgb_list[i][key] != gb_list[i][key]):
            print("Error on line " + str(i))
            print(gb_list[i])
            print(binjgb_list[i])
            exit()

print("Perfect!")
