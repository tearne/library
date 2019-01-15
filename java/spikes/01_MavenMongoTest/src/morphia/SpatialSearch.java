package morphia;

import java.net.UnknownHostException;

import org.mongodb.morphia.Datastore;
import org.mongodb.morphia.Morphia;
import org.mongodb.morphia.annotations.Entity;
import org.mongodb.morphia.annotations.Id;
import org.mongodb.morphia.annotations.Indexed;
import org.mongodb.morphia.query.Shape;
import org.mongodb.morphia.utils.IndexDirection;

import com.mongodb.MongoClient;

public class SpatialSearch {
	public static void main(String[] args){
		Morphia morphia = new Morphia();
		morphia.map(Holding.class);
		Datastore ds;
		try {
			ds = morphia.createDatastore(new MongoClient( "localhost" , 27017 ), "test");
		} catch (UnknownHostException e) {
			throw new RuntimeException("Badness", e);
		}
		ds.getCollection(Holding.class).drop();
		ds.ensureIndexes();
		
		ds.save(new Holding(1, 10, 10, 100, "Cows"));
		ds.save(new Holding(2, 10, 20, 200, "Sheep"));
		ds.save(new Holding(3, 10, 30, 300, "Pigs"));
		ds.save(new Holding(4, 10, 40, 400, "Ducks"));
		ds.save(new Holding(5, 10, 50, 500, "Elephants"));
		ds.save(new Holding(6, 10, 60, 600, "Zebra"));
		ds.save(new Holding(7, 10, 70, 700, "Mongoose"));
		ds.save(new Holding(2, 10, 25, 800, "!!Overwritten Sheep!!"));
		
		Shape.Point point = new Shape.Point(10,40);
		for(Holding h : ds.find(Holding.class).field("coords").within(Shape.center(point, 19))){
			System.out.println(h);
		}
	}
}

@Entity
class Holding{
	
	@Id
	@Indexed(unique = true)
	private int id;
	
	@Indexed(IndexDirection.GEO2D) 
	private int[] coords;
	
	private int size;
	private String type;

	@SuppressWarnings("unused")
	private Holding(){}
	
	public Holding(int id, int easting, int northing, int size, String type) {
		this.id = id;
		this.coords = new int[]{easting, northing};
		this.size = size;
		this.type = type;
	}

	@Override
	public String toString() {
		StringBuilder bldr = new StringBuilder();
		bldr.append("Holding(");
		bldr.append(id);
		bldr.append(", ");
		bldr.append("("+coords[0]+","+coords[1]+")");
		bldr.append(", ");
		bldr.append(size);
		bldr.append(", ");
		bldr.append(type);
		bldr.append(")");
		return bldr.toString();
	}
}
