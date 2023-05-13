import kotlin.system.exitProcess
import java.io.File

object Constants {
    const val MATRIX_DIMENSION: Int = 5
}

object Matrix {

    fun print(matrix: Array<UIntArray>, matrix_deminsion: Int) {
        for (i in 0..matrix_deminsion - 1) {
            for (j in 0..matrix_deminsion - 1) {
                print("${matrix[i][j]}\t");
            }
            print("\n");
        }
        print("\n");
    }

    fun matrix_multiply(matrix_deminsion: Int, matrix_a: Array<UIntArray>, matrix_b: Array<UIntArray>): Array<UIntArray> {

        val matrix_result: Array<UIntArray> = Array(Constants.MATRIX_DIMENSION) { UIntArray(Constants.MATRIX_DIMENSION) };
        val t_matrix_b: Array<UIntArray> = Array(Constants.MATRIX_DIMENSION) { UIntArray(Constants.MATRIX_DIMENSION) };
        for (i in 0..matrix_deminsion - 1) {
            for (j in 0..matrix_deminsion - 1) {
                t_matrix_b[i][j] = matrix_b[j][i];
            }
        }

        for (i in 0..matrix_deminsion - 1) {
            for (j in 0..matrix_deminsion - 1) {
               var s: UInt = 0u;
                for (k in 0..matrix_deminsion - 1) {
                    s += matrix_a[i][k] * t_matrix_b[j][k];
                }
                matrix_result[i][j] = s;
            }
        }
        return matrix_result;
    }
}


fun create_matrix(file_path: String): Array<UIntArray> {
    val matrix: Array<UIntArray> = Array(Constants.MATRIX_DIMENSION) { UIntArray(Constants.MATRIX_DIMENSION) };

    val data: String = File(file_path).inputStream().readBytes().toString(Charsets.UTF_8)
    val lines: MutableList<String> = data.split("\n").toMutableList();

    lines.removeLast();

    var i: Int = 0;
    var j: Int = 0;
    for (line in lines) {
        val line_record: List<String> = line.split(",");
        for (record in line_record) {
            matrix[i][j] = record.toUInt();
            j++;
        }
        i++;
        j = 0;
    }

//    Matrix.print(matrix, Constants.MATRIX_DIMENSION);
    return matrix;
}

fun main(args: Array<String>) {
    if (args.size < 2) {
        println("Error! Both paths to matrix data required\\n")
        exitProcess(1)
    }
    val matrix_a_path: String = args[0];
    val matrix_b_path: String = args[1];

    val matrix_a: Array<UIntArray> = create_matrix(matrix_a_path);
    val matrix_b: Array<UIntArray> = create_matrix(matrix_b_path);

    val matrix_result: Array<UIntArray> = Matrix.matrix_multiply(Constants.MATRIX_DIMENSION, matrix_a, matrix_b);

//    Matrix.print(matrix_result, Constants.MATRIX_DIMENSION);
}