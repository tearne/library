import java.nio.file.Paths

import org.apache.commons.io.FileUtils
import play.api.libs.json.{JsValue, Json, Writes}
import sampler.distribution.Distribution
import sampler.maths.Random
import sampler._

object Matrix extends App {
  implicit val r = Random

  case class Cell(x: Int, y: Int, value: Int)
  object Cell{
    implicit val writes: Writes[Cell] = Json.writes[Cell]
  }
  val matirx: Seq[JsValue] = {
    val d = Distribution.geometric(0.6)
    for{
      i <- 1 to 10
      j <- 1 to 10
    } yield {
      Json.toJson(Cell(i, j, d.sample))
    }
  }

  FileUtils.writeStringToFile(
    Paths.get("app", "data", "matrix.json").toFile,
    Json.prettyPrint(Json.obj(
      "matrix" -> matirx
    ))
  )

}
