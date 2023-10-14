namespace qsort;

internal static class Constants
{
    public static int DataToSortSize = 0;
}

internal static class Program 
{
    private static void Main(string[] args)
    {
        if (args.Length < 1)
        {
            Console.Error.WriteLine("Error! File with data to sort must be specified\n");
            Environment.Exit(1);
        }

        var dataToSortFile = args[0];

        var lines = File.ReadAllLines(dataToSortFile);

        Constants.DataToSortSize = int.Parse(lines[0][2..]);

        var dataToSort = new uint[Constants.DataToSortSize];
        var i = 0;
        foreach (var line in lines[1..])
        {
            dataToSort[i] = uint.Parse(line);
            i++;
        }

        QuickSort.Sort(dataToSort, 0,  Constants.DataToSortSize - 1);

        // QuickSort.DataPrint(dataToSort, Constants.DataToSortSize);

    }
}