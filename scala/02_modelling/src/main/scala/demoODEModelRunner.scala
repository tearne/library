//object demoODEModelRunner extends App {
//
//  object MyODEModel extends ODEModel[Parameters, ODEState] {
//    override val timeSpan: Range = 0 to 10
//    override val ode: ODE[Parameters] = new ODE[Parameters] {
//      val param = 2.0
//      val y0: State = ODEState(10, 20)
//
//      override def computeDerivatives(t: Double, y: Array[Double], yDot: Array[Double]): Unit = {
//        yDot(0) = param * y(1)
//        yDot(1) = -param * y(0)
//      }
//    }
//
//    override def stateFromArray(a: Array[Double]): ODEState = ODEState(a(0), a(1))
//  }
//
//  case class Parameters(a: Double)
//
//  case class ODEState(s: Double, i: Double) extends State {
//    override def toArray: Array[Double] = Array(s, i)
//  }
//
//  MyODEModel.solveAndPrint()
//}
