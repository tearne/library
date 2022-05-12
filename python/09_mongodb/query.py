import pymongo
import argparse
from pprint import pprint

parser = argparse.ArgumentParser(description='Read stuffs from MongoDB.')
parser.add_argument('-d', dest="database", help='database')
parser.add_argument('-c', dest="collection", help='collection')
args = parser.parse_args()

client = pymongo.MongoClient("mongodb://localhost:27017")

db=client[args.database]
pprint(db)
pprint("--------")

collection = db[args.collection]
pprint(collection)
pprint("--------")

result=collection.find({})

pprint(result[1]["meta"])