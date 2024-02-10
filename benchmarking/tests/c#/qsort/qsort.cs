namespace qsort;

internal abstract class QuickSort
{
    private static long Partition(ulong[] data, long low, long high)
    {
        var pivot = data[high];
        var i = low - 1;

        for (var j = low; j <= high - 1; j++)
        {
            if (data[j] <= pivot)
            {
                i++;
                (data[i], data[j]) = (data[j], data[i]);
            }
        }

        (data[i + 1], data[high]) = (data[high], data[i + 1]);

        return i + 1;
    }

    public static void Sort(ulong[] data, long low, long high)
    {
        if (low < high) {
            var pi = Partition(data, low, high);

            Sort(data, low, pi - 1);
            Sort(data, pi + 1, high);
        }
    }

    public static void DataPrint(ulong[] data, uint size)
    {
        for (var i = 0; i < size; i++)
        {
            Console.Write($"{data[i]} ");
        }
    }
}