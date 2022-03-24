import argparse
import os, subprocess
import glob

parser = argparse.ArgumentParser(description='Import a directory of stuffs to MongoDB.')

parser.add_argument('dir', help='upload directory')
parser.add_argument('-d', dest="database", help='database')
parser.add_argument('-c', dest="collection", help='collection')
parser.add_argument('-g', dest="glob", help='glob', default="*")

args = parser.parse_args()
print(args)

os.chdir(args.dir)

for file in glob.glob(args.glob):
    abspath = os.path.abspath(file)
    #print("insert", abspath)
    subprocess.run([
        "mongoimport",
        "-d", args.database,
        "-c", args.collection,
        "--file", abspath
    ])
   

