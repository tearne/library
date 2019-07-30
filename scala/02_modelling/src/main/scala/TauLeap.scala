import java.nio.file.{Path, Paths}

import breeze.stats.distributions.Poisson
import demoODESolver.SolutionPoint
import org.apache.commons.io.FileUtils
import play.api.libs.json.Json
import sampler.r.script.RScript

import scala.annotation.tailrec

trait TauLeap {

  def computeNextStep(y: Array[Double], timeStep: Double): Array[Double]

  def solve(y0: Array[Double], t0: Double, t1: Double, timeStep: Double): IndexedSeq[(Double, Array[Double])] = {

    @tailrec def go(toProcess: Seq[BigDecimal], y: Array[Double], acc: IndexedSeq[(Double, Array[Double])]): IndexedSeq[(Double, Array[Double])] = {
      if (toProcess.isEmpty) {
        acc
      } else {
        val thisState = computeNextStep(y, timeStep)
        val solutionPoint = (toProcess.head.toDouble, thisState)

        go(toProcess.tail, thisState, acc.:+(solutionPoint))
      }
    }

    val times: Seq[BigDecimal] = BigDecimal(t0) to t1 by timeStep
    go(times, y0, IndexedSeq((0.0, y0)))
  }
}

object DemoTauLeapSolver extends App {

  case class Parameters(beta: Double, gamma: Double)

  val p = Parameters(0.001, 0.1)

  val startTime: Double = 0
  val endTime: Double = 60
  val stepSize: Double = 1

  val y0 = Array(500.0, 1.0, 0.0)

  case class myTauLeap(p: Parameters) extends TauLeap {
    def computeNextStep(y: Array[Double], timeStep: Double): Array[Double] = {
      val StoI = Math.min(Poisson(p.beta * y(0) * y(1) * timeStep).sample, y(0))
      val ItoC = Math.min(Poisson((p.gamma * y(1)) * timeStep).sample, y(1))

      val dy0 = y(0) - StoI
      val dy1 = y(1) + StoI - ItoC
      val dy2 = y(2) + ItoC

      Array(dy0, dy1, dy2)
    }
  }

  // TODO for plot: do a bunch of tau-leap sim's, draw ribbon to show 95% CI & draw a sim or 2 to show 'wigglyness'

  val tauSolution = (1 to 1000).map { _ =>
    myTauLeap(p).solve(y0, startTime, endTime, stepSize)
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

    val ode = MyODE(p)

    ode
        .solve(y0, startTime, endTime, stepSize)
        .map { case (t, arr) => SolutionPoint(t, arr(0), arr(1), arr(2)) }
  }

  val odeSolution = solveODE(p)

  output.saveOutput(odeSolution, tauSolution)
  //  output.compareWithODE(startTime, endTime)
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

  def compareWithODE(startTime: Double, endTime: Double): Unit = {
    val lineWidth = 1.5
    val script =
      s"""
         |lapply(c("ggplot2", "reshape2", "jsonlite","plyr"), require, character.only=T)
         |pdf("compareTauLeap.pdf", width=12, height=6, title = "compare Tau Leap solver with ODE solver")
         |
         |ode = (fromJSON("$odeFileName"))
         |tauLeap = (fromJSON("$tauLeapFileName"))
         |
         |tauLeapEx1 = tauLeap[22]
         |
         |allTauSims = do.call(rbind,tauLeap)
         |
         |odeData = melt(ode,id=c("time"))
         |names(odeData) = c("time","variable","ODE")
         |
         |tauData = melt(allTauSims,id=c("time"))
         |lower = aggregate(value~time+variable,tauData,quantile,probs=c(0.25))
         |names(lower) = c("time","variable","lower")
         |upper = aggregate(value~time+variable,tauData,quantile,probs=c(0.75))
         |names(upper) = c("time","variable","upper")
         |
         |lower$$upper = upper$$upper
         |tauData = lower
         |
         |data = merge(odeData,tauData,by=c("time","variable"))
         |ggplot(data,aes(x=time)) +
         |geom_ribbon(aes(ymin=lower, ymax=upper)) +
         |facet_grid(variable~.)

       """.stripMargin

//    ggplot(data, aes(x=time, y=value,colour=solver)) +
//        geom_line(size=$lineWidth) +
//        facet_grid(variable~.) +
//        theme(text = element_text(size = 20)) +
//        scale_x_continuous(breaks = c($startTime : $endTime), labels = c($startTime : $endTime)) +
//        theme(axis.text.x = element_text(size = 10, angle = 65, hjust = 1, vjust = 1)) +
//        ggtitle("Comparison of solutions by ODE solver and Tau-Leap Solver")

    RScript(script, outDir.resolve("compareTauLeapScript.R"))
  }
}
