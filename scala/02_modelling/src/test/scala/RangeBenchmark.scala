
import org.scalameter.api._

object RangeBenchmark extends Bench.LocalTime {
  val sizes = Gen.range("size")(300000, 1500000, 300000)

  val ranges = for {
    size <- sizes
  } yield 0 until size

  performance of "Range" in {
    measure method "map" in {
      using(ranges) in {
        r => r.map(_ + 1)
      }
    }
  }
}

object ConfigBased extends App /*with Bench.LocalTime*/ {
  import org.scalameter.{Key, config, Warmer}

  val benchConfig = config(
    Key.exec.minWarmupRuns -> 100,
    Key.exec.maxWarmupRuns -> 300,
    Key.exec.benchRuns -> 2000
  ).withWarmer(new Warmer.Default)

  val t = benchConfig.measure{
    (1 to 100).foldLeft(List.empty[Int]){case (acc, next) => next +: acc}
  }

  println(t)
}