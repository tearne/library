package vanilla;

import java.net.UnknownHostException;
import java.util.ArrayList;

import com.mongodb.BasicDBList;
import com.mongodb.BasicDBObject;
import com.mongodb.DB;
import com.mongodb.DBCollection;
import com.mongodb.DBObject;
import com.mongodb.MongoClient;

public class SpatialSearch {
	public static void main(String[] args) {
		MongoClient mongoClient = null;
		try {
			mongoClient = new MongoClient( "localhost" , 27017 );
		} catch (UnknownHostException e) {
			// TODO Auto-generated catch block
			e.printStackTrace();
		}
		DB database = mongoClient.getDB( "test" );
		
		for(String name : database.getCollectionNames()) 
			System.out.println(name);
		
		DBCollection collection = database.getCollection("spatial");
		collection.drop();
		collection.createIndex(new BasicDBObject(Holding.ID, 1), new BasicDBObject("unique", 1));
		collection.createIndex(
				new BasicDBObject("coords", "2d"),
				new BasicDBObject("min", "0").append("max", "100")
		);
		
		collection.insert(new Holding(1, 10, 10, 100, "Cows").toDBObject());
		collection.insert(new Holding(2, 10, 20, 100, "Sheep").toDBObject());
		collection.insert(new Holding(3, 10, 30, 100, "Pigs").toDBObject());
		collection.insert(new Holding(4, 10, 40, 100, "Ducks").toDBObject());
		collection.insert(new Holding(5, 10, 50, 100, "Elephants").toDBObject());
		collection.insert(new Holding(6, 10, 60, 100, "Zebra").toDBObject());
		collection.insert(new Holding(7, 10, 70, 100, "Mongoose").toDBObject());
//		collection.insert(new Holding(2, 10, 80, 100, "DuplicateHoldingId").toDBObject());

		BasicDBList coords = new BasicDBList();
		coords.add(10);
		coords.add(40);
		BasicDBList searchSpace = new BasicDBList();
		searchSpace.add(coords);
		searchSpace.add(19);
		
		
		DBObject spatialQuery = new BasicDBObject(Holding.COORDS,
			new BasicDBObject("$geoWithin", 
				new BasicDBObject("$center", searchSpace)
			)
		);
		
		System.out.println("Holdings within 19 units of (10, 40)");
		for(DBObject next : collection.find(spatialQuery)){
			Holding h = Holding.fromDBObject((BasicDBObject)next);
			System.out.println(" - "+h);
		}
	}
}

class Holding{
	public static final String ID = "holding_id";
	public static final String COORDS = "loc";
	private static final String SIZE = "size";
	private static final String TYPE = "type";
	
	private int id;
	private int easting;
	private int northing;
	private int size;
	private String type;

	public Holding(int id, int easting, int northing, int size, String type) {
		this.id = id;
		this.easting = easting;
		this.northing = northing;
		this.size = size;
		this.type = type;
	}
	
	public DBObject toDBObject() {
		DBObject o = new BasicDBObject();
		o.put(ID, id);
		o.put(COORDS, new int[]{easting, northing});
		o.put(SIZE, size);
		o.put(TYPE, type);
		
		return o;
	}
	
	public static Holding fromDBObject(BasicDBObject dbo) {
		@SuppressWarnings("unchecked")
		ArrayList<Integer> coordsList = (ArrayList<Integer>) dbo.get(COORDS);
		int x = coordsList.get(0);
		int y = coordsList.get(1);
		
		return new Holding(
			dbo.getInt(ID),
			x,
			y,
			dbo.getInt(SIZE),
			dbo.getString(TYPE)
		);
	}

	@Override
	public String toString() {
		StringBuilder bldr = new StringBuilder();
		bldr.append("Holding(");
		bldr.append(id);
		bldr.append(",");
		bldr.append(easting);
		bldr.append(",");
		bldr.append(northing);
		bldr.append(",");
		bldr.append(size);
		bldr.append(",");
		bldr.append(type);
		bldr.append(")");
		return bldr.toString();
	}
	
	
}
