sealed interface Foo

data class Adel(
        val foo: String,
) : Foo

data class Tung(
        val bar: String,
) : Foo

fun main() {
    // val foo: Foo = Adel("Foo")
    // when (foo) {
    //     is Adel -> println("Hello I am Adel!")
    //     is Tung -> println("Hey I am Tung")
    // }

    val array = arrayOf(1, 2, 3, 4)
    println(array.reduce { acc, item -> acc + item })
}
