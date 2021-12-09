import tarfile
import shutil
import os
import logging

logging.info("building START")

with os.popen("cargo build --release") as info:
    while True:
        s = info.readline()
        if s == "":
            break
        logging.info(s)

logging.info("building DONE")

logging.info("conbin sql START")
os.makedirs("./sqls", exist_ok=True)

files = [os.path.join("./migrations", x) for x in os.listdir("./migrations")]

with open("./sql/data.sql", 'w')as sql:
    for fn in files:
        with open(fn, "r")as src:
            sql.write(src.read())
            sql.write("\n-- next table \n")

logging.info("conbin sql DONE")

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
