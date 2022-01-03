from os import linesep

binjgb_trace_log = open("/Users/siddharth/Downloads/binjgb-v0.1.11/bin/test.txt")
gb_trace_log = open("/Users/siddharth/Code/rust/gameboy_emulator/test.txt")

binjgb_list = []
gb_list = []
blarggs_list = []

for line in gb_trace_log:
    line_array = line.split(" ")
    gb_list.append({"A": int(line_array[1], 16), "F": line_array[3], "BC": int(line_array[5], 16), "DE": int(line_array[7], 16), "HL": int(line_array[9], 16), "Opcode": int(line_array[15].strip(), 16), "PC": int(line_array[13], 16), "CY": line_array[19].strip()})

#  for i in range(0, 2602291):
    #  line_array = gb_trace_log.readline().split(" ")
    #  gb_list.append({"A": int(line_array[1], 16), "F": line_array[3], "BC": int(line_array[5], 16), "DE": int(line_array[7], 16), "HL": int(line_array[9], 16), "Opcode": int(line_array[15].strip(), 16), "PC": int(line_array[13], 16), "CY": line_array[19].strip()})

for line in binjgb_trace_log:
    line_array = line.split(" ")
    binjgb_list.append({"A": int(line_array[0].split(":")[1], 16), "F": line_array[1].split(":")[1], "BC": int(line_array[2].split(":")[1], 16), "DE": int(line_array[3].split(":")[1], 16), "HL": int(line_array[4].split(":")[1], 16), "Opcode": int(line_array[11], 16), "PC": int(line_array[6].split(":")[1], 16), "CY": line_array[8].split(")")[0]})

for i in range(0, len(gb_list)):
    for key in binjgb_list[i]:
        if (key in gb_list[i] and binjgb_list[i][key] != gb_list[i][key]):
            print("Error on line " + str(i))
            print(gb_list[i])
            print(binjgb_list[i])
            exit()

print("Perfect!")
