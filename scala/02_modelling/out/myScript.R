setwd("/home/olivertearne/CODE/library/scala/02_modelling/out")

lapply(c("ggplot2", "reshape2", "deSolve", "jsonlite","plyr"), require, character.only=T)
pdf("compareODESolver.pdf", width=8, height=4, title = "compare ODE Solutions")

output = (fromJSON("commons-math-result.json"))
scalaData = melt(output,id=c("time"))
scalaData$solver = "Scala"

params = c(
  alpha = 2.2
)

Y0 = c(
  x = 10.0,
  y = 20.0
)
dY <-function(t, state, parameters) {
  with(as.list(c(state, parameters)),{
    dx <- alpha * y
    dy <- -alpha * x

list(c(dx, dy))
  })
}

times = seq(0.0, 5.0, by = 0.2)
out <- ode(y = Y0, times = times, func = dY, parms = params)
rData = melt(as.data.frame(out), id="time")
rData$solver = "R"

data = rbind(scalaData,rData)

ggplot(data, aes(x=time, y=value,colour=variable)) +
  geom_line(size=1.5) +
  facet_grid(solver~.) +
  theme(text = element_text(size = 20)) +
  scale_x_continuous(breaks = c(0.0 : 5.0), labels = c(0.0 : 5.0)) +
  ggtitle("Comparison of solutions by R and Scala")

#Some ugly hacking going on here with the time variable to make sure dcast works
data$time = paste(data$time)
wide_data = dcast(data,time+variable~solver,value.var="value")
wide_data$error = wide_data$R - wide_data$Scala
wide_data$time = as.numeric(wide_data$time)

ggplot(wide_data,aes(x=time,y=error,colour=variable)) +
  geom_line(size=1.5) +
  theme(text = element_text(size = 20)) +
  scale_x_continuous(breaks = c(0.0 : 5.0), labels = c(0.0 : 5.0)) +
  ggtitle("Error between R and Scala solution")
      
