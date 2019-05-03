import scala.io.Source

//  Loading a configs.json from resources.  Tested to work within both
//  1) IntelliJ IDEA
//
//  AND
//
//  2) On the command line with jars and configs.json in a lib dir: java -cp "lib/*:lib" LoadResource
//  where lib contains jars and resources
//
//  To build far jar just "clean" and "package" and jar will appear in target/scala-2.xx
//  Alternatively, if using the native packager, run "stage" and look in target/universal/stage

object LoadResource extends App {
  val resourceName = "my.resource"

  val resourceLines: Iterator[String] = Source.fromResource(resourceName).getLines()

  println("----- Resource -----")
  resourceLines.foreach(println(_))
}

