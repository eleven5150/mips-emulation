using System;
using System.IO;

class Prog 
{
    static public void quick_sort(int[] array, int leftIndex, int rightIndex)
    {
        var i = leftIndex;
        var j = rightIndex;
        var pivot = array[leftIndex];
        while (i <= j)
        {
            while (array[i] < pivot)
            {
                i++;
            }
            
            while (array[j] > pivot)
            {
                j--;
            }
            if (i <= j)
            {
                int temp = array[i];
                array[i] = array[j];
                array[j] = temp;
                i++;
                j--;
            }
        }
        
        if (leftIndex < j)
            quick_sort(array, leftIndex, j);
        if (i < rightIndex)
            quick_sort(array, i, rightIndex);
    }

    // Prints the content of an int[] array
    static public void print_array(int[] array, int size) {

        for (int i = 0; i < size; ++i) {
            Console.Write("{0} ", array[i]);
        }

        Console.WriteLine(" ");
    }

    static void Main(string[] args)
    {
        // Reading the size of array to be sorted
        var data_size = args.Length > 0 ? int.Parse(args[0]) : 100;

        int[] dataset = new int[data_size];

        string file_name = "dataset.txt";

        // Writing values from file to string - needs to be converted in int further
        string raw_data = File.ReadAllText(file_name);

        // Splitting string into substrings considiring the space symbol as splitter
        string[] split_raw_data = raw_data.Split(new char[] {' '});

        // Converting into int[] array
        for (int i = 0; i < data_size; ++i) {

            dataset[i] = Int32.Parse(split_raw_data[i]);

        }

        quick_sort(dataset, 0, data_size - 1);

        //print_array(dataset, data_size);

    }
}