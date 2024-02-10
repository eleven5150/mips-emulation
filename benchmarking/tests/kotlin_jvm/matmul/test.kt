import kotlin.system.exitProcess
import java.io.File

object Constants {
    var MATRIX_DIMENSION: Int = 0;
}

object Matrix {

    fun print(matrix: Array<ULongArray>, matrixDimension: Int) {
        for (i in 0..matrixDimension - 1) {
            for (j in 0..matrixDimension - 1) {
                print("${matrix[i][j]}\t");
            }
            print("\n");
        }
        print("\n");
    }

    fun matrixMultiply(matrixDimension: Int, matrixA: Array<ULongArray>, matrixB: Array<ULongArray>): Array<ULongArray> {
        val matrixResult: Array<ULongArray> = Array(Constants.MATRIX_DIMENSION) { ULongArray(Constants.MATRIX_DIMENSION) };
        val tMatrixB: Array<ULongArray> = Array(Constants.MATRIX_DIMENSION) { ULongArray(Constants.MATRIX_DIMENSION) };
        for (i in 0..matrixDimension - 1) {
            for (j in 0..matrixDimension - 1) {
                tMatrixB[i][j] = matrixB[j][i];
            }
        }

        for (i in 0..matrixDimension - 1) {
            for (j in 0..matrixDimension - 1) {
               var s: ULong = 0u;
                for (k in 0..matrixDimension - 1) {
                    s += matrixA[i][k] * tMatrixB[j][k];
                }
                matrixResult[i][j] = s;
            }
        }
        return matrixResult;
    }
}


fun createMatrix(filePath: String): Array<ULongArray> {
    val data: String = File(filePath).inputStream().readBytes().toString(Charsets.UTF_8)
    val lines: MutableList<String> = data.split("\n").toMutableList();

    lines.removeLast();

    val matrixDimensionString = lines[0];
    Constants.MATRIX_DIMENSION = matrixDimensionString.substring(2).toInt()

    lines.removeFirst()

    val matrix: Array<ULongArray> = Array(Constants.MATRIX_DIMENSION) { ULongArray(Constants.MATRIX_DIMENSION) };

    for ((i, line) in lines.withIndex()) {
        val lineRecord: List<String> = line.split(",");
        for ((j, record) in lineRecord.withIndex()) {
            matrix[i][j] = record.toULong();
        }
    }

//    Matrix.print(matrix, Constants.MATRIX_DIMENSION);
    return matrix;
}

fun main(args: Array<String>){
    if (args.size < 2) {
        println("Error! Both paths to matrix data required\n")
        exitProcess(1)
    }
    val matrixAPath: String = args[0];
    val matrixBPath: String = args[1];

    val matrixA: Array<ULongArray> = createMatrix(matrixAPath);
    val matrixB: Array<ULongArray> = createMatrix(matrixBPath);

    val matrixResult: Array<ULongArray> = Matrix.matrixMultiply(Constants.MATRIX_DIMENSION, matrixA, matrixB);

//    Matrix.print(matrixResult, Constants.MATRIX_DIMENSION);
}