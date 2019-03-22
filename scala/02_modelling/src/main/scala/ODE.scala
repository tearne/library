import org.apache.commons.math3.ode.FirstOrderDifferentialEquations
import org.apache.commons.math3.ode.nonstiff.DormandPrince853Integrator
import org.apache.commons.math3.ode.sampling.{FixedStepHandler, StepNormalizer}

import scala.collection.mutable

trait ODE extends FirstOrderDifferentialEquations {

  def solve(initialValue: Array[Double], startTime: Double, endTime: Double): Vector[(Double, Array[Double])] = {
    assume(initialValue.length == this.getDimension, "Dimension of initial condition does not match dimension of ODE")

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
    val out = Array.fill(getDimension)(0.0)

    dp853.addStepHandler(stepNormalizer)

    // Nothing is returned in this call as the stepHander is mutated during integration
    dp853.integrate(this, startTime, initialValue, endTime, out)

    steps.toVector
  }

  def solveAndPrint(y0: Array[Double], t0: Double, t1: Double): Unit = {
    solve(y0, t0, t1)
        .foreach { case (t, state) => println(s"t = $t, [${state.deep.mkString(", ")}]") }
  }
}

object demoODESolver extends App {

  case class Parameters(alpha: Double)

  case class MyODE(p: Parameters) extends ODE {
    override def getDimension: Int = 2

    override def computeDerivatives(t: Double, y: Array[Double], yDot: Array[Double]): Unit = {
      yDot(0) = p.alpha * y(1)
      yDot(1) = -p.alpha * y(0)
    }
  }

  val y0 = Array(10, 20.0)
  val p = Parameters(2.0)

  val ode = MyODE(p)
  ode.solveAndPrint(y0, 0, 10)
}