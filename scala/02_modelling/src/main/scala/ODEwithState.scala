import org.apache.commons.math3.ode.FirstOrderDifferentialEquations
import org.apache.commons.math3.ode.nonstiff.DormandPrince853Integrator
import org.apache.commons.math3.ode.sampling.{FixedStepHandler, StepNormalizer}

import scala.collection.mutable

object demoODEModelRunner extends App {

  case class MyODEwithState(alpha: Double) extends ODEwithState {

    // TODO what if I'd like to use an array instead of defining my own state?
    case class State(x: Double, y: Double)

    val initialValue = State(10, 20)

    def toArray(s: State) = Array(s.x, s.y)

    def toState(a: Array[Double]) = State(a(0), a(1))

    override def computeDerivatives(t: Double, y: Array[Double], yDot: Array[Double]): Unit = {
      yDot(0) = alpha * y(1)
      yDot(1) = -alpha * y(0)
    }
  }

  val myODE = MyODEwithState(2.0)

  ODESolverWithState.solveAndPrint(myODE, 0, 10)
}

trait ODEwithState extends FirstOrderDifferentialEquations {
  type State

  def toArray(s: State): Array[Double]

  def toState(a: Array[Double]): State

  val initialValue: State

  override def getDimension: Int = toArray(initialValue).length
}

object ODESolverWithState {

  def solve[C <: ODEwithState](ode: C, initialTime: Double, targetTime: Double): Vector[(Double, ode.State)] = {
    val relTol = 1e-11
    val absTol = 1e-11
    val minStep = 1e-8
    val maxStep = 100.0 // TODO Should this have an effect.  E.g. setting as 0.01 doesn't change the steps given - step normaliser is doing this....why do we need this if we are using step normaliser?

    val steps = mutable.Buffer[(Double, ode.State)]()

    val stepHandler: FixedStepHandler = new FixedStepHandler {
      def init(t0: Double, y0: Array[Double], t: Double): Unit = {}

      def handleStep(t: Double, y: Array[Double], yDot: Array[Double], isLast: Boolean): Unit = {
        val rounded = y.map { value => if (math.abs(value) < 1e-2) 0 else value }
        steps.append((t, ode.toState(rounded)))
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

  def solveAndPrint[C <: ODEwithState](ode: C, initialTime: Double, targetTime: Double): Unit = {
    solve(ode, initialTime, targetTime).foreach(println)
  }
}

object ODESolver {
  def solve[C <: FirstOrderDifferentialEquations](ode: C, initialValue: Array[Double], startTime: Double, endTime: Double): Vector[(Double, Array[Double])] = {
    assume(initialValue.length == ode.getDimension, "Dimension of initial condition does not match dimension of ODE")

    val relTol = 1e-11
    val absTol = 1e-11
    val minStep = 1e-8
    val maxStep = 100.0 // TODO Should this have an effect.  E.g. setting as 0.01 doesn't change the steps given - step normaliser is doing this....why do we need this if we are using step normaliser?

    val steps = mutable.Buffer[(Double, Array[Double])]()

    val stepHandler: FixedStepHandler = new FixedStepHandler {
      def init(t0: Double, y0: Array[Double], t: Double): Unit = {}

      def handleStep(t: Double, y: Array[Double], yDot: Array[Double], isLast: Boolean): Unit = {
        val rounded = y.map { value => if (math.abs(value) < 1e-2) 0 else value }
        steps.append((t, rounded))
      }
    }

    val stepNormalizer = new StepNormalizer(1.0, stepHandler)
    val dp853 = new DormandPrince853Integrator(minStep, maxStep, absTol, relTol)
    val out = Array.fill(ode.getDimension)(0.0)

    dp853.addStepHandler(stepNormalizer)

    // Nothing is returned in this call as the stepHander is mutated during integration
    dp853.integrate(ode, startTime, initialValue, endTime, out)

    steps.toVector
  }

  def solveAndPrint(ode: FirstOrderDifferentialEquations, y0: Array[Double], t0: Double, t1: Double): Unit = {
    solve(ode, y0, t0, t1)
        .foreach { case (t, state) => println(s"t = $t, [${state.deep.mkString(", ")}]") }
  }
}

object demoArrayODESolver extends App {

  case class Parameters(alpha: Double)

  val y0 = Array(10.0, 20.0)

  case class MyODE(p: Parameters) extends FirstOrderDifferentialEquations {
    override def computeDerivatives(t: Double, y: Array[Double], yDot: Array[Double]): Unit = {
      yDot(0) = p.alpha * y(1)
      yDot(1) = -p.alpha * y(0)
    }

    override def getDimension: Int = 2
  }

  val p = Parameters(2.0)

  val ode = MyODE(p)

  ODESolver.solveAndPrint(ode, y0, 0, 10)
}