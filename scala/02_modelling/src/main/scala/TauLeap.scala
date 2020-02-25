import java.nio.file.{Path, Paths}
import java.time.LocalDate
import java.time.format.DateTimeFormatter

import breeze.stats.distributions.Poisson
import demoODESolver.SolutionPoint
import org.apache.commons.io.FileUtils
import play.api.libs.json.Json
import sampler.r.script.RScript

trait TauLeap {

  def computeNextStep(y: Array[Double], timeStep: Double): Array[Double]

  def solve(y0: Array[Double], t0: Double, t1: Double, stepSize: Double): IndexedSeq[(Double, Array[Double])] = {

    val times = BigDecimal(t0) to t1 by stepSize
    val initialCondition = (t0, y0)

    times.tail.foldLeft(IndexedSeq(initialCondition)) { case (acc, nextTime) => {
      val (prevTime, prevY) = acc.last
      val newY = computeNextStep(prevY, (nextTime - prevTime).toDouble)
      acc :+ (nextTime.toDouble, newY)
    }
    }
  }
}

object DemoTauLeapSolver extends App {

  case class Parameters(beta: Double, gamma: Double)

  val p = Parameters(0.001, 0.1)

  val startTime: Double = 0
  val endTime: Double = 50
  val stepSize: Double = 5

  val y0 = Array(500.0, 10.0, 0.0)

  case class MyTauLeap(p: Parameters) extends TauLeap {
    def computeNextStep(y: Array[Double], timeStep: Double): Array[Double] = {
      val StoI = Math.min(Poisson(p.beta * y(0) * y(1) * timeStep).sample, y(0))
      // cannot convert more than available
      val ItoC = Math.min(Poisson((p.gamma * y(1)) * timeStep).sample, y(1))
      val dy0 = y(0) - StoI
      val dy1 = y(1) + StoI - ItoC
      val dy2 = y(2) + ItoC

      Array(dy0, dy1, dy2)
    }
  }

  // Do lots of reps to get a confidence ribbon
  val tauSolution = (1 to 1000).map { _ =>
    MyTauLeap(p).solve(y0, startTime, endTime, stepSize)
        .map { case (t, arr) => SolutionPoint(t, arr(0), arr(1), arr(2)) }
  }

  def solveODE(p: Parameters): Seq[SolutionPoint] = {
    case class MyODE(p: Parameters) extends ODE {
      override def getDimension: Int = 3

      override def computeDerivatives(t: Double, y: Array[Double], yDot: Array[Double]): Unit = {
        yDot(0) = -p.beta * y(0) * y(1)
        yDot(1) = p.beta * y(0) * y(1) - p.gamma * y(1)
        yDot(2) = p.gamma * y(1)
      }
    }

    MyODE(p)
        .solve(y0, startTime, endTime, stepSize)
        .map { case (t, arr) => SolutionPoint(t, arr(0), arr(1), arr(2)) }
  }

  val odeSolution = solveODE(p)

  output.saveOutput(odeSolution, tauSolution)
  output.compareWithODE(startTime, endTime, stepSize)
}

object output {
  val outDir: Path = Paths.get("out")
  val odeFileName = "ode-result.json"
  val tauLeapFileName = "tau-leap-result.json"

  def saveOutput(odeSolution: Seq[SolutionPoint], tauSolution: Seq[Seq[SolutionPoint]]): Unit = {
    FileUtils.writeStringToFile(
      outDir.resolve(odeFileName).toFile,
      Json.prettyPrint(Json.toJson(odeSolution))
    )

    FileUtils.writeStringToFile(
      outDir.resolve(tauLeapFileName).toFile,
      Json.prettyPrint(Json.toJson(tauSolution))
    )
  }

  def compareWithODE(startTime: Double, endTime: Double, timeStep: Double): Unit = {
    val myFormatObj = DateTimeFormatter.ofPattern("yyyyMMdd")
    val today = LocalDate.now().format(myFormatObj)

    val description = s"${today}_timeStep=$timeStep"
    val lineWidth = 1.5

    val script =
      s"""
         |lapply(c("ggplot2", "reshape2", "jsonlite","plyr"), require, character.only=T)
         |pdf("$description.pdf", width=10, height=8, title = "$description")
         |
         |ode = (fromJSON("$odeFileName"))
         |tauLeap = (fromJSON("$tauLeapFileName"))
         |
         |tauLeapEx1 = tauLeap[22]
         |ex1 = melt(tauLeapEx1,id=("time"))
         |names(ex1) = c("time","variable","example")
         |
         |allTauSims = do.call(rbind,tauLeap)
         |
         |ode$$time = round(ode$$time,2)
         |odeData = melt(ode,id=c("time"))
         |names(odeData) = c("time","variable","ODE")
         |
         |tauData = melt(allTauSims,id=c("time"))
         |tauData = aggregate(value~time+variable,tauData,quantile,probs=c(0.025,0.25,0.5,0.75,0.975))
         |
         |data = merge(merge(odeData,tauData,by=c("time","variable")),ex1,by=c("time","variable"))
         |
         |ggplot(data,aes(x=time)) +
         |facet_grid(variable~.) +
         |geom_ribbon(aes(ymin = value[,1], ymax = value[,5],fill="Tau-Leap 95% CI")) +
         |geom_line(aes(y = example, color = "Tau Leap Solution"),size = $lineWidth) +
         |geom_line(aes(y = ODE, color = "ODE Solution"),size = $lineWidth) +
         |theme(text = element_text(size = 20)) +
         |theme(axis.text.x = element_text(size = 14)) +
         |scale_y_continuous(name = "") +
         |scale_x_continuous(breaks = seq($startTime, $endTime,5), labels = seq($startTime, $endTime,5)) +
         |scale_fill_manual(name = "", values = c("Tau-Leap 95% CI" = "grey")) +
         |scale_color_manual(name = "", values = c("ODE Solution" = "blue","Tau Leap Solution" = "red")) +
         |ggtitle("Comparison of solutions by ODE solver and Tau-Leap Solver")
       """.stripMargin

    RScript(script, outDir.resolve("compareTauLeapScript.R"))
  }
}
