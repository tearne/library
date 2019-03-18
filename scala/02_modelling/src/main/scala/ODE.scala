import org.apache.commons.math3.ode.FirstOrderDifferentialEquations
import org.apache.commons.math3.ode.nonstiff.DormandPrince853Integrator
import org.apache.commons.math3.ode.sampling.{FixedStepHandler, StepNormalizer}

import scala.collection.mutable

object demoODEModelRunner extends App {

  case class MyODE(alpha: Double) extends ODE {
    case class State(x: Double, y: Double)

    val initialValue = State(10,20)

    def toArray(s: State) = Array(s.x, s.y)
    def toState(arr: Array[Double]) = State(arr(0), arr(1))

    override def getDimension: Int = 2

    override def computeDerivatives(t: Double, y: Array[Double], yDot: Array[Double]): Unit = {
      yDot(0) = alpha * y(1)
      yDot(1) = -alpha * y(0)
    }
  }

  val myODE = MyODE(2.0)
  ODESolver.solveAndPrint(myODE)(myODE.initialValue, 0, 10)
}


trait ODE extends FirstOrderDifferentialEquations {
  type State

  val initialValue: State

  def toArray(s: State): Array[Double]
  def toState(arr: Array[Double]): State
}

object ODESolver {

  def solve[C <: ODE](ode: C)(initialValue: ode.State, initialTime: Double, targetTime: Double): Vector[(Double, ode.State)] = {
    val relTol = 1e-11
    val absTol = 1e-11
    val minStep = 1e-8
    val maxStep =100.0  // Should this have an effect.  E.g. setting as 0.01 doesn't change the steps given

    val steps = mutable.Buffer[(Double, ode.State)]()

    val stepHandler: FixedStepHandler = new FixedStepHandler {
      def init(t0: Double, y0: Array[Double], t: Double): Unit = {}

      def handleStep(t: Double, y: Array[Double], yDot: Array[Double], isLast: Boolean): Unit = {
        steps.append((t, ode.toState(y)))
      }
    }

    val stepNormalizer = new StepNormalizer(1.0, stepHandler)
    val dp853 = new DormandPrince853Integrator(minStep, maxStep, absTol, relTol)
    val out = Array.fill(ode.getDimension)(0.0)

    dp853.addStepHandler(stepNormalizer)

    // Nothing is returned in this call as the stepHander is mutated during integration
    dp853.integrate(ode, initialTime, ode.toArray(ode.initialValue), targetTime, out)

    steps.toVector
  }

  def solveAndPrint[E <: ODE](ode: E)(initialValue: ode.State, initialTime: Double, targetTime: Double): Unit = {
    solve(ode)(initialValue, initialTime, targetTime).foreach(println)
  }
}