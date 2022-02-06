from os import linesep

gb_trace_log = open(
    "/Users/siddharth/Code/rust/gameboy_emulator/trace_log.txt")
rb_trace_log = open(
    "/Users/siddharth/Downloads/rboy-master/target/release/trace_log.txt")

rb_list = []
gb_list = []

for line in gb_trace_log:
    line_array = line.split(" ")
    gb_list.append({"A": int(line_array[1], 16), "F": int(line_array[3], 16), "BC": int(line_array[5], 16), "DE": int(line_array[7], 16), "HL": int(line_array[9], 16), "SP": int(line_array[11], 16), "PC": int(line_array[13], 16), "CY": int(line_array[15], 16), "Opcode": int(line_array[17].strip(), 16)})

i = 0
for line in rb_trace_log:
    if i < 61163:
        line_array = line.split(" ")
        rb_list.append({"A": int(line_array[1], 16), "F": int(line_array[3], 16), "BC": int(line_array[5], 16), "DE": int(line_array[7], 16), "HL": int(
            line_array[9], 16), "SP": int(line_array[11], 16), "PC": int(line_array[13], 16), "CY": int(line_array[15], 16), "Opcode": int(line_array[17].strip(), 16)})
        i += 1

for i in range(0, len(gb_list)):
    for key in rb_list[i]:
        if (key in gb_list[i] and rb_list[i][key] != gb_list[i][key]):
            print("Error on line " + str(i))
            print(gb_list[i])
            print(rb_list[i])
            exit()

print("Perfect!")
