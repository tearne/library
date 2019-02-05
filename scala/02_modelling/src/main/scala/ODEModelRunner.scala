import org.apache.commons.math3.ode.FirstOrderDifferentialEquations
import org.apache.commons.math3.ode.nonstiff.DormandPrince853Integrator
import org.apache.commons.math3.ode.sampling.{FixedStepHandler, StepNormalizer}

import scala.collection.mutable

trait ODE[P] extends FirstOrderDifferentialEquations {
  val params: P
  val y0: State

  override def getDimension: Int = y0.toArray.length
}

trait State {
  def toArray: Array[Double]
}

trait ODEModel[P, ODEState] {
  val timeSpan: Range
  val ode: ODE[P]

  def stateFromArray(a: Array[Double]): ODEState

  def solve(): Map[Int, ODEState] = {
    val relTol = 1e-11
    val absTol = 1e-11
    val minStep = 1e-8
    val maxStep = 100.0
    val timeZero = timeSpan.min
    val numDays = timeSpan.length + 1

    val steps = mutable.Buffer[ODEState]()

    val stepHandler: FixedStepHandler = new FixedStepHandler {
      override def init(t0: Double, y0: Array[Double], t: Double): Unit = {}

      override def handleStep(t: Double, y: Array[Double], yDot: Array[Double], isLast: Boolean): Unit = {
        steps.append(stateFromArray(y))
      }
    }

    val stepNormalizer = new StepNormalizer(1.0, stepHandler)
    val dp853 = new DormandPrince853Integrator(minStep, maxStep, absTol, relTol)
    val out = Array.fill(ode.getDimension)(0.0)

    dp853.addStepHandler(stepNormalizer)
    dp853.integrate(ode, timeZero, ode.y0.toArray, numDays, out)

    val integrated: List[ODEState] = steps.toList

    timeSpan.foldLeft(Map[Int, ODEState]()) { case (map, t) => map.updated(t, integrated(t)) }
  }

  def solveAndPrint(): Unit = {
    val sortedTimeSeries: Seq[(Int, ODEState)] = solve().toSeq.sortBy(_._1)
    for (state <- sortedTimeSeries) {
      println(s"t = ${state._1}, ${state._2}")
    }
  }
}