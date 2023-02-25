namespace matmul;

internal static class Constants
{
    public const int MatrixDimension = 1000;
}

internal static class Program
{
    private static int[,] create_matrix(string filePath)
    {
        var matrix = new int[Constants.MatrixDimension, Constants.MatrixDimension];
        var lines = File.ReadAllLines(filePath);

        var i = 0;
        var j = 0;
        foreach (var line in lines)
        {
            var lineRecords = line.Split(",");
            foreach (var record in lineRecords)
            {
                matrix[i, j] = int.Parse(record);
                j++;
            }

            i++;
            j = 0;
        }
        
        Matrix.Print(matrix, Constants.MatrixDimension);

        return matrix;
    }

    private static void Main(string[] args)
    {
        if (args.Length < 2)
        {
            Console.Error.WriteLine("Error! Both paths to matrix data required\n");
            Environment.Exit(1);
        }

        var matrixAFile = args[1];
        var matrixBFile = args[2];

        var matrixA = create_matrix(matrixAFile);
        var matrixB = create_matrix(matrixBFile);

        var matrixResult = Matrix.Multiply(Constants.MatrixDimension, ref matrixA, ref matrixB);
        
        Matrix.Print(matrixResult, Constants.MatrixDimension);
    }
}