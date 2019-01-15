package anonymise

import java.nio.file.Files
import java.nio.file.Paths
import java.nio.charset.Charset
import scala.annotation.tailrec
import java.nio.file.Path
import scala.util.Try
import scala.collection.mutable.Map

object IndexCPH extends App{
  val inPath = Paths.get("dataIn", "anonymise", "dummyIn.csv")
  val reader = Files.newBufferedReader(inPath, Charset.defaultCharset())
  val numLines = CountLines(inPath)
  val tenPercent = (numLines * 0.1).toInt
  
  val outPath = Paths.get("dataOut", "anonymise")
  val movsOutFile = outPath.resolve( "movements.csv")
  val locationsOutPath = outPath.resolve("locations_jitter.csv")
  val cphOutPath = outPath.resolve("cph.csv")
  
  Files.createDirectories(outPath)
  val writer = Files.newBufferedWriter(movsOutFile, Charset.defaultCharset())
  writer.write("Ani, Date, Off, Trans, On")
  writer.newLine
 
  def getAsOption(array: Array[String], id: Int): Option[String] = {
    if(array.size > id) Some(array(id))
    else None
  }
  
  val locs = new Locations()
  val anis = new Animals()
  
  trait Node{
  	val cphColumnIdx: Int
  	lazy val (eastingColumnIdx, northingColumnIdx) = (cphColumnIdx - 2, cphColumnIdx - 1)
  	lazy val typeColumnIdx = cphColumnIdx - 3
  }
  case object On extends Node{
  	val cphColumnIdx = 10
  }
  case object Off extends Node{
  	val cphColumnIdx = 5
  }
  case object Transition extends Node{
  	val cphColumnIdx = 15
  }
  
  def parseCoords(node: Node, toks: Seq[String]) = {
    val easting = Try(toks(node.eastingColumnIdx).toInt).toOption
    val northing = Try(toks(node.northingColumnIdx).toInt).toOption
    easting.flatMap(e => northing.map(n => (e,n)))
  }
  
  def getElseCreateId(node: Node, toks: Seq[String]): Int = {
  	val cph = toks(node.cphColumnIdx)
  	locs.getId(cph) match {
      	case Some(id) => id
      	case None =>
      		locs.generateNewId(
      			cph,
      			parseCoords(node, toks)
      		)
      }
  }
  
  @tailrec
 	def processLine(line: String, counter: Int = 1) {
  	if(line != null) {
  		try{
        val toks = line.split(",")
        
        //Need to check with Adam what happens with birth movements
        val offIdx = getElseCreateId(Off, toks).toString()
        
        val onIdx = 
        	if(toks.size > On.cphColumnIdx) getElseCreateId(On, toks).toString
        	else ""

        val transIdx =
        	if(toks.size > Transition.cphColumnIdx) getElseCreateId(Transition, toks).toString
        	else ""
        
        val animalIdx = anis.getOrCreateIdx(toks(0).toInt)
        val date = toks(1).takeWhile(_ != ' ')
        
        writer.write(s"$animalIdx, $date, $offIdx, $transIdx, $onIdx")
        writer.newLine()
      } catch {
        case e: Throwable => println(s"Line num $counter is bad: $line () $e")
          throw new Exception(e)
      }
      if(counter % tenPercent == 0) println(f"done ${100.0 * counter / numLines}%.1f percent")
      processLine(reader.readLine(), counter + 1)
  	}
  }
  
  println(numLines+" lines to process")
  println(s"Header")
  reader.readLine.split(",").zipWithIndex.foreach{case (title, id) => println(s" - $id, $title")}
  
  val start = System.currentTimeMillis()
  processLine(reader.readLine)
  writer.close
  println(s"Processing took ${(System.currentTimeMillis() - start)/1000.0} s")  
  
  locs.saveLocations(locationsOutPath, true)
  locs.saveCPHtable(cphOutPath)
}

object CountLines {
  def apply(path: Path): Int = {
	  val reader = Files.newBufferedReader(path, Charset.defaultCharset())
	
	  Iterator
	    .continually(reader.readLine)
	    .takeWhile { _ != null }
	    .foldLeft(0){case (a,b) => a + 1}
  }  
}

class Animals {
	private val animalIdxById = collection.mutable.Map[Int, Int]()
	
	def getOrCreateIdx(id: Int) = {
		if(!animalIdxById.contains(id)){
			val newIdx = animalIdxById.size
			animalIdxById.put(id, newIdx)
		}
		animalIdxById(id)
	}
	
	def saveTo(path: Path){
		throw new Exception("TODO")
	}
}

class Locations {
	case class Location(cph: String, idx: Int, coords: Option[(Int,Int)])
	
	import scala.collection.mutable.Map
	private val locationsByCph = Map[String, Location]()
	
	def getId(cph: String): Option[Int] = locationsByCph.get(cph).map(_.idx)
	
	def generateNewId(cph: String, coords: Option[(Int, Int)]): Int = {
		if(!locationsByCph.contains(cph)){
			val newLocation = Location(cph, locationsByCph.size, coords)
			locationsByCph.put(cph, newLocation)
			newLocation.idx
		}
		else throw new Exception(s"CPH $cph is already indexed")
	}
	
	def saveCPHtable(path: Path){
    val writer = Files.newBufferedWriter(path, Charset.defaultCharset())
    writer.write("CPH, idx")
    writer.newLine
    
		locationsByCph.foreach{case (cph, Location(_,idx,_)) =>
			writer.write(cph+","+idx)
      writer.newLine
		}
    writer.close
	}
  
  def saveLocations(path: Path, jitter: Boolean = true){
    def getCoords(coordOpt: Option[(Int, Int)]) = coordOpt.getOrElse(("",""))
    def getJitteredCoords(coordOpt: Option[(Int, Int)]): (String, String) = {
      coordOpt
        .map{case (east, north) =>
          val newEast = east + 5
          val newNorth = north + 10
          (newEast, newNorth)  //TODO something better than this!
        }
        .map{case (east, north) =>
          (east.toString, north.toString)
        }
        .getOrElse(("", ""))
    }

    val coordinateExtractor = {
      if(jitter) getJitteredCoords _
      else getCoords _
    }
    
    val writer = Files.newBufferedWriter(path, Charset.defaultCharset())
    writer.write("idx, easting, northing")
    writer.newLine
    
    locationsByCph.foreach{case (cph, Location(_,idx, coordOpt)) =>
      val (east, north) = coordinateExtractor(coordOpt)
      writer.write(s"$idx, $east, $north")
      writer.newLine
    }
    writer.close
  }
}