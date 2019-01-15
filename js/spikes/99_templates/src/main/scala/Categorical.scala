import java.nio.file.Paths

import org.apache.commons.io.FileUtils
import play.api.libs.json.{JsValue, Json}
import sampler.distribution.Distribution
import sampler._
import sampler.maths.Random

object Categorical extends App {
  implicit val r = Random

  case class Count(variable: String, count: Int)
  object Count{
    implicit val writes = Json.writes[Count]
  }
  val categorical: JsValue = {
    trait Colour
    case object Red extends Colour
    case object Green extends Colour
    case object Blue extends Colour

    val bag = Map(Red -> 100, Green -> 200, Blue -> 300)
    val countsDist = Distribution.from(implicit r =>
      bag.draw(100).drawnCounts.map{case (k,v) => Count(k.toString, v)}
    )
    Json.toJson(countsDist.sample)
  }

  FileUtils.writeStringToFile(
    Paths.get("app", "data", "categorical.json").toFile,
    Json.prettyPrint(Json.obj(
      "categorical" -> categorical
    ))
  )
}
