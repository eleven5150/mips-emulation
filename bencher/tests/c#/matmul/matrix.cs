namespace matmul;

internal abstract class Matrix
{
    public static void Print(uint[,] matrix, uint matrixDimension)
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

    public static uint[,] Multiply(uint matrixDimension, ref uint[,] matrixA, ref uint[,] matrixB)
    {
        var matrixResult = new uint[matrixDimension, matrixDimension];
        var tMatrixB = new uint[matrixDimension, matrixDimension];
        
        for (var i = 0; i < matrixDimension; ++i)
        for (var j = 0; j < matrixDimension; ++j)
            tMatrixB[j, i] = matrixB[i, j];

        for (var i = 0; i < matrixDimension; ++i)
        for (var j = 0; j < matrixDimension; ++j)
        {
            var s = 0u;
            for (var k = 0; k < matrixDimension; ++k)
                s += matrixA[i, k] * tMatrixB[j, k];
            matrixResult[i, j] = s;
        }

        return matrixResult;
    }
}