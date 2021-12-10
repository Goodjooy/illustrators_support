import os

print("conbin sql START")
os.makedirs("./sqls", exist_ok=True)

files = [os.path.join("./migrations", x) for x in os.listdir("./migrations")]

with open("./sqls/data.sql", 'w')as sql:
    for fn in files:
        with open(fn, "r")as src:
            sql.write(src.read())
            sql.write("\n-- next table \n")

print("conbin sql DONE")


