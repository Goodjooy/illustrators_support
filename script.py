from genericpath import exists
import os

os.makedirs("./sqls",exist_ok=True)

files=[os.path.join("./migrations",x) for x in os.listdir("./migrations")]

with open("data.sql",'w')as sql:
    for fn in files:
        with open(fn,"r")as src:
            sql.write( src.read())
            sql.write("\n-- next table \n")

print("conbin sql DONE")

os.makedirs("./pack",exist_ok=True)

import shutil

shutil.copyfile("./sqls/data.sql","./pack/db.sql")
shutil.copyfile("./target/release/illustrators_support", "./pack/illustrators_support")

import tarfile

file=tarfile.open("./pack.tar","w:gz")
file.add("./pack/db.sql")
file.add("./pack/illustrators_support")

file.close()

print("package Files DONE")