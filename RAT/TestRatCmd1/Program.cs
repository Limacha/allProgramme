namespace TestRatCmd1
{
    internal class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello, World!");
            Console.WriteLine("c) client\ns) server");
            switch (Console.ReadKey().Key)
            {
                case (ConsoleKey.C):
                    RATClient.InitClient();
                    break;
                case (ConsoleKey.S):
                    RATServer.InitServer();
                    break;
            }
        }
    }
}
