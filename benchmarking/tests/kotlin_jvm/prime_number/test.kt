import kotlin.system.exitProcess
import java.io.File

fun main(args: Array<String>) {
    if (args.size < 1) {
        println("Error! File with prime number count must be specified\n")
        exitProcess(1)
    }

    var primeNumberCountPath: String = args[0];

    val data: String = File(primeNumberCountPath).inputStream().readBytes().toString(Charsets.UTF_8);
    val lines: MutableList<String> = data.split("\n").toMutableList();

    lines.removeLast();

    var primeNumberCount: ULong = lines[0].toULong();

    var number: ULong = 0uL;
    while (primeNumberCount > 0uL) {
        number++;
        var j: ULong = 0uL;

        var i: ULong = 1uL;
        while (i <= number) {
            if (number % i.toULong() == 0uL) {
                j++;
            }
            i++;
        }

        if (j == 2uL) {
            primeNumberCount--;
        }
    }

    println("The latest prime number: ${number}\n");
}
