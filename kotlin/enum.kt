enum class Direction {
    NORTH, SOUTH, WEST, EAST
}

fun main() {
    val direction = Direction.NORTH
    println(direction)
    when (direction) {
        Direction.NORTH -> println("Norden!")
        else -> {
            println("else")
        }
    }
}
