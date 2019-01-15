import java.nio.file.Paths

import org.apache.commons.io.FileUtils
import play.api.libs.json.Json
import sampler.distribution.Distribution
import sampler.maths.Random
import sampler._

object Real extends App {
  implicit val r = Random

  val real: Seq[Double] = {
    val d = Distribution.exponential(700)
        .flatMap(v => Distribution.exponential(v))
        .map(_.decimalPlaces(3))
    (1 to 100).map(_ => d.sample)
  }

  FileUtils.writeStringToFile(
    Paths.get("app", "data", "real.json").toFile,
    Json.prettyPrint(Json.obj(
      "real" -> real
    ))
  )
}
