import java.nio.file.Paths

import org.apache.commons.io.FileUtils
import play.api.libs.json.Json
import sampler.distribution.Distribution
import sampler.maths.Random
import sampler._

object HeatMap extends App {
  implicit val r = Random

  case class HeatCellLowerCorner(x: Double, y: Double, count: Int)
  object HeatCellLowerCorner{
    implicit val writes = Json.writes[HeatCellLowerCorner]
  }
  case class HeatMap(xBinWidth: Double, yBinWidth: Double, cellCounts: Seq[HeatCellLowerCorner])
  object HeatMap{
    implicit val writes = Json.writes[HeatMap]
  }
  val heatMap = {
    val dist =
      for{
        x <- Distribution.uniform(100.0, 400.0)
        y <- Distribution.uniform(10.0, 20.0)
      } yield (x, y * y)
    val samples = (1 to 100000).map(_ => dist.sample)
    val numBins = 100
    val xWidth = {
      val xs = samples.map(_._1)
      (xs.max - xs.min) / numBins.toDouble
    }
    val yWidth = {
      val ys = samples.map(_._2)
      (ys.max - ys.min) / numBins.toDouble
    }

    val binnedCounts: Seq[HeatCellLowerCorner] = samples
        .groupBy{case (x,y) => ((x/xWidth).toInt, (y/yWidth).toInt)}
        .map{case (pt, values) => HeatCellLowerCorner(
          (xWidth * pt._1).decimalPlaces(2),
          (yWidth * pt._2).decimalPlaces(2),
          values.size)}
        .toSeq
    HeatMap(xWidth,yWidth,binnedCounts)
  }

  FileUtils.writeStringToFile(
    Paths.get("app", "data", "heat-map.json").toFile,
    Json.prettyPrint(Json.obj(
      "heat-map" -> heatMap
    ))
  )
}
