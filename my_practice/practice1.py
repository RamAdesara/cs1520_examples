a_dict = {1:'one', 2:'two', 3:'three'}
list_of_tups = sorted(a_dict.items(), key=lambda t: t[1])

for i in list_of_tups:
    print(i)