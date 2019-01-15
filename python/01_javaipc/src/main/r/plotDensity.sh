#!/usr/bin/Rscript

lapply(c("ggplot2", "reshape2"), require, character.only=T)

data = read.csv("out.csv")

pdf("plot.pdf", width=6, height=4)

ggplot(data, aes(x = Time, colour = Method)) +
    geom_density(adjust = 2) +
    ggtitle("Round trip time by message size (bytes)") +
    scale_x_continuous(name = "milliseconds") +
    facet_grid(Bytes ~ ., scales ="free_y") +
    theme(axis.title.y=element_blank(),
        axis.text.y=element_blank(),
        axis.ticks.y=element_blank()
    )

dev.off()