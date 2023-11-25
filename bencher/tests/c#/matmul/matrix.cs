namespace matmul;

internal abstract class Matrix
{
    public static void Print(ulong[,] matrix, uint matrixDimension)
    {
        for (var i = 0; i < matrixDimension; i++)
        {
            for (var j = 0; j < matrixDimension; j++)
            {
                Console.Write($"{matrix[i, j]}\t");
            }
            Console.WriteLine("\n");
        }
        Console.WriteLine("\n");
    }

    public static ulong[,] Multiply(uint matrixDimension, ref ulong[,] matrixA, ref ulong[,] matrixB)
    {
        var matrixResult = new ulong[matrixDimension, matrixDimension];
        var tMatrixB = new ulong[matrixDimension, matrixDimension];
        
        for (var i = 0; i < matrixDimension; ++i)
        for (var j = 0; j < matrixDimension; ++j)
            tMatrixB[j, i] = matrixB[i, j];

        for (var i = 0; i < matrixDimension; ++i)
        for (var j = 0; j < matrixDimension; ++j)
        {
            var s = 0ul;
            for (var k = 0; k < matrixDimension; ++k)
                s += matrixA[i, k] * tMatrixB[j, k];
            matrixResult[i, j] = s;
        }

        return matrixResult;
    }
}