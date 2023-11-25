internal static class Program
{
    private static void Main(string[] args)
    {
        if (args.Length < 1)
        {
            Console.Error.WriteLine("Error! File with prime number count must be specified\n");
            Environment.Exit(1);
        }

        var primeNumberCountPath = args[0];

        var primeNumberCountString = File.ReadAllLines(primeNumberCountPath);
        var primeNumberCount = ulong.Parse(primeNumberCountString[0]);

        var currNumber = 0ul;
        while (primeNumberCount > 0)
        {
            currNumber++;
            var j = 0;

            for (var i = 1ul; i <= currNumber; i++)
            {
                if (currNumber % i == 0)
                {
                    j++;
                }
            }

            if (j == 2)
            {
                primeNumberCount--;
            }
        }

        Console.Write($"The latest prime number: {currNumber}\n");
    }
}