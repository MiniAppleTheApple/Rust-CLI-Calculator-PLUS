string = list(input())
txt = ""
oper = ""
parsed = []
for char in string:
    for num in list("1234567890"):
        if char == num:
            txt += char
    for operator in list("+-/*"):
        if char == operator:
            parsed.append(txt)
            txt = ""
            oper = char
parsed.append(txt)
print(parsed)
print(oper)
if oper == "+":
    print(f"{int(parsed[0]) + int(parsed[1])}")
elif oper == "-":
    print(f"{int(parsed[0]) - int(parsed[1])}")
elif oper == "*":
    print(f"{int(parsed[0]) * int(parsed[1])}")
elif oper == "/":
    print(f"{int(parsed[0]) / int(parsed[1])}")