import shutil
import os
os.makedirs("./pack",exist_ok=True)

shutil.copy("./sqls/data.sql","./pack/db.sql")
shutil.copy("./target/release/illustrators_support","./pack/illustrators_support")

import tarfile

with tarfile.open("pack.tar","w:gz")as fh:
    fh.add("./pack")