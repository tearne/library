import java.nio.file.Paths

import org.apache.commons.io.FileUtils
import play.api.libs.json.Json
import sampler.distribution.Distribution
import sampler.maths.Random
import sampler._

object Integer extends App {
  implicit val r = Random

  val integer: Seq[Int] = {
    val d = for{
      numSuccesses <- Distribution.uniform(5,10)
      v <- Distribution.hypergeometric(100, 200, numSuccesses)
    } yield v
    (1 to 100).map(_ => d.sample)
  }

  FileUtils.writeStringToFile(
    Paths.get("app", "data", "integer.json").toFile,
    Json.prettyPrint(Json.obj(
      "integer" -> integer
    ))
  )
}
