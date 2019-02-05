object demoODEModelRunner extends App {

  object MyODEModel extends ODEModel[Parameters, ODEState] {
    override val timeSpan: Range = 0 to 10
    override val ode: ODE[Parameters] = new ODE[Parameters] {
      override val params: Parameters = Parameters(2.0)
      override val y0: State = ODEState(10, 20)

      override def computeDerivatives(t: Double, y: Array[Double], yDot: Array[Double]): Unit = {
        import params._
        yDot(0) = a * y(1)
        yDot(1) = -a * y(0)
      }
    }

    override def stateFromArray(a: Array[Double]): ODEState = ODEState(a(0), a(1))
  }

  case class Parameters(a: Double)

  case class ODEState(s: Double, i: Double) extends State {
    override def toArray: Array[Double] = Array(s, i)
  }

  MyODEModel.solveAndPrint()
}
