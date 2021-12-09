import tarfile
import shutil
import os

print("building START")

with os.popen("cargo build --release") as info:
    pass

print("building DONE")

print("conbin sql START")
os.makedirs("./sqls", exist_ok=True)

files = [os.path.join("./migrations", x) for x in os.listdir("./migrations")]

with open("./sqls/data.sql", 'w')as sql:
    for fn in files:
        with open(fn, "r")as src:
            sql.write(src.read())
            sql.write("\n-- next table \n")

print("conbin sql DONE")

os.makedirs("./pack", exist_ok=True)

print("package Files START")

shutil.copyfile("./sqls/data.sql", "./pack/db.sql")
shutil.copyfile("./target/release/illustrators_support",
                "./pack/illustrators_support")


file = tarfile.open("./pack.tar", "w:gz")
file.add("./pack/db.sql")
file.add("./pack/illustrators_support")

file.close()

print("package Files DONE")
