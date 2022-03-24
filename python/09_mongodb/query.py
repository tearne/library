import pymongo

parser = argparse.ArgumentParser(description='Read stuffs from MongoDB.')
parser.add_argument('-d', dest="database", help='database')
parser.add_argument('-c', dest="collection", help='collection')

client = MongoClient("mongodb://localhost:27017")

db=client.otdb
result=db.testing.find()
pprint(result)