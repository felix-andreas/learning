suppressPackageStartupMessages({
    library(tidyverse)
})

obs <- 104

x <- starwars[c(   "name" , "height")]

ggsave(
    "foo.png",
    plot = ggplot(mpg, aes(displ, hwy, colour = class)) +
        geom_point(),
)
