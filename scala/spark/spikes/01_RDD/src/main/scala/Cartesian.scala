

import org.apache.spark.{SparkContext,SparkConf}
import org.apache.spark.rdd.RDD
import org.apache.commons.lang3.time.StopWatch
import org.apache.log4j.{Logger,Level}

object Cartesian {
  def main(args: Array[String]): Unit = {
    Logger.getLogger("org").setLevel(Level.OFF)
    Logger.getLogger("akka").setLevel(Level.OFF)
    
    case class Location(id: Int)
    case class Movement(from: Int, to: Int)
    
    val numLocations = 4
    def rndInt() = (math.random * numLocations + 1).toInt
    
    val locations = (1 to numLocations).map(Location.apply)
    val movements = (1 to 100000)
      .map(_ => Movement(rndInt,rndInt))
      .filter(mov => mov.to != mov.from)
    
    val stopwatch = new StopWatch()
      
    /*
     * Scala collection approach
     */
    stopwatch.start
    val result = locations.map{loc =>
      val relevantMovs = movements.filter(mov => mov.from == loc.id || mov.to == loc.id)  
      val numPresent = relevantMovs.foldLeft(100){case (acc, Movement(from, to)) => 
        if(to == loc.id) (acc + 1) 
        else (acc - 1)
      }
      loc -> numPresent
    }
    stopwatch.stop
    
    println("Scala collections: "+stopwatch.getTime)
    result.foreach(println)
    
    
    /*
     * Spark cartesian approach
     */
    val sc = new SparkContext(new SparkConf().setAppName("Test").setMaster("local"))
    val locs: RDD[Location] = sc.parallelize(locations)
    val movs: RDD[Movement] = sc.parallelize(movements)
    
    stopwatch.reset
    stopwatch.start
    val t = locs.cartesian(movs)
      .filter{case (loc, mov) => mov.from == loc.id || mov.to == loc.id}
      .aggregateByKey(Seq.empty[Movement])(
          (acc, mov) => mov +: acc, 
          (s1, s2) => s1 ++: s2
      )
      .map{case (loc, movs) => 
        val numPresent = movs.foldLeft(100){case (acc, Movement(from, to)) =>
          if(to == loc.id) (acc + 1) 
          else (acc - 1)
        }
        loc -> numPresent
      }
    stopwatch.stop
     
    println("Spark collect: "+stopwatch.getTime)
    t.collect.sortBy(_._1.id).foreach(println)
  }
}