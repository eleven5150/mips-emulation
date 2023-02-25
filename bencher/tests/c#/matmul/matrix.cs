namespace matmul;

internal abstract class Matrix
{
    public static void Print(int[,] matrix, int matrixDimension)
    {
        for (var i = 0; i < matrixDimension; i++)
        {
            for (var j = 0; j < matrixDimension; j++)
            {
                Console.WriteLine($"{matrix[i, j]}\t");
            }
            Console.WriteLine("\n");
        }
        Console.WriteLine("\n");
    }

    public static int[,] Multiply(int matrixDimension, ref int[,] matrixA, ref int[,] matrixB)
    {
        var matrixResult = new int[matrixDimension, matrixDimension];
        var tMatrixB = new int[matrixDimension, matrixDimension];
        
        for (var i = 0; i < matrixDimension; ++i)
        for (var j = 0; j < matrixDimension; ++j)
            tMatrixB[j, i] = matrixB[i, j];

        for (var i = 0; i < matrixDimension; ++i)
        for (var j = 0; j < matrixDimension; ++j)
        {
            var s = 0;
            for (var k = 0; k < matrixDimension; ++k)
                s += matrixA[i, k] * tMatrixB[j, k];
            matrixResult[i, j] = s;
        }

        return matrixResult;
    }
}