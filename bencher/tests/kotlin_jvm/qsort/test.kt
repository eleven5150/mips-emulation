import kotlin.system.exitProcess
import java.io.File

object Constants {
    var DATA_TO_SORT_SIZE: Int = 0;
}

object QuickSort {

    fun Print(data: ULongArray, size: Int) {
        for (i in 0..size - 1) {
            print("${data[i]}\n");
        }
    }

    fun Partition(data: ULongArray, low: Int, high: Int): Int {
        val pivot: ULong = data[high];
        var i: Int = low - 1;

        for (j in low..high - 1) {
            if (data[j] <= pivot) {
                i++;
                data[i] = data[j].also { data[j] = data[i] };
            }
        }

        data[i + 1] = data[high].also { data[high] = data[i + 1] };

        return i + 1;
    }

    fun Sort(data: ULongArray, low: Int, high: Int) {
        if (low < high) {
            val pi: Int = QuickSort.Partition(data, low, high);

            Sort(data, low, pi - 1);
            Sort(data, pi + 1, high);
        }
    }
}


fun main(args: Array<String>){
    if (args.size < 1) {
        println("Error! File with data to sort must be specified\n");
        exitProcess(1);
    }

    val dataToSortFile: String = args[0];

    val data: String = File(dataToSortFile).inputStream().readBytes().toString(Charsets.UTF_8);
    val lines: MutableList<String> = data.split("\n").toMutableList();

    lines.removeLast();
    Constants.DATA_TO_SORT_SIZE = lines[0].substring(2).toInt();
    lines.removeFirst();

    var dataToSort: ULongArray = ULongArray(Constants.DATA_TO_SORT_SIZE);

    for ((it, line) in lines.withIndex()) {
        dataToSort[it] = line.toULong();
    }

    QuickSort.Sort(dataToSort, 0, Constants.DATA_TO_SORT_SIZE - 1);

    QuickSort.Print(dataToSort, Constants.DATA_TO_SORT_SIZE);
}