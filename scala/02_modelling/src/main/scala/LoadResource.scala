import scala.io.Source

object LoadResource extends App {
  val resourceName = "my.resource"

  val resourceLines: Iterator[String] = Source.fromResource(resourceName).getLines()

  println("----- Resource -----")
  resourceLines.foreach(println(_))
}
